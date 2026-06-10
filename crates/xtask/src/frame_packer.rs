use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use error::{ErrorKind, GameError, Result};
use image::{GenericImage, RgbaImage};
use serde::Serialize;

const SOURCE_ROOT: &str = "workbench/source_frames";
const OUTPUT_ROOT: &str = "assets/2d/animated";

#[derive(Debug, Clone, Copy)]
pub struct PackFrameOptions {
    pub columns: Option<u32>,
    pub fps: f32,
    pub repeat: bool,
}

impl Default for PackFrameOptions {
    fn default() -> Self {
        Self {
            columns: None,
            fps: 12.0,
            repeat: true,
        }
    }
}

#[derive(Debug)]
struct SourceFrame {
    path: PathBuf,
    clip_name: String,
    frame_number: u32,
}

#[derive(Serialize)]
struct FrameManifest {
    image: String,
    frame_size: (u32, u32),
    columns: u32,
    rows: u32,
    clips: BTreeMap<String, FrameClip>,
}

#[derive(Serialize)]
struct FrameClip {
    frames: Vec<usize>,
    fps: f32,
    repeat: bool,
}

pub fn pack_frame_target(target: &str, options: PackFrameOptions) -> Result<()> {
    validate_target(target)?;

    let source_dir = Path::new(SOURCE_ROOT).join(target);
    let Some(name) = Path::new(target)
        .file_name()
        .and_then(|name| name.to_str())
        .filter(|name| !name.is_empty())
    else {
        return Err(asset_error(
            "pack-frame-target",
            format!("target `{target}` must end with a resource name"),
        ));
    };

    let output_dir = Path::new(OUTPUT_ROOT).join(target);
    let output_image = output_dir.join(format!("{name}.png"));
    let output_manifest = output_dir.join(format!("{name}.frames.ron"));

    let frames = collect_source_frames(&source_dir)?;
    let packed = pack_frames(&frames, options)?;

    fs::create_dir_all(&output_dir)?;
    packed.image.save(&output_image).map_err(|error| {
        asset_error(
            "pack-frame-save-image",
            format!("failed to save {}: {error}", output_image.display()),
        )
    })?;

    let manifest_text = ron::ser::to_string_pretty(&packed.manifest, ron::ser::PrettyConfig::new())
        .map_err(|error| asset_error("pack-frame-manifest", error.to_string()))?;
    fs::write(&output_manifest, manifest_text)?;

    println!("packed {} frame(s)", frames.len());
    println!("image: {}", output_image.display());
    println!("manifest: {}", output_manifest.display());

    Ok(())
}

fn collect_source_frames(source_dir: &Path) -> Result<Vec<SourceFrame>> {
    let entries = fs::read_dir(source_dir).map_err(|error| {
        asset_error(
            "pack-frame-source",
            format!("failed to read {}: {error}", source_dir.display()),
        )
    })?;

    let mut frames = Vec::new();

    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if !path.is_file() || path.extension().is_none_or(|extension| extension != "png") {
            continue;
        }

        let Some(stem) = path.file_stem().and_then(|stem| stem.to_str()) else {
            continue;
        };
        let Some((clip_name, frame_number)) = parse_frame_name(stem) else {
            return Err(asset_error(
                "pack-frame-name",
                format!("{} must be named like `idle_down_000.png`", path.display()),
            ));
        };

        frames.push(SourceFrame {
            path,
            clip_name,
            frame_number,
        });
    }

    if frames.is_empty() {
        return Err(asset_error(
            "pack-frame-empty",
            format!("{} does not contain any PNG frames", source_dir.display()),
        ));
    }

    frames.sort_by(|left, right| {
        left.clip_name
            .cmp(&right.clip_name)
            .then(left.frame_number.cmp(&right.frame_number))
            .then(left.path.cmp(&right.path))
    });

    Ok(frames)
}

fn parse_frame_name(stem: &str) -> Option<(String, u32)> {
    let (clip_name, frame_number) = stem.rsplit_once('_')?;
    if clip_name.is_empty() || frame_number.is_empty() {
        return None;
    }
    if !frame_number
        .chars()
        .all(|character| character.is_ascii_digit())
    {
        return None;
    }

    Some((clip_name.to_string(), frame_number.parse().ok()?))
}

struct PackedFrames {
    image: RgbaImage,
    manifest: FrameManifest,
}

fn pack_frames(frames: &[SourceFrame], options: PackFrameOptions) -> Result<PackedFrames> {
    let first = image::open(&frames[0].path)
        .map_err(|error| {
            asset_error(
                "pack-frame-open",
                format!("failed to open {}: {error}", frames[0].path.display()),
            )
        })?
        .into_rgba8();
    let frame_width = first.width();
    let frame_height = first.height();

    let frame_count = frames.len() as u32;
    let columns = options
        .columns
        .unwrap_or_else(|| (frame_count as f32).sqrt().ceil() as u32)
        .max(1);
    let rows = frame_count.div_ceil(columns);

    let mut output = RgbaImage::new(frame_width * columns, frame_height * rows);
    let mut clips = BTreeMap::<String, FrameClip>::new();

    for (index, frame) in frames.iter().enumerate() {
        let image = if index == 0 {
            first.clone()
        } else {
            image::open(&frame.path)
                .map_err(|error| {
                    asset_error(
                        "pack-frame-open",
                        format!("failed to open {}: {error}", frame.path.display()),
                    )
                })?
                .into_rgba8()
        };

        if image.width() != frame_width || image.height() != frame_height {
            return Err(asset_error(
                "pack-frame-size",
                format!(
                    "{} is {}x{}, expected {}x{}",
                    frame.path.display(),
                    image.width(),
                    image.height(),
                    frame_width,
                    frame_height
                ),
            ));
        }

        let index_u32 = index as u32;
        let x = (index_u32 % columns) * frame_width;
        let y = (index_u32 / columns) * frame_height;
        output.copy_from(&image, x, y).map_err(|error| {
            asset_error(
                "pack-frame-copy",
                format!(
                    "failed to copy {} into atlas: {error}",
                    frame.path.display()
                ),
            )
        })?;

        clips
            .entry(frame.clip_name.clone())
            .or_insert_with(|| FrameClip {
                frames: Vec::new(),
                fps: options.fps,
                repeat: options.repeat,
            })
            .frames
            .push(index);
    }

    Ok(PackedFrames {
        image: output,
        manifest: FrameManifest {
            image: manifest_image_path(frames)?,
            frame_size: (frame_width, frame_height),
            columns,
            rows,
            clips,
        },
    })
}

fn manifest_image_path(frames: &[SourceFrame]) -> Result<String> {
    let source_dir = frames[0].path.parent().ok_or_else(|| {
        asset_error(
            "pack-frame-source",
            format!("{} has no parent directory", frames[0].path.display()),
        )
    })?;
    let target = source_dir.strip_prefix(SOURCE_ROOT).map_err(|error| {
        asset_error(
            "pack-frame-source",
            format!("failed to resolve {}: {error}", source_dir.display()),
        )
    })?;
    let name = target
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| {
            asset_error(
                "pack-frame-target",
                format!("{} must end with a resource name", target.display()),
            )
        })?;

    Ok(Path::new("2d")
        .join("animated")
        .join(target)
        .join(format!("{name}.png"))
        .to_string_lossy()
        .replace('\\', "/"))
}

fn validate_target(target: &str) -> Result<()> {
    let path = Path::new(target);
    if target.is_empty()
        || path.is_absolute()
        || target.contains("..")
        || path.components().count() != 2
    {
        return Err(asset_error(
            "pack-frame-target",
            "target must look like `{category}/{name}`",
        ));
    }

    Ok(())
}

fn asset_error(code: &'static str, message: impl Into<String>) -> GameError {
    GameError::from_kind(ErrorKind::Asset, code, message)
}

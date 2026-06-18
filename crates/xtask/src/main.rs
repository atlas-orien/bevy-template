mod asset_packer;
mod frame_packer;
mod rules;
mod tileset_packer;

use asset_packer::pack_all_assets;
use error::{ErrorKind, GameError, Result};
use frame_packer::{PackFrameOptions, pack_frame_target};
use rules::CheckStatus;
use std::env;
use std::process::ExitCode;
use tileset_packer::{PackTilesetOptions, pack_tileset_target};

fn main() -> ExitCode {
    let mut args = env::args().skip(1);
    let Some(command) = args.next() else {
        print_help();
        return ExitCode::SUCCESS;
    };

    let result = match command.as_str() {
        "check" | "check-architecture" => rules::check_architecture(),
        "pack-assets" | "pack-all-assets" => match pack_all_assets() {
            Ok(()) => CheckStatus::Passed,
            Err(error) => CheckStatus::Failed(vec![error.to_string()]),
        },
        "pack-frame" | "pack-frames" => match parse_pack_frame_args(args) {
            Ok((target, options)) => match pack_frame_target(&target, options) {
                Ok(()) => CheckStatus::Passed,
                Err(error) => CheckStatus::Failed(vec![error.to_string()]),
            },
            Err(error) => CheckStatus::Failed(vec![error.to_string()]),
        },
        "pack-tileset" | "pack-tilesets" => match parse_pack_tileset_args(args) {
            Ok((target, options)) => match pack_tileset_target(&target, options) {
                Ok(()) => CheckStatus::Passed,
                Err(error) => CheckStatus::Failed(vec![error.to_string()]),
            },
            Err(error) => CheckStatus::Failed(vec![error.to_string()]),
        },
        "help" | "-h" | "--help" => {
            print_help();
            return ExitCode::SUCCESS;
        }
        unknown => CheckStatus::Failed(vec![format!("unknown xtask command: {unknown}")]),
    };

    match result {
        CheckStatus::Passed => {
            println!("xtask command completed");
            ExitCode::SUCCESS
        }
        CheckStatus::Failed(errors) => {
            eprintln!("architecture checks failed:");
            for error in errors {
                eprintln!("- {error}");
            }
            ExitCode::from(1)
        }
    }
}

fn print_help() {
    println!("用法: cargo run -p xtask -- <命令>");
    println!();
    println!("命令:");
    println!("  check                         检查项目架构规则");
    println!("  pack-assets                   打包 workbench 下所有已配置资源");
    println!("  pack-frame <category/name>    打包 workbench/source_frames 下的帧动画散图");
    println!("  pack-tileset <name>           打包 workbench/source_tilesets 下的 tileset 图片");
    println!();
    println!("pack-frame 选项:");
    println!("  --columns <n>    指定输出 sprite sheet 列数");
    println!("  --fps <n>        指定所有 clip 的默认 fps，默认 12");
    println!("  --once           生成 repeat=false 的 clip 描述");
    println!();
    println!("pack-tileset 选项:");
    println!("  --rows <n>          指定 tileset array 行数");
    println!("  --tile-size <n>     指定方形 tile 尺寸");
    println!("  --tile-size <w>x<h> 指定 tile 宽高");
    println!("  --from-static       使用 assets/2d/static/tilemaps 已存在的图片，只生成 manifest");
}

fn parse_pack_frame_args(args: impl Iterator<Item = String>) -> Result<(String, PackFrameOptions)> {
    let mut target = None;
    let mut options = PackFrameOptions::default();
    let mut args = args.peekable();

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--columns" => {
                let Some(value) = args.next() else {
                    return Err(argument_error("--columns requires a number"));
                };
                options.columns =
                    Some(value.parse().map_err(|_| {
                        argument_error(format!("invalid --columns value: {value}"))
                    })?);
            }
            "--fps" => {
                let Some(value) = args.next() else {
                    return Err(argument_error("--fps requires a number"));
                };
                options.fps = value
                    .parse()
                    .map_err(|_| argument_error(format!("invalid --fps value: {value}")))?;
            }
            "--once" => {
                options.repeat = false;
            }
            value if value.starts_with('-') => {
                return Err(argument_error(format!(
                    "unknown pack-frame option: {value}"
                )));
            }
            value => {
                if target.replace(value.to_string()).is_some() {
                    return Err(argument_error(
                        "pack-frame accepts exactly one target like `{category}/{name}`",
                    ));
                }
            }
        }
    }

    let Some(target) = target else {
        return Err(argument_error(
            "pack-frame requires a target like `{category}/{name}`",
        ));
    };

    Ok((target, options))
}

fn argument_error(message: impl Into<String>) -> GameError {
    GameError::from_kind(ErrorKind::Config, "xtask-args", message)
}

fn parse_pack_tileset_args(
    args: impl Iterator<Item = String>,
) -> Result<(String, PackTilesetOptions)> {
    let mut target = None;
    let mut rows = None;
    let mut tile_size = None;
    let mut from_static = false;
    let mut args = args.peekable();

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--rows" => {
                let Some(value) = args.next() else {
                    return Err(argument_error("--rows requires a number"));
                };
                rows = Some(
                    value
                        .parse()
                        .map_err(|_| argument_error(format!("invalid --rows value: {value}")))?,
                );
            }
            "--tile-size" => {
                let Some(value) = args.next() else {
                    return Err(argument_error("--tile-size requires a value"));
                };
                tile_size = Some(parse_tile_size(&value)?);
            }
            "--from-static" => {
                from_static = true;
            }
            value if value.starts_with('-') => {
                return Err(argument_error(format!(
                    "unknown pack-tileset option: {value}"
                )));
            }
            value => {
                if target.replace(value.to_string()).is_some() {
                    return Err(argument_error(
                        "pack-tileset accepts exactly one target like `demo_tileset`",
                    ));
                }
            }
        }
    }

    let Some(target) = target else {
        return Err(argument_error(
            "pack-tileset requires a target like `demo_tileset`",
        ));
    };
    let Some(rows) = rows else {
        return Err(argument_error("pack-tileset requires --rows <n>"));
    };
    let Some(tile_size) = tile_size else {
        return Err(argument_error(
            "pack-tileset requires --tile-size <n|w>x<h>",
        ));
    };

    Ok((
        target,
        PackTilesetOptions {
            rows,
            tile_size,
            from_static,
        },
    ))
}

fn parse_tile_size(value: &str) -> Result<(u32, u32)> {
    if let Some((width, height)) = value.split_once('x') {
        let width = width
            .parse()
            .map_err(|_| argument_error(format!("invalid --tile-size value: {value}")))?;
        let height = height
            .parse()
            .map_err(|_| argument_error(format!("invalid --tile-size value: {value}")))?;
        return Ok((width, height));
    }

    let size = value
        .parse()
        .map_err(|_| argument_error(format!("invalid --tile-size value: {value}")))?;
    Ok((size, size))
}

mod frame_packer;
mod rules;

use error::{ErrorKind, GameError, Result};
use frame_packer::{PackFrameOptions, pack_frame_target};
use rules::CheckStatus;
use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    let mut args = env::args().skip(1);
    let Some(command) = args.next() else {
        print_help();
        return ExitCode::SUCCESS;
    };

    let result = match command.as_str() {
        "check" | "check-architecture" => rules::check_architecture(),
        "pack-frame" | "pack-frames" => match parse_pack_frame_args(args) {
            Ok((target, options)) => match pack_frame_target(&target, options) {
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
    println!("  pack-frame <category/name>    打包 workbench/source_frames 下的帧动画散图");
    println!();
    println!("pack-frame 选项:");
    println!("  --columns <n>    指定输出 sprite sheet 列数");
    println!("  --fps <n>        指定所有 clip 的默认 fps，默认 12");
    println!("  --once           生成 repeat=false 的 clip 描述");
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

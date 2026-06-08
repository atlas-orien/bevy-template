mod rules;

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
        "help" | "-h" | "--help" => {
            print_help();
            return ExitCode::SUCCESS;
        }
        unknown => Err(vec![format!("unknown xtask command: {unknown}")]),
    };

    match result {
        Ok(()) => {
            println!("architecture checks passed");
            ExitCode::SUCCESS
        }
        Err(errors) => {
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
    println!("  check    检查项目架构规则");
}

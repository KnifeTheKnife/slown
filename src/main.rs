use std::process::ExitCode;

mod help;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        help::print_help();
        return ExitCode::FAILURE;
    }
    if args[1] == "-h" || args[1] == "--help" {
        help::print_help();
        return ExitCode::SUCCESS;
    }
    ExitCode::SUCCESS
}

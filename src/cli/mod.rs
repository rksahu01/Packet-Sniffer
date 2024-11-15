use crate::utils::formatted_strings::APP_VERSION;
use crate::{Configs, SNIFFNET_LOWERCASE};

/// Parse CLI arguments, and exit if `--help`, `--version`, or an
/// unknown argument was supplied
pub fn parse_cli_args() {
    let mut args = std::env::args().skip(1);
    if let Some(arg) = args.next() {
        match arg.as_str() {
            "--help" | "-h" => print_help(),
            "--version" | "-v" => print_version(),
            "--restore-default" => restore_default(),
            _ => {
                unknown_argument(&arg);
                std::process::exit(1);
            }
        }
        std::process::exit(0);
    }
}

fn print_help() {
    println!(
        "Application to comfortably monitor your Internet traffic\n\
        Usage: {SNIFFNET_LOWERCASE} [OPTIONS]\n\
        Options:\n\
        \t-h, --help            Print help\n\
        \t--restore-default     Restore default settings\n\
        \t-v, --version         Print version info\n\
        (Run without options to start the app)"
    );
}

fn print_version() {
    println!("{SNIFFNET_LOWERCASE} {APP_VERSION}");
}

fn restore_default() {
    Configs::default().store();
    println!("Default settings have been restored");
}

fn unknown_argument(arg: &str) {
    eprintln!(
        "{SNIFFNET_LOWERCASE}: unknown option '{arg}'\n\
        For more information, try '{SNIFFNET_LOWERCASE} --help'"
    );
}

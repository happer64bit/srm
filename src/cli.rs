use std::env;

// Version information constant
const VERSION: &str = "1.0.0";

// Parse arguments and return regex pattern (if provided) and skip_errors flag
pub fn parse_args() -> (Option<String>, bool) {
    let args: Vec<String> = env::args().collect();

    // Handle `--help` and `--version` flags
    if args.contains(&String::from("--help")) {
        print_help(&args[0]);
        std::process::exit(0);
    }

    if args.contains(&String::from("--version")) {
        println!("{} version {}", args[0], VERSION);
        std::process::exit(0);
    }

    // Check for the skip flag
    let skip_errors = args.contains(&String::from("-s"));

    // If a regex pattern is provided, return it; otherwise, return None
    let pattern = if args.len() >= 2 && !args[1].starts_with('-') {
        Some(args[1].clone())
    } else {
        None
    };

    (pattern, skip_errors)
}

// Print the help message
fn print_help(program_name: &str) {
    println!("Usage: {} <regex_pattern> [OPTIONS]", program_name);
    println!("\nOptions:");
    println!("  --help       Show this help message and exit");
    println!("  --version    Show version information and exit");
    println!("  -s           Skip errors when deleting files");
}

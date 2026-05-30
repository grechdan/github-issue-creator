use std::env;
use std::process;

fn main() {
    let mut args = env::args();

    let program_name = args.next().unwrap_or_else(|| "issue-validator".to_string());

    let Some(input_file) = args.next() else {
        eprintln!("Usage: {program_name} <issues-file>");
        process::exit(1);
    };

    println!("Starting validation for: {input_file}");
    println!("Temporary success: validation logic is not implemented yet.");
}
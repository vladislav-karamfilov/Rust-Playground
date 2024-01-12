use regex::Regex;
use std::env;
use std::fs;
use text_colorizer::*;

fn main() {
    let args = parse_args();

    let file_contents = match fs::read_to_string(&args.input_filename) {
        Ok(v) => v,
        Err(e) => {
            eprintln!(
                "{} failed to read from file '{}': {:?}",
                "Error:".red().bold(),
                args.input_filename,
                e
            );

            std::process::exit(1);
        }
    };

    let replaced_file_contents = match replace_text(&args.target, &args.replacement, &file_contents)
    {
        Ok(result) => result,
        Err(e) => {
            eprintln!("{} failed to replace text: {:?}", "Error:".red().bold(), e);
            std::process::exit(1);
        }
    };

    match fs::write(&args.output_filename, replaced_file_contents) {
        Ok(_) => {}
        Err(e) => {
            eprintln!(
                "{} failed to write to file '{}': {:?}",
                "Error:".red().bold(),
                args.output_filename,
                e
            );

            std::process::exit(1);
        }
    };
}

fn replace_text(target: &str, replacement: &str, text: &str) -> Result<String, regex::Error> {
    let regex = Regex::new(target)?;
    Ok(regex.replace_all(text, replacement).to_string())
}

fn parse_args() -> Arguments {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 4 {
        print_usage();
        eprintln!(
            "{} wrong number of arguments: expected 4, got {}.",
            "Error:".red().bold(),
            args.len()
        );

        std::process::exit(1);
    }

    Arguments {
        target: args[0].clone(),
        replacement: args[1].clone(),
        input_filename: args[2].clone(),
        output_filename: args[3].clone(),
    }
}

fn print_usage() {
    eprintln!(
        "{} - change occurrences of one string into another",
        "quick_replace".green()
    );

    eprintln!("Usage: quick_replace <target> <replacement> <INPUT_FILE> <OUTPUT_FILE>");
}

#[derive(Debug)]
struct Arguments {
    target: String,
    replacement: String,
    input_filename: String,
    output_filename: String,
}

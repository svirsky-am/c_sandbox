use clap::{Arg, Command};

use rr_parser_lib::{Format, parse_input, serialize_output}; // Uses the library crate
use std::fs;
use std::path::Path;

// #[derive(Parser)]
// #[command(
//     name = "format-converter",
//     version,
//     about = "Convert between data formats",
//     long_about = "A simple format converter that reads input files and converts between supported formats"
// )]

// Plain struct — no macros, no derives from clap
pub struct Cli {
    pub input: String,
    pub output: String,
    pub in_format: Format,
    pub out_format: Format,
}

fn parse_cli() -> Result<Cli, Box<dyn std::error::Error>> {
    let matches = Command::new("format-converter")
        .version("0.1.0")
        .about("Convert between data formats")
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .help("Input file path")
                .required(true)
                .value_parser(clap::value_parser!(String)),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .help("Output file path (use '-' for stdout)")
                .default_value("-")
                .value_parser(clap::value_parser!(String)),
        )
        .arg(
            Arg::new("in-format")
                .long("in-format")
                .help("Input format [json, yaml, toml]")
                .default_value("json")
                .value_parser(parse_format),
        )
        .arg(
            Arg::new("out-format")
                .long("out-format")
                .help("Output format [json, yaml, toml]")
                .default_value("json")
                .value_parser(parse_format),
        )
        .get_matches();

    // Extract and build Cli struct manually
    let cli = Cli {
        input: matches.get_one::<String>("input").unwrap().clone(),
        output: matches.get_one::<String>("output").unwrap().clone(),
        in_format: matches.get_one::<Format>("in-format").unwrap().clone(),
        out_format: matches.get_one::<Format>("out-format").unwrap().clone(),
    };

    Ok(cli)
}

// Helper: Convert string to Format for clap
fn parse_format(s: &str) -> Result<Format, String> {
    s.parse()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = parse_cli()?;

    // Read input file
    let input_content = fs::read_to_string(&cli.input)?;

    // Parse input
    let data = parse_input(&input_content, &cli.in_format)?;

    // Serialize to output format
    let output_content = serialize_output(&data, &cli.out_format)?;

    // Handle output
    if cli.output == "-" {
        println!("{}", output_content);
    } else {
        let output_path = Path::new(&cli.output);

        // ✅ Create parent directory if it doesn't exist
        if let Some(parent) = output_path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)?;
            }
        }

        // Write the file
        fs::write(output_path, output_content)?;
    }

    Ok(())
}

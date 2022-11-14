use clap::{Parser, Subcommand};
use sha256::try_digest;
use std::{path::Path, process};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// does testing things
    Check {
        /// Path of the file to check
        #[arg(short, long, value_name = "FILE")]
        path: Option<String>,

        /// Sum to check - SHA256
        #[arg(short, long, value_name = "SUM")]
        sum: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Check { path, sum } => {
            let file_path = match path {
                Some(p) => p,
                None => {
                    eprintln!("❌ Please provide a valid file path ❌");
                    process::exit(1)
                }
            };
            let sum_to_compare = match sum {
                Some(s) => s.to_string(),
                None => {
                    eprintln!("❌ Please provide a sum ❌");
                    process::exit(1)
                }
            };

            let input = Path::new(file_path);

            let path_exist = input.is_file();

            if !path_exist {
                eprintln!("❌ File does not exist, please verify the path ❌");
                process::exit(1)
            } else {
                let val = try_digest(input);
                match val {
                    Ok(v) => {
                        if v == sum_to_compare {
                            println!("🚀 File is correct, checksums are the same 🚀")
                        } else {
                            println!("⚠️ Invalid checksum ⚠️")
                        }
                    }
                    Err(e) => {
                        eprintln!("❌ Error while getting the file checksum ❌, {}", e);
                        process::exit(1)
                    }
                }
            }
        }
    }
}

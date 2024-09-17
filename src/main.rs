use clap::{Parser, Subcommand};
use codecrafters_interpreter::*;
use miette::{IntoDiagnostic, Result, WrapErr};
use std::{fs, path::PathBuf};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Tokenize { filename: PathBuf },
}

fn main() -> Result<()> {
    let args = Args::parse();
    let mut any_cc_err = false;

    match args.command {
        Commands::Tokenize { filename } => {
            let file_contents = fs::read_to_string(&filename)
                .into_diagnostic()
                .wrap_err_with(|| format!("reading '{}' failed", filename.display()))?;

            if file_contents.chars().count() > 0 {
                for token in Lexer::new(&file_contents) {
                    let token = match token {
                        Ok(t) => t,
                        Err(e) => {
                            eprintln!("{e:?}");
                            if let Some(unrecognized) = e.downcast_ref::<SingleTokenError>() {
                                any_cc_err = true;
                                eprintln!(
                                    "[line {}] Error: Unexpected character: {}",
                                    unrecognized.line(),
                                    unrecognized.token
                                );
                                // std::process::exit(65);
                            }
                            // return Err(e);
                            continue;
                        }
                    };
                    println!("{token}");
                }
            }

            println!("EOF  null");

            if any_cc_err {
                std::process::exit(65);
            }
        }
    }

    Ok(())
}

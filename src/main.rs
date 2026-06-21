mod entity;
mod printer;
mod readable_size;

use crate::printer::Printer;
use clap::Parser;
use std::{fs, path::PathBuf};

#[derive(Parser)]
#[command(
    version,
    about = "Display directory structures in a tree-like format.",
    long_about = "A fast, lightweight command-line utility written in Rust for displaying directory structures as a tree. The tool recursively traverses directories, clearly differentiates files, symbolic links, and folders, supports configurable depth limits, handles filesystem errors gracefully, and presents output in a clean, readable format with subtle terminal styling. Designed with performance, reliability, and cross-platform compatibility in mind."
)]
struct Cli {
    #[arg(
        help = "Root directory to display. Defaults to the current directory if omitted.",
        default_value = "./"
    )]
    path: PathBuf,

    #[arg(
        long = "max-depth",
        short = 'l',
        value_name = "DEPTH",
        help = "Limit the tree to DEPTH levels. Shows only first level if omitted.",
        default_value_t = 1
    )]
    max_depth: usize,
}

fn main() {
    let cli = Cli::parse();

    match fs::exists(&cli.path) {
        Ok(does_exist) => {
            if does_exist {
                let mut printer = Printer::new(cli.max_depth);

                match printer.check_entity(&cli.path) {
                    Ok(_) => (),
                    Err(e) => eprintln!("{}", e),
                }
            } else {
                eprintln!("Path does not exist");
            }
        }
        Err(e) => eprintln!("{}", e),
    }
}

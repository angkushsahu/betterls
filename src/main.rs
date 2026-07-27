mod entity;
mod log;
mod printer;
mod readable_size;

use crate::{log::error_log, printer::Printer};
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

    #[arg(
        short = 'a',
        long = "all",
        help = "Show all files and directories, including hidden ones",
        default_value_t = false
    )]
    show_hidden: bool,
}

fn main() {
    let cli = Cli::parse();

    match fs::exists(&cli.path) {
        Ok(does_exist) => {
            if does_exist {
                // Initializing the printer struct with user provided or default values
                let mut printer = Printer::new(cli.max_depth, cli.show_hidden);

                match printer.check_entity(&cli.path) {
                    Ok(_) => (),
                    Err(e) => error_log(&e.to_string()),
                }
            } else {
                error_log("Path does not exist");
            }
        }
        Err(e) => error_log(&e.to_string()),
    }
}

mod cli;
mod entity;
mod log;
mod printer;
mod readable_size;

use crate::{cli::Cli, log::error_log, printer::Printer};
use clap::Parser;
use std::{fs, path::PathBuf};

fn main() {
    let cli = Cli::parse();

    let paths = if cli.paths.is_empty() {
        vec![PathBuf::from("./")]
    } else {
        cli.paths
    };

    for path in paths {
        match fs::exists(&path) {
            Ok(does_exist) => {
                if does_exist {
                    // Trim whitespace in case the user added spaces around commas
                    let ignore: Vec<String> = cli
                        .ignore
                        .iter()
                        .map(|name| name.trim().to_string())
                        .collect();

                    // Initializing the printer struct with user provided or default values
                    let mut printer = Printer::new(cli.max_depth, cli.show_hidden, ignore);

                    match printer.check_entity(&path) {
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
}

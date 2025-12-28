use crate::{
    fs_tree::get_files,
    output::{print_error, print_table},
};
use clap::Parser;
use std::{fs, path::PathBuf};

mod cli;
mod fs_tree;
mod output;
mod size;
mod types;

#[derive(Parser)]
#[command(version, about, long_about = "TODO")]
struct Cli {
    path: Option<PathBuf>,
    #[arg(short, long)]
    json: bool,
}

fn main() {
    let cli = Cli::parse();
    let path = cli.path.unwrap_or(PathBuf::from(".")); // check path, if not present, set it to current directory

    if let Ok(does_exist) = fs::exists(&path) {
        if does_exist {
            // check if the --json argument is passed
            if cli.json {
                let files = get_files(&path);
                let files_json =
                    serde_json::to_string(&files).unwrap_or(String::from("Unable to parse JSON"));
                println!("{}", files_json);
            } else {
                print_table(&path);
            }
        } else {
            // In case path does not exist
            print_error("Path does not exist");
        }
    } else {
        // In case fs::exists throws error
        print_error("Unable to read or find directory");
    }
}

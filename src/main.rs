use crate::{
    fs_tree::get_files::get_files,
    output::{print_error, print_table},
};
use clap::Parser;
use std::{fs, path::PathBuf};

mod fs_tree;
mod output;
mod types;

#[derive(Parser)]
#[command(
    version,
    about,
    long_about = "A minimal ls alternative that shows accurate recursive directory sizes in a tree like structure."
)]
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
            let files = get_files(&path);

            // check if the --json argument is passed
            if cli.json {
                let files_json =
                    serde_json::to_string(&files).unwrap_or(String::from("Unable to parse JSON"));
                println!("{}", files_json);
            } else {
                // print_table(&path);
                print_table(files);
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

use chrono::{DateTime, Utc};
use clap::Parser;
use owo_colors::OwoColorize;
use serde::Serialize;
use std::{
    fs,
    path::{Path, PathBuf},
};
use strum_macros::Display;
use tabled::{
    Table, Tabled,
    settings::{
        Color, Style,
        object::{Columns, Rows},
    },
};

#[derive(Display, Serialize)]
enum EntryType {
    File,
    Directory,
}

#[derive(Tabled, Serialize)]
struct FileEntry {
    #[tabled{rename = "Name"}]
    name: String,
    #[tabled{rename = "Type"}]
    entry_type: EntryType,
    #[tabled{rename = "Size"}]
    size: String,
    #[tabled{rename = "Last Modified"}]
    modified: String,
}

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

fn get_files(path: &Path) -> Vec<FileEntry> {
    let mut data = Vec::new();

    if let Ok(read_dir) = fs::read_dir(path) {
        for file in read_dir.flatten() {
            map_data(file, &mut data);
        }
    }

    data
}

fn map_data(file: fs::DirEntry, data: &mut Vec<FileEntry>) {
    if let Ok(metadata) = fs::metadata(file.path()) {
        let name = file
            .file_name()
            .into_string()
            .unwrap_or(String::from("Unknown name"));

        let entry_type = if metadata.is_dir() {
            EntryType::Directory
        } else {
            EntryType::File
        };

        let size = readable_size(metadata.len() as u128);

        let modified = if let Ok(modified) = metadata.modified() {
            let date: DateTime<Utc> = modified.into();
            format!("{}", date.format("%a %b %e %Y, %H:%M"))
        } else {
            String::new()
        };

        let file_entry = FileEntry {
            name,
            entry_type,
            size,
            modified,
        };

        data.push(file_entry);
    }
}

fn print_table(path: &Path) {
    let files = get_files(path);

    let mut table = Table::new(files);
    table.with(Style::rounded());

    table.modify(Columns::first(), Color::FG_BRIGHT_CYAN);
    table.modify(Columns::one(2), Color::FG_BRIGHT_MAGENTA);
    table.modify(Columns::one(3), Color::FG_BRIGHT_YELLOW);
    table.modify(Rows::first(), Color::FG_BRIGHT_GREEN);

    println!("{}", table);
}

fn print_error(message: &str) {
    println!("{}", message.bright_red());
}

fn readable_size(bytes: u128) -> String {
    const UNITS: [&str; 7] = ["B", "KB", "MB", "GB", "TB", "PB", "EB"];

    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        let next = size / 1024.0;
        if next < 1.0 {
            break;
        }

        size = next;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{} {}", bytes, UNITS[0])
    } else {
        format!("{:.2} {}", size, UNITS[unit_index])
    }
}

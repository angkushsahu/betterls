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
    size: u128,
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
    let path = cli.path.unwrap_or(PathBuf::from("."));

    if let Ok(does_exist) = fs::exists(&path) {
        if does_exist {
            if cli.json {
                let files = get_files(&path);
                let files_json =
                    serde_json::to_string(&files).unwrap_or(String::from("Unable to parse JSON"));
                println!("{}", files_json);
            } else {
                print_table(&path);
            }
        } else {
            println!("{}", "Path does not exist".bright_red());
        }
    } else {
        println!("{}", "Unable to read or find directory".bright_red());
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
    let name = file
        .file_name()
        .into_string()
        .unwrap_or(String::from("Unknown name"));

    if let Ok(metadata) = fs::metadata(file.path()) {
        let file_entry = FileEntry {
            name,
            entry_type: if metadata.is_dir() {
                EntryType::Directory
            } else {
                EntryType::File
            },
            size: metadata.len() as u128,
            modified: if let Ok(modified) = metadata.modified() {
                let date: DateTime<Utc> = modified.into();
                format!("{}", date.format("%a %b %e %Y, %H:%M"))
            } else {
                String::new()
            },
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

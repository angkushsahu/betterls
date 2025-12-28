use crate::{
    size::readable_size,
    types::{EntryType, FileEntry},
};
use chrono::{DateTime, Utc};
use std::{fs, path::Path, time::SystemTime};

pub(crate) fn format_system_time(time: SystemTime) -> String {
    let date_time: DateTime<Utc> = time.into();
    format!("{}", date_time.format("%a %b %e %Y, %H:%M"))
}

pub(crate) fn get_files(path: &Path) -> Vec<FileEntry> {
    let mut data = Vec::new();

    if let Ok(read_dir) = fs::read_dir(path) {
        for file in read_dir.flatten() {
            map_data(file, &mut data);
        }
    }

    data
}

pub(crate) fn map_data(file: fs::DirEntry, data: &mut Vec<FileEntry>) {
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
            format_system_time(modified)
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

use crate::{
    fs_tree::{directory_size::directory_size, size::readable_size},
    types::{EntryType, FileEntry},
};
use chrono::{DateTime, Utc};
use std::fs;

pub(super) fn map_data(file: fs::DirEntry, data: &mut Vec<FileEntry>) {
    let file_path = file.path();

    if let Ok(metadata) = fs::symlink_metadata(&file_path) {
        let name = file
            .file_name()
            .into_string()
            .unwrap_or(String::from("Unknown name"));

        let (entry_type, size) = if metadata.file_type().is_symlink() {
            let size = readable_size(metadata.len() as u128);
            (EntryType::SymLink, size)
        } else if metadata.is_dir() {
            let size = directory_size(&file_path);
            (EntryType::Directory, readable_size(size))
        } else {
            let size = readable_size(metadata.len() as u128);
            (EntryType::File, size)
        };

        let modified = if let Ok(modified) = metadata.modified() {
            let date_time: DateTime<Utc> = modified.into();
            format!("{}", date_time.format("%a %b %e %Y, %H:%M"))
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

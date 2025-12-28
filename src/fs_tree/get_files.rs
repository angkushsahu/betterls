use crate::{fs_tree::map_data::map_data, types::FileEntry};
use std::{fs, path::Path};

pub(crate) fn get_files(path: &Path) -> Vec<FileEntry> {
    let mut data = Vec::new();

    if let Ok(read_dir) = fs::read_dir(path) {
        for file in read_dir.flatten() {
            map_data(file, &mut data);
        }
    }

    data
}

use std::{fs, path::Path};

pub(super) fn directory_size(path: &Path) -> u128 {
    let mut total_size = 0;

    if let Ok(read_dir) = fs::read_dir(path) {
        for entry in read_dir.flatten() {
            let entry_path = entry.path();

            if let Ok(metadata) = fs::symlink_metadata(&entry_path) {
                let size = if metadata.file_type().is_symlink() || metadata.is_file() {
                    metadata.len() as u128
                } else if metadata.is_dir() {
                    directory_size(&entry_path)
                } else {
                    0
                };

                total_size += size;
            }
        }
    }

    total_size
}

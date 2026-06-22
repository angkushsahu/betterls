use crate::{entity::Entity, readable_size::read_size};
use std::{
    fs::{self, DirEntry, Metadata},
    io::Result,
    path::PathBuf,
};

pub(crate) struct Printer {
    stack: Vec<bool>,
    max_depth: usize,
    secondary_color: String,
    show_hidden: bool,
}

impl Printer {
    pub(crate) fn new(max_depth: usize, show_hidden: bool) -> Self {
        Self {
            stack: Vec::new(),
            max_depth,
            secondary_color: String::from("\x1b[90m"),
            show_hidden,
        }
    }

    fn get_file_name(path: &PathBuf) -> &str {
        if path == "./" || path == "." {
            return "./";
        }

        path.file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("<Invalid UTF-8>")
    }

    fn print_prefix(&self, is_last: bool) {
        for &has_siblings in self.stack.iter() {
            if has_siblings {
                print!("{}│   \x1b[0m", self.secondary_color);
            } else {
                print!("{}    \x1b[0m", self.secondary_color);
            }
        }

        if is_last {
            print!("{}└── \x1b[0m", self.secondary_color);
        } else {
            print!("{}├── \x1b[0m", self.secondary_color);
        }
    }

    pub(crate) fn check_entity(&mut self, path: &PathBuf) -> Result<()> {
        let metadata = fs::symlink_metadata(path)?;

        if metadata.is_symlink() {
            self.file_and_symlink(path, metadata, Entity::SymLink);
        } else if metadata.is_dir() {
            self.directory(path)?;
        } else if metadata.is_file() {
            self.file_and_symlink(path, metadata, Entity::File);
        }

        Ok(())
    }

    fn file_and_symlink(&self, path: &PathBuf, metadata: Metadata, entity_type: Entity) {
        let size = read_size(metadata.len() as usize);
        let file_name = Self::get_file_name(path);

        println!(
            "{} {}({}, {})\x1b[0m",
            file_name, self.secondary_color, entity_type, size
        );
    }

    fn directory(&mut self, path: &PathBuf) -> Result<()> {
        let file_name = Self::get_file_name(path);
        println!(
            "{} {}({})\x1b[0m",
            file_name,
            self.secondary_color,
            Entity::Directory
        );

        if self.stack.len() >= self.max_depth {
            return Ok(());
        }

        let mut files = Vec::new();
        let mut symlinks = Vec::new();
        let mut directories = Vec::new();

        for entry in fs::read_dir(path)?.flatten() {
            // check if files are hidden or not, if they are, and the flag for hidden files are
            // unset, then don't process them

            if !self.show_hidden && self.is_hidden(&entry) {
                continue;
            }

            let file_type = entry.file_type()?;

            if file_type.is_file() {
                files.push(entry);
            } else if file_type.is_symlink() {
                symlinks.push(entry);
            } else if file_type.is_dir() {
                directories.push(entry);
            }
        }

        files.sort_by_key(|e| e.file_name());
        symlinks.sort_by_key(|e| e.file_name());
        directories.sort_by_key(|e| e.file_name());

        let total_entries = files.len() + symlinks.len() + directories.len();
        let mut entry_count = 0;

        for entry in files.into_iter() {
            let entry_path = entry.path();
            let metadata = fs::symlink_metadata(&entry_path)?;

            entry_count += 1;
            let is_last = entry_count == total_entries;
            self.print_prefix(is_last);

            self.stack.push(!is_last);
            self.file_and_symlink(&entry_path, metadata, Entity::File);
            self.stack.pop();
        }
        for entry in symlinks.into_iter() {
            let entry_path = entry.path();
            let metadata = fs::symlink_metadata(&entry_path)?;

            entry_count += 1;
            let is_last = entry_count == total_entries;
            self.print_prefix(is_last);

            self.stack.push(!is_last);
            self.file_and_symlink(&entry_path, metadata, Entity::SymLink);
            self.stack.pop();
        }
        for entry in directories.into_iter() {
            let entry_path = entry.path();

            entry_count += 1;
            let is_last = entry_count == total_entries;
            self.print_prefix(is_last);

            self.stack.push(!is_last);
            let result = self.directory(&entry_path);
            self.stack.pop();

            if let Err(error) = result {
                eprintln!("Failed to access {}: {}", entry_path.display(), error);
            }
        }

        Ok(())
    }

    fn is_hidden(&self, entry: &DirEntry) -> bool {
        #[cfg(target_os = "windows")]
        {
            use std::os::windows::fs::MetadataExt;
            const FILE_ATTRIBUTE_HIDDEN: u32 = 0x2;

            entry
                .metadata
                .map(|m| m.file_attributes() & FILE_ATTRIBUTE_HIDDEN != 0)
                .unwrap_or(false)
        }

        #[cfg(not(target_os = "windows"))]
        {
            entry.file_name().to_string_lossy().starts_with('.')
        }
    }

    // NOTE: dO NOT REMOVE this code block, this is much more performant as it does not follow the
    // file first rule and therefore, no sorting is done here
    // fn directory(&mut self, path: &PathBuf) -> Result<()> {
    //     let file_name = Self::get_file_name(path);
    //     println!("{} {}({})\x1b[0m", file_name, self.secondary_color, Entity::Directory);

    //     if self.stack.len() >= self.max_depth {
    //         return Ok(());
    //     }

    //     let mut contents = fs::read_dir(path)?.flatten().peekable();

    //     while let Some(content) = contents.next() {
    //         let is_last = contents.peek().is_none();
    //         self.print_prefix(is_last);

    //         self.stack.push(!is_last);
    //         self.check_entity(&content.path())?;
    //         self.stack.pop();
    //     }

    //     Ok(())
    // }
}

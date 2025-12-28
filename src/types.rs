use serde::Serialize;
use tabled::Tabled;

#[derive(Serialize)]
pub(crate) enum EntryType {
    File,
    Directory,
    SymLink,
}

impl std::fmt::Display for EntryType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let format_type = match self {
            Self::File => "File",
            Self::Directory => "Directory",
            Self::SymLink => "SymLink",
        };

        write!(f, "{}", format_type)
    }
}

#[derive(Tabled, Serialize)]
pub(crate) struct FileEntry {
    #[tabled{rename = "Name"}]
    pub name: String,
    #[tabled{rename = "Type"}]
    pub entry_type: EntryType,
    #[tabled{rename = "Size"}]
    pub size: String,
    #[tabled{rename = "Last Modified"}]
    pub modified: String,
}

use serde::Serialize;
use strum_macros::Display;
use tabled::Tabled;

#[derive(Display, Serialize)]
pub(crate) enum EntryType {
    File,
    Directory,
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

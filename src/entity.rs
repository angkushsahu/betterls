pub(crate) enum Entity {
    File,
    SymLink,
    Directory,
}

impl std::fmt::Display for Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::File => write!(f, "F"),
            Self::SymLink => write!(f, "S"),
            Self::Directory => write!(f, "D"),
        }
    }
}

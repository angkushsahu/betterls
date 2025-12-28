use crate::types::FileEntry;
use owo_colors::OwoColorize;
use tabled::{
    Table,
    settings::{
        Color, Style,
        object::{Columns, Rows},
    },
};

pub(crate) fn print_table(files: Vec<FileEntry>) {
    let mut table = Table::new(files);
    table.with(Style::rounded());

    table.modify(Columns::first(), Color::FG_BRIGHT_CYAN);
    table.modify(Columns::one(2), Color::FG_BRIGHT_MAGENTA);
    table.modify(Columns::one(3), Color::FG_BRIGHT_YELLOW);
    table.modify(Rows::first(), Color::FG_BRIGHT_GREEN);

    println!("{}", table);
}

pub(crate) fn print_error(message: &str) {
    println!("{}", message.bright_red());
}

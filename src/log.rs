use crate::entity::Entity;
use owo_colors::{
    OwoColorize,
    Stream::{Stderr, Stdout},
};

#[inline]
pub(crate) fn error_log(message: &str) {
    // let error_color = "\x1b[31m";
    // eprintln!("{}{}\x1b[0m", error_color, message);
    eprintln!(
        "{}",
        message.if_supports_color(Stderr, |msg| msg.bright_red())
    );
}

#[inline]
pub(crate) fn muted_log(message: &str) {
    print!(
        "{}",
        message.if_supports_color(Stdout, |msg| msg.bright_black())
    );
}

#[inline]
pub(crate) fn print_dir_name(file_name: &str) {
    let details = format!("({})", Entity::Directory);
    let details = details.if_supports_color(Stdout, |msg| msg.bright_black());

    println!("{} {}", file_name, details);
}

#[inline]
pub(crate) fn print_file_or_symlink_name(file_name: &str, entity_type: Entity, size: &str) {
    let details = format!("({}, {})", entity_type, size);
    let details = details.if_supports_color(Stdout, |msg| msg.bright_black());

    println!("{} {}", file_name, details);
}

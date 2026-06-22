pub(crate) fn error_log(message: &str) {
    let error_color = "\x1b[31m";
    eprintln!("{}{}\x1b[0m", error_color, message);
}

use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    version,
    about = "Display directory structures in a tree-like format.",
    long_about = "A fast, lightweight command-line utility written in Rust for displaying directory structures as a tree. The tool recursively traverses directories, clearly differentiates files, symbolic links, and folders, supports configurable depth limits, handles filesystem errors gracefully, and presents output in a clean, readable format with subtle terminal styling. Designed with performance, reliability, and cross-platform compatibility in mind."
)]
pub struct Cli {
    #[arg(
        help = "Root directory to display. Defaults to the current directory if omitted.",
        default_value = "./"
    )]
    pub path: PathBuf,

    #[arg(
        long = "max-depth",
        short = 'l',
        value_name = "DEPTH",
        help = "Limit the tree to DEPTH levels. Shows only first level if omitted.",
        default_value_t = 1
    )]
    pub max_depth: usize,

    #[arg(
        short = 'a',
        long = "all",
        help = "Show all files and directories, including hidden ones",
        default_value_t = false
    )]
    pub show_hidden: bool,

    #[arg(
        short = 'i',
        long = "ignore",
        value_name = "NAMES",
        help = "Comma-separated list of file or directory names to ignore (e.g., target,node_modules,build)",
        value_delimiter = ','
    )]
    pub ignore: Vec<String>,
}

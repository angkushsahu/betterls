# Better LS

### Project Description

**bls** is a minimalist, Rust-based tree viewer that generates a visual representation of directory hierarchies. Features include recursive traversal, configurable depth limits, file type awareness (files, directories, and symbolic links), graceful error handling for inaccessible paths, deterministic sorting, and lightweight terminal styling. Built with a focus on performance and minimal dependencies.

### Key features

- Human-readable file sizes (KB, MB, GB, …)
- Clean, hierarchical output with proper indentation
- Tree view with configurable depth
- Files listed before directories for improved visibility
- Symlinks are never followed
- Option to show all files and directories, including hidden ones
- Option to ignore specific files or directories by name
- Minimal, opinionated feature set focused on core filesystem inspection

### Design philosophy

`bls` intentionally avoids feature creep. It does not include permissions, ownership,
icons, themes, git integration, or configuration files. The goal is to provide a clear,
trustworthy view of filesystem structure without unnecessary noise.

The project is also intended as a learning-focused systems exercise, demonstrating
idiomatic Rust for filesystem traversal, error handling, and CLI tooling.

## ⚙️ CLI Options

| Argument      | Short | Description                                                                                     | Default |
| ------------- | ----- | ----------------------------------------------------------------------------------------------- | ------- |
| `path`        |   -  | Root directory to display.                                                                       |   `./`  |
| `--max-depth` | `-l` | Limit tree traversal to the specified depth.                                                     |   `1`   |
| `--all`       | `-a` | Show all files and directories, including hidden ones.                                           |    -    |
| --ignore      | `-i` | Comma separated list of file and directory names to ignore (example, node_modules,target,build). |    -    |

### Examples

```bash
# Current directory (default depth = 1)
bls

# Scan a specific directory
bls ./src

# Show up to 3 levels
bls -l 3

# Show up to 5 levels for a specific path
bls ~/Documents -l 5

# Show all files in the current directory (including the hidden files)
bls -a

# Show "javascript-project" directory contents upto 2 level except the node-modules directory
bls javascript-project -l 2 -i node_modules
```

```bash
$ bls

./ (D)
├── .gitignore (F, 8 B)
├── Cargo.lock (F, 4.79 KB)
├── Cargo.toml (F, 293 B)
├── README.md (F, 2.76 KB)
├── output.txt (F, 1.08 KB)
├── .git (D)
├── src (D)
└── target (D)


$ bls ./src

src (D)
├── entity.rs (F, 348 B)
├── main.rs (F, 1.56 KB)
├── printer.rs (F, 4.83 KB)
└── readable_size.rs (F, 526 B)

$ bls ./target -l 2

target (D)
├── .rustc_info.json (F, 1.69 KB)
├── CACHEDIR.TAG (F, 177 B)
├── debug (D)
│   ├── .cargo-lock (F, 0 B)
│   ├── betterls (F, 4.14 MB)
│   ├── betterls.d (F, 99 B)
│   ├── bls (F, 13.32 MB)
│   ├── bls.d (F, 240 B)
│   ├── .fingerprint (D)
│   ├── build (D)
│   ├── deps (D)
│   ├── examples (D)
│   └── incremental (D)
├── flycheck0 (D)
│   ├── stderr (F, 72 B)
│   └── stdout (F, 15.76 KB)
└── release (D)
    ├── .cargo-lock (F, 0 B)
    ├── bls (F, 727.12 KB)
    ├── bls.d (F, 242 B)
    ├── .fingerprint (D)
    ├── build (D)
    ├── deps (D)
    ├── examples (D)
    └── incremental (D)

$ bls ./target -l 2 -i flycheck0,release
# The above command is equivalent to
$ ./target -l 2 -i "flycheck0, release"

target (D)
├── CACHEDIR.TAG (F, 177 B)
└── debug (D)
    ├── editoria (F, 8.81 MB)
    ├── editoria.d (F, 1.86 KB)
    ├── build (D)
    ├── deps (D)
    ├── examples (D)
    └── incremental (D)

$ bls -a

./ (D)
├── .hidden.txt (F, 0 B)
└── not-hidden.txt (F, 0 B)
```

## 👨🏻‍💻 Tech Stack

![My Skills](https://skillicons.dev/icons?i=rust&theme=dark)

## 🚀 Installation

> **Note:** Make sure that rust is installed in your system, you can install rust from their [official website](https://rust-lang.org/tools/install/).

### Build from source

```bash
git clone https://github.com/angkushsahu/betterls.git
cd betterls
cargo build --release
```

The compiled binary will be available at:

```text
target/release/bls
```

### Install system-wide (Linux)

```bash
sudo cp target/release/bls /usr/local/bin/
sudo chmod +x /usr/local/bin/bls
```

Verify the installation:

```bash
bls --help
```

## 🔗 Links

Contact Me from [here](https://angkushsahu.vercel.app/contact)

[![portfolio](https://img.shields.io/badge/my_portfolio-teal?style=for-the-badge&logo=ko-fi&logoColor=white)](https://angkushsahu.vercel.app/)
[![linkedin](https://img.shields.io/badge/linkedin-0A66C2?style=for-the-badge&logo=linkedin&logoColor=white)](https://linkedin.com/in/angkush-sahu-0409311bb)
[![mail](https://img.shields.io/badge/Mail-red?style=for-the-badge&logo=gmail&logoColor=white)](https://angkushsahu.vercel.app/contact)
[![github](https://img.shields.io/badge/Github-gray?style=for-the-badge&logo=github&logoColor=white)](https://github.com/angkushsahu)

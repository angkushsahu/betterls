# Better LS

### Project Description

**bls** is a minimalist, Rust-based alternative to the Unix `ls` command designed to fix
one of its most common shortcomings: misleading directory sizes.

Traditional `ls` reports directory sizes based only on filesystem metadata, which does
not reflect the total size of the files contained within a directory. `bls` instead
computes **accurate recursive directory sizes**, summing all nested files and
subdirectories while remaining fast and predictable.

### Key features

- Accurate recursive directory size calculation
- Human-readable file sizes (KB, MB, GB, …)
- Clean, hierarchical output with proper indentation
- Optional tree view with configurable depth
- Files listed before directories for improved visibility
- Explicit, safe symlink handling (symlinks are never followed)
- Minimal, opinionated feature set focused on core filesystem inspection

### Design philosophy

`bls` intentionally avoids feature creep. It does not include permissions, ownership,
icons, themes, git integration, or configuration files. The goal is to provide a clear,
trustworthy view of filesystem structure and disk usage without unnecessary noise.

The project is also intended as a learning-focused systems exercise, demonstrating
idiomatic Rust for filesystem traversal, error handling, and CLI tooling.

## 👨🏻‍💻 Tech Stack

![My Skills](https://skillicons.dev/icons?i=rust&theme=dark)

## 🔗 Links

Contact Me from [here](https://angkushsahu.vercel.app/contact)

[![portfolio](https://img.shields.io/badge/my_portfolio-teal?style=for-the-badge&logo=ko-fi&logoColor=white)](https://angkushsahu.vercel.app/)
[![linkedin](https://img.shields.io/badge/linkedin-0A66C2?style=for-the-badge&logo=linkedin&logoColor=white)](https://linkedin.com/in/angkush-sahu-0409311bb)
[![mail](https://img.shields.io/badge/Mail-red?style=for-the-badge&logo=gmail&logoColor=white)](https://angkushsahu.vercel.app/contact)
[![github](https://img.shields.io/badge/Github-gray?style=for-the-badge&logo=github&logoColor=white)](https://github.com/angkushsahu)

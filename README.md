Gitno: Gitignore Template Generator
Overview
Gitno is a terminal user interface (TUI) application that helps you generate .gitignore files by combining multiple templates from GitHub's gitignore collection.
Features

Fetch gitignore templates from GitHub
Interactive selection of multiple templates
Combine and deduplicate gitignore rules
Generate a .gitignore file in the current directory

Installation
bashCopygit clone https://github.com/andwati/gitno.git
cd gitno
cargo build --release
Usage
bashCopy# Run the application
cargo run

# Navigate:
# - Use Space to select/deselect templates
# - Press Enter to generate .gitignore
# - Press Q to quit
Requirements

Rust 1.75 or higher
Internet connection to fetch templates

Contributing
Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details.

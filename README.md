# treee - A Fast Tree Command with Gitignore Support

A high-performance tree command-line tool written in Rust, featuring gitignore support and flexible filtering capabilities.

## Features

- 🚀 **High Performance** - Written in Rust for fast traversal of large directories
- 🎯 **Smart Filtering** - Support for include/exclude patterns and filename pattern matching
- 🔍 **Gitignore Support** - Respects .gitignore rules by default, with option to disable
- 🎨 **Colored Output** - Automatic terminal detection with different colors for directories and files
- 📁 **Flexible Display** - Support for both tree format and full path format
- ⚙️ **Rich Options** - Depth control, hidden files, directory/file-only display, and more

## Installation

### From crates.io
```bash
cargo install treee
```

### From source
```bash
git clone https://github.com/yourusername/treee.git
cd treee
cargo install --path .
```

## Usage

### Basic Usage

```bash
# Display tree structure of current directory
treee

# Display specific directory
treee /path/to/directory

# Limit display depth
treee -L 3

# Show hidden files
treee -a
```

### Filtering Features

```bash
# Show only .rs files
treee --include "*.rs"

# Exclude target directory and .lock files
treee --exclude "target" --exclude "*.lock"

# Use filename pattern matching
treee --pattern "*.toml" --pattern "*.md"

# Show directories only
treee --directories-only

# Show files only
treee --files-only
```

### Git Integration

```bash
# Disable gitignore rules
treee --no-git-ignore

# By default, .gitignore rules are automatically applied
treee  # Automatically excludes files in gitignore
```

### Output Formats

```bash
# Display full paths instead of tree format
treee --full-path

# Disable colored output
treee --no-color
```

## Command Line Options

```text
Usage: treee [OPTIONS] [PATH]

Arguments:
  [PATH]  Directory to traverse [default: .]

Options:
  -L, --depth <DEPTH>               Maximum depth to traverse [default: 10]
  -a, --all                         Show hidden files
      --no-color                    Don't use colors
  -d, --directories-only            Show directories only
  -I, --include <INCLUDE_PATTERNS>  Include paths matching these glob patterns (can be used multiple times)
  -E, --exclude <EXCLUDE_PATTERNS>  Exclude paths matching these glob patterns (can be used multiple times)
  -P, --pattern <FILE_PATTERNS>     File name patterns to match (glob patterns, can be used multiple times)
      --no-git-ignore               Disable gitignore rules
  -f, --files-only                  Show only files (opposite of --directories-only)
      --full-path                   Print full paths instead of tree format
  -h, --help                        Print help
  -V, --version                     Print version
```

## Examples

### Find all Rust source files

```bash
treee --include "*.rs" --full-path
```

### Show project structure (excluding build artifacts)

```bash
treee --exclude "target" --exclude "node_modules" --exclude "*.lock"
```

### Show only configuration files

```bash
treee --pattern "*.toml" --pattern "*.json" --pattern "*.yaml"
```

### View deep directory structure

```bash
treee -L 5 --directories-only
```

### Complex filtering example

```bash
treee --exclude "target" --include "*.rs" --include "*.toml" --depth 3
```

## Performance

treee uses efficient file system traversal algorithms and parallel processing, delivering excellent performance on large codebases. Compared to traditional tree commands, it provides richer filtering options and better Git integration.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

MIT License

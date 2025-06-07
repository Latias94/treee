use anyhow::Result;
use clap::Parser;
use colored::*;
use ignore::WalkBuilder;
use std::path::{Path, PathBuf};
use glob::Pattern;

#[derive(Parser)]
#[command(name = "treee")]
#[command(about = "A fast tree command with gitignore support and flexible filtering")]
#[command(version = "0.1.0")]
struct Args {
    /// Directory to traverse
    #[arg(default_value = ".")]
    path: PathBuf,

    /// Maximum depth to traverse
    #[arg(short = 'L', long, default_value = "10")]
    depth: usize,

    /// Show hidden files
    #[arg(short = 'a', long)]
    all: bool,

    /// Don't use colors
    #[arg(long)]
    no_color: bool,

    /// Show directories only
    #[arg(short = 'd', long)]
    directories_only: bool,

    /// Include paths matching these glob patterns (can be used multiple times)
    #[arg(short = 'I', long = "include", action = clap::ArgAction::Append)]
    include_patterns: Vec<String>,

    /// Exclude paths matching these glob patterns (can be used multiple times)
    #[arg(short = 'E', long = "exclude", action = clap::ArgAction::Append)]
    exclude_patterns: Vec<String>,

    /// File name patterns to match (glob patterns, can be used multiple times)
    #[arg(short = 'P', long = "pattern", action = clap::ArgAction::Append)]
    file_patterns: Vec<String>,

    /// Disable gitignore rules
    #[arg(long = "no-git-ignore")]
    no_git_ignore: bool,

    /// Show only files (opposite of --directories-only)
    #[arg(short = 'f', long)]
    files_only: bool,

    /// Print full paths instead of tree format
    #[arg(long)]
    full_path: bool,
}

struct PathFilter {
    include_patterns: Vec<Pattern>,
    exclude_patterns: Vec<Pattern>,
    file_patterns: Vec<Pattern>,
}

impl PathFilter {
    fn new(
        include_patterns: &[String],
        exclude_patterns: &[String],
        file_patterns: &[String],
    ) -> Result<Self> {
        let include_patterns = include_patterns
            .iter()
            .map(|p| Pattern::new(p))
            .collect::<Result<Vec<_>, _>>()?;

        let exclude_patterns = exclude_patterns
            .iter()
            .map(|p| Pattern::new(p))
            .collect::<Result<Vec<_>, _>>()?;

        let file_patterns = file_patterns
            .iter()
            .map(|p| Pattern::new(p))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            include_patterns,
            exclude_patterns,
            file_patterns,
        })
    }

    fn should_include(&self, path: &Path) -> bool {
        let path_str = path.to_string_lossy();
        let file_name = path.file_name().map(|n| n.to_string_lossy()).unwrap_or_default();

        // Check exclude patterns first
        for pattern in &self.exclude_patterns {
            if pattern.matches(&path_str) || pattern.matches(&file_name) {
                return false;
            }
        }

        // For directories, always include them unless explicitly excluded
        // This allows traversal to find matching files in subdirectories
        if path.is_dir() {
            return true;
        }

        // For files, check include patterns
        if !self.include_patterns.is_empty() {
            let included = self.include_patterns.iter().any(|pattern| {
                pattern.matches(&path_str) || pattern.matches(&file_name)
            });
            if !included {
                return false;
            }
        }

        // Check file patterns for files (only if there are file patterns)
        if !self.file_patterns.is_empty() {
            return self.file_patterns.iter().any(|pattern| {
                pattern.matches(&file_name)
            });
        }

        true
    }
}

struct TreePrinter {
    use_color: bool,
    full_path: bool,
}

impl TreePrinter {
    fn new(use_color: bool, full_path: bool) -> Self {
        Self { use_color, full_path }
    }

    fn print_entry(&self, path: &Path, prefix: &str, is_last: bool, is_dir: bool) {
        if self.full_path {
            // Print full path
            let path_str = path.to_string_lossy();
            let formatted_path = if self.use_color {
                if is_dir {
                    path_str.blue().bold().to_string()
                } else {
                    path_str.to_string()
                }
            } else {
                path_str.to_string()
            };
            println!("{}", formatted_path);
        } else {
            // Print tree format
            let connector = if is_last { "└── " } else { "├── " };
            let name = path.file_name().unwrap().to_string_lossy();

            let formatted_name = if self.use_color {
                if is_dir {
                    name.blue().bold().to_string()
                } else {
                    name.to_string()
                }
            } else {
                name.to_string()
            };

            println!("{}{}{}", prefix, connector, formatted_name);
        }
    }

    fn get_child_prefix(&self, prefix: &str, is_last: bool) -> String {
        if self.full_path {
            String::new() // No prefix needed for full path mode
        } else {
            let extension = if is_last { "    " } else { "│   " };
            format!("{}{}", prefix, extension)
        }
    }
}

fn main() -> Result<()> {
    let args = Args::parse();

    if !args.path.exists() {
        eprintln!("Error: Path '{}' does not exist", args.path.display());
        std::process::exit(1);
    }

    // Validate conflicting options
    if args.directories_only && args.files_only {
        eprintln!("Error: Cannot use both --directories-only and --files-only");
        std::process::exit(1);
    }

    let use_color = !args.no_color && atty::is(atty::Stream::Stdout);
    let printer = TreePrinter::new(use_color, args.full_path);

    // Create path filter
    let path_filter = PathFilter::new(
        &args.include_patterns,
        &args.exclude_patterns,
        &args.file_patterns,
    )?;

    // Print the root directory (only in tree mode)
    if !args.full_path {
        let root_name = args.path.file_name()
            .unwrap_or_else(|| args.path.as_os_str())
            .to_string_lossy();

        let formatted_root = if use_color {
            root_name.blue().bold().to_string()
        } else {
            root_name.to_string()
        };

        println!("{}", formatted_root);
    }

    // Build the walker
    let mut builder = WalkBuilder::new(&args.path);
    builder
        .max_depth(Some(args.depth))
        .hidden(!args.all)
        .git_ignore(!args.no_git_ignore)
        .git_exclude(!args.no_git_ignore)
        .git_global(!args.no_git_ignore);

    let walker = builder.build();

    // Collect entries and organize them
    let mut entries: Vec<_> = walker
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            let path = entry.path();
            // Skip the root directory itself
            if path == args.path {
                return false;
            }

            // Apply path filter
            if !path_filter.should_include(path) {
                return false;
            }

            // Filter directories only if requested
            if args.directories_only && !path.is_dir() {
                return false;
            }

            // Filter files only if requested
            if args.files_only && !path.is_file() {
                return false;
            }

            true
        })
        .collect();

    // Sort entries by path
    entries.sort_by(|a, b| a.path().cmp(b.path()));

    // Group entries by their parent directory
    let mut dir_contents: std::collections::HashMap<PathBuf, Vec<_>> = std::collections::HashMap::new();

    for entry in entries {
        let path = entry.path();
        if let Some(parent) = path.parent() {
            dir_contents.entry(parent.to_path_buf()).or_default().push(path.to_path_buf());
        }
    }

    // Print the tree recursively
    print_tree_recursive(&args.path, &dir_contents, &printer, "", true)?;

    Ok(())
}

fn print_tree_recursive(
    current_dir: &Path,
    dir_contents: &std::collections::HashMap<PathBuf, Vec<PathBuf>>,
    printer: &TreePrinter,
    prefix: &str,
    _is_last: bool,
) -> Result<()> {
    if let Some(children) = dir_contents.get(current_dir) {
        let mut sorted_children = children.clone();
        sorted_children.sort();

        for (i, child_path) in sorted_children.iter().enumerate() {
            let is_last = i == sorted_children.len() - 1;
            let is_dir = child_path.is_dir();

            printer.print_entry(child_path, prefix, is_last, is_dir);

            if is_dir {
                let child_prefix = printer.get_child_prefix(prefix, is_last);
                print_tree_recursive(child_path, dir_contents, printer, &child_prefix, is_last)?;
            }
        }
    }

    Ok(())
}

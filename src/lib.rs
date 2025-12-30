//! `highlite` — A configurable syntax-highlighting filter for terminal output.
//!
//! `highlite` reads text from a file or standard input and highlights matching
//! keywords or regular expressions according to a user-provided YAML
//! configuration file.
//!
//! ## Features
//!
//! - Highlight fixed keywords or regular expressions
//! - Per-rule and global case-insensitive matching
//! - Support for preset ANSI colors and 24-bit RGB colors
//! - Read from files or `stdin`
//! - Efficient multi-pattern matching using a single compiled regex
//!
//! ## Example
//!
//! ```bash
//! highlite --config rules.yaml --file file.txt
//!
//! cat file.txt | highlite --config rules.yaml
//! ```

use std::io::{BufRead, Write};

/// Types related to highlighting rules and color definitions.
///
/// This module defines the data structures used to describe how text
/// should be highlighted, including keywords, matching behavior, and
/// ANSI color output.
pub mod rules;

/// Command-line argument parsing and configuration loading.
///
/// This module defines CLI options and functions for loading and resolving
/// highlighting rules from YAML configuration files, including recursive
/// `include` directives.
pub mod arg_parser;

/// High-performance text highlighting engine.
///
/// This module compiles all rules into a single regular expression and
/// efficiently renders matched text with ANSI color sequences.
///
/// # Examples
///
/// ```rust
/// use highlite::highlight::HighlightingEngine;
/// use highlite::rules::{Color, Rule};
/// let rules = vec![
///     Rule {
///         keyword: "error".into(),
///         color: Color::Preset{ name: "Red".into() },
///         is_regex: false,
///         ignore_case: false,
///     },
/// ];
///
/// let engine = HighlightingEngine::new(&rules, true).unwrap();
///
/// let out = engine.highlight_line("An error occurred\n");
///
/// assert!(out.contains("\x1b[31m"));
/// ```
pub mod highlight;
mod preset;

/// Executes the main program logic using the provided CLI configuration.
///
/// This function loads the highlighting rules from the configuration file,
/// initializes the highlighting engine, and processes either the specified
/// input file or standard input.
///
/// If no input file is provided, the function reads from `stdin`.
/// When `stdin` is connected to a terminal, an informational message is printed
/// to stderr before waiting for input.
///
/// # Arguments
///
/// * `cli_args` - Parsed command-line arguments controlling input, configuration,
///   and matching behavior.
///
/// # Errors
///
/// Returns an error if:
///
/// * The configuration path is missing
/// * The configuration file cannot be read or parsed
/// * The input file cannot be opened
/// * An I/O error occurs while reading input or writing output
/// * The highlighting engine fails to initialize
///
/// # Examples
///
/// ```no_run
/// use highlite::{run, arg_parser::CliArgs};
///
/// let cli_args = CliArgs {
///     ignore_case: false,
///     file: Some(String::from("path/to/file").into()),
///     config: Some(String::from("path/to/config.yaml").into()),
///     follow_file: None,
///     follow_journal: false,
///     preset: None,
/// };
///
/// run(cli_args).unwrap();
/// ```
///
/// This function flushes all output before returning.
///
pub fn run(cli_args: arg_parser::CliArgs) -> anyhow::Result<()> {
    use std::fs;
    use std::io::{BufReader, BufWriter, IsTerminal, Write};
    use std::process::{Command, Stdio};


    let raw_rules = if let Some(config_path) = cli_args.config.as_ref() {
        arg_parser::load_rules_from_file(config_path)?
    } else if let Some(preset_name) = cli_args.preset.as_ref() {
        preset::get_preset(preset_name)?
    } else {
        // 默认预设
        preset::get_preset("logs")?
    };

    let engine = highlight::HighlightingEngine::new(&raw_rules, cli_args.ignore_case)?;
    let stdout = std::io::stdout();
    let mut writer = BufWriter::new(stdout.lock());

    // 如果是跟随日志选项
    if cli_args.follow_journal {
        let child = Command::new("journalctl")
            .args(["-f"])
            .stdout(Stdio::piped())
            .spawn()?;
        let stdout = child.stdout.unwrap();
        process_stream(BufReader::new(stdout), &engine, &mut writer)?;
    } else if let Some(path) = cli_args.follow_file {
        let child = Command::new("tail")
            .args(["-f", &path.to_string_lossy()])
            .stdout(Stdio::piped())
            .spawn()?;
        let stdout = child.stdout.unwrap();
        process_stream(BufReader::new(stdout), &engine, &mut writer)?;
    } else if let Some(path) = cli_args.file {
        let f = fs::File::open(path)?;
        process_stream(BufReader::new(f), &engine, &mut writer)?;
    } else {
        if std::io::stdin().is_terminal() {
            eprintln!("(Info: Waiting for stdin... Press Ctrl+D to end)");
        }
        process_stream(BufReader::new(std::io::stdin()), &engine, &mut writer)?;
    }

    writer.flush()?;
    Ok(())
}

/// Processes a buffered input stream and writes highlighted output.
///
/// This function reads input line by line, applies syntax highlighting,
/// and writes the result to the provided output writer.
///
/// String buffers are reused across iterations to reduce allocations.
///
/// # Errors
///
/// Returns an error if an I/O error occurs while reading or writing.
fn process_stream<R: BufRead, W: Write>(
    mut reader: R,
    engine: &highlight::HighlightingEngine,
    writer: &mut W,
) -> anyhow::Result<()> {
    let mut line_buffer = String::new();
    let mut out_buffer = String::new();

    // 循环复用 String 内存，避免每行都分配内存
    while reader.read_line(&mut line_buffer)? > 0 {
        engine.render_line(&line_buffer, &mut out_buffer);
        writer.write_all(out_buffer.as_bytes())?;
        line_buffer.clear();
    }
    Ok(())
}


#[cfg(test)]
mod tests {
    use crate::highlight::HighlightingEngine;
    use crate::rules::{Color, Rule};

    #[test]
    fn rule_level_ignore_case_works() {
        let rules = vec![Rule {
            keyword: "error".into(),
            color: Color::Preset { name: "Red".into() },
            is_regex: false,
            ignore_case: true,
        }];

        let engine = HighlightingEngine::new(&rules, false).unwrap();
        let mut out = String::new();

        engine.render_line("ERROR\n", &mut out);
        assert!(out.contains("\x1b[31mERROR\x1b[0m"));
    }

    #[test]
    fn cli_ignore_case_overrides_rules() {
        let rules = vec![Rule {
            keyword: "error".into(),
            color: Color::Preset { name: "Red".into() },
            is_regex: false,
            ignore_case: false,
        }];

        let engine = HighlightingEngine::new(&rules, true).unwrap();
        let mut out = String::new();

        engine.render_line("ERROR\n", &mut out);
        assert!(out.contains("\x1b[31mERROR\x1b[0m"));
    }

    #[test]
    fn case_sensitive_rule_does_not_match() {
        let rules = vec![Rule {
            keyword: "error".into(),
            color: Color::Preset { name: "Red".into() },
            is_regex: false,
            ignore_case: false,
        }];

        let engine = HighlightingEngine::new(&rules, false).unwrap();
        let mut out = String::new();

        engine.render_line("ERROR\n", &mut out);
        assert!(!out.contains("\x1b[31m"));
    }
}

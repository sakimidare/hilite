use serde::Deserialize;

/// A single highlighting rule.
///
/// Each rule defines a keyword or pattern to match, along with the color
/// used to render matched text.
///
/// # Case sensitivity
///
/// By default, matching is case-sensitive.
/// If `ignore_case` is set to `true`, this rule will match text
/// case-insensitively.
///
/// Note:
/// - If the CLI flag `--ignore-case` is provided, it overrides this
///   setting and forces all rules to be case-insensitive.
///
/// # YAML
/// Rules are typically loaded from a YAML configuration file.
///
/// # Examples
///
/// ```yaml
/// rules:
///   - keyword: "ERROR"
///     color: { type: "Red" }
///     is_regex: false
///   - keyword: "//.*|/\\*.*\\*/"
///     is_regex: true
///     ignore_case: false
///     color: { r: 106, g: 153, b: 85 }
/// ```
#[derive(Debug, Clone, Deserialize)]
pub struct Rule {
    pub keyword: String,
    pub color: Color,
    #[serde(default)]
    pub is_regex: bool,
    #[serde(default)]
    pub ignore_case: bool,
}

/// A predefined ANSI color.
///
/// These colors correspond to standard 8-color ANSI escape sequences.
#[derive(Debug, Copy, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "PascalCase", content = "value")]
pub enum PresetColor {
    Red,
    Yellow,
    Blue,
    Green,
    Cyan,
    Magenta,
}

/// A color specification for highlighted text.
///
/// Colors can be specified either as a predefined ANSI color
/// or as a 24-bit RGB value.
///
/// # Examples
///
/// Using a preset ANSI color:
///
/// ```yaml
/// color: { type: Red }
/// ```
///
/// Using a 24-bit RGB value:
/// ```yaml
/// color: { r: 181, g: 206, b: 168 }
/// ```
#[derive(Debug, Copy, Clone, Deserialize)]
#[serde(untagged)]
pub enum Color {
    Preset(PresetColor),
    RGB { r: u8, g: u8, b: u8 },
}

impl Color {
    /// Converts this color into an ANSI escape sequence.
    ///
    /// The returned string enables the color when written to a terminal.
    /// Callers are responsible for resetting formatting (e.g. with `\x1b[0m`)
    /// after use.
    pub fn to_ansi(&self) -> String {
        match self {
            Color::Preset(p) => match p {
                PresetColor::Red => "\x1b[31m".to_string(),
                PresetColor::Yellow => "\x1b[33m".to_string(),
                PresetColor::Blue => "\x1b[34m".to_string(),
                PresetColor::Green => "\x1b[32m".to_string(),
                PresetColor::Cyan => "\x1b[36m".to_string(),
                PresetColor::Magenta => "\x1b[35m".to_string(),
            },
            Color::RGB { r, g, b } => format!("\x1b[38;2;{};{};{}m", r, g, b),
        }
    }
}
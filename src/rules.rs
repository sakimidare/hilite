macro_rules! define_preset_colors {
    (
        $(
            $Name:ident => {
                ansi: $ansi:expr,
                aliases: [$($alias:expr),+ $(,)?]
            }
        ),+ $(,)?
    ) => {
        #[derive(Debug, Copy, Clone)]
        pub(crate) enum PresetColor {
            $($Name),+
        }

        impl PresetColor {
            pub(crate) fn to_ansi(self) -> &'static str {
                match self {
                    $(PresetColor::$Name => $ansi),+
                }
            }

            pub(crate) fn parse(name: &str) -> anyhow::Result<Self> {
                let name = name.to_ascii_lowercase();
                match name.as_str() {
                    $(
                        $($alias)|+ => Ok(PresetColor::$Name),
                    )+
                    _ => anyhow::bail!("Unknown preset color: {}", name),
                }
            }
        }
    };
}

define_preset_colors! {
    Red => {
        ansi: "\x1b[31m",
        aliases: ["red"]
    },
    Yellow => {
        ansi: "\x1b[33m",
        aliases: ["yellow", "yel"]
    },
    Blue => {
        ansi: "\x1b[34m",
        aliases: ["blue"]
    },
    Green => {
        ansi: "\x1b[32m",
        aliases: ["green"]
    },
    Cyan => {
        ansi: "\x1b[36m",
        aliases: ["cyan"]
    },
    Magenta => {
        ansi: "\x1b[35m",
        aliases: ["magenta", "purple"]
    },
}


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
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum Color {
    Preset{name: String},
    RGB { r: u8, g: u8, b: u8 },
}

impl Color {
    /// Converts this color into an ANSI escape sequence.
    ///
    /// The returned string enables the color when written to a terminal.
    /// Callers are responsible for resetting formatting (e.g. with `\x1b[0m`)
    /// after use.
    pub(crate) fn to_ansi(&self) -> anyhow::Result<String> {
        match self {
            Color::Preset { name } => {
                let preset = PresetColor::parse(name)?;
                Ok(preset.to_ansi().parse()?)
            }
            Color::RGB { r, g, b } => {
                Ok(format!("\x1b[38;2;{};{};{}m", r, g, b))
            }
        }
    }
}
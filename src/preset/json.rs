use crate::rules::{Color, Rule};
use once_cell::sync::Lazy;
use std::convert::Into;

/// JSON 预设
pub(super) static JSON: Lazy<Vec<Rule>> = Lazy::new(|| vec![
    // ===== Keys =====
    Rule {
        keyword: r#""[^"]+"\s*:"#.to_string(),
        is_regex: true,
        ignore_case: false,
        color: Color::RGB { r: 214, g: 157, b: 133 }, // purple-ish
    },
    // ===== Strings =====
    Rule {
        keyword: r#""([^"\\]|\\.)*""#.to_string(),
        is_regex: true,
        ignore_case: false,
        color: Color::RGB { r: 181, g: 206, b: 168 }, // green-ish
    },
    // ===== Numbers =====
    Rule {
        keyword: r"\b\d+(\.\d+)?\b".to_string(),
        is_regex: true,
        ignore_case: false,
        color: Color::RGB { r: 206, g: 145, b: 120 },
    },
    // ===== Booleans / null =====
    Rule {
        keyword: r"\b(true|false|null)\b".to_string(),
        is_regex: true,
        ignore_case: true,
        color: Color::Preset { name: "Cyan".into() },
    },
]);

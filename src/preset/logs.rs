use once_cell::sync::Lazy;
use crate::rules::{Rule, Color, PresetColor};

pub static LOGS: Lazy<Vec<Rule>> = Lazy::new(|| vec![
    // ===== Log levels =====
    Rule {
        keyword: r"\b(FATAL|CRITICAL)\b".to_string(),
        is_regex: true,
        ignore_case: true,
        color: Color::RGB { r: 255, g: 0, b: 0 },
    },
    Rule {
        keyword: r"\bERROR\b".to_string(),
        is_regex: true,
        ignore_case: true,
        color: Color::Preset(PresetColor::Red),
    },
    Rule {
        keyword: r"\bWARN(ING)?\b".to_string(),
        is_regex: true,
        ignore_case: true,
        color: Color::Preset(PresetColor::Yellow),
    },
    Rule {
        keyword: r"\bINFO\b".to_string(),
        is_regex: true,
        ignore_case: true,
        color: Color::Preset(PresetColor::Green),
    },
    Rule {
        keyword: r"\bDEBUG\b".to_string(),
        is_regex: true,
        ignore_case: true,
        color: Color::Preset(PresetColor::Cyan),
    },
    Rule {
        keyword: r"\bTRACE\b".to_string(),
        is_regex: true,
        ignore_case: true,
        color: Color::RGB { r: 160, g: 160, b: 160 },
    },

    // ===== Timestamps =====
    Rule {
        keyword: r"\b\d{4}-\d{2}-\d{2}[ T]\d{2}:\d{2}:\d{2}(\.\d+)?\b".to_string(),
        is_regex: true,
        ignore_case: false,
        color: Color::RGB { r: 180, g: 180, b: 180 },
    },

    // ===== Thread / PID =====
    Rule {
        keyword: r"\[(main|worker-\d+|thread-\d+)\]".to_string(),
        is_regex: true,
        ignore_case: true,
        color: Color::RGB { r: 140, g: 140, b: 255 },
    },
    Rule {
        keyword: r"\bpid=\d+\b".to_string(),
        is_regex: true,
        ignore_case: false,
        color: Color::RGB { r: 140, g: 140, b: 255 },
    },

    // ===== Source / Module =====
    Rule {
        keyword: r"\b([A-Za-z_][\w$]*\.)+[A-Za-z_][\w$]*\b".to_string(),
        is_regex: true,
        ignore_case: false,
        color: Color::RGB { r: 86, g: 156, b: 214 },
    },

    // ===== Common fields =====
    Rule {
        keyword: r"\b(user|uid|id|request_id|trace_id|span_id)=\S+\b".to_string(),
        is_regex: true,
        ignore_case: true,
        color: Color::RGB { r: 206, g: 145, b: 120 },
    },

    // ===== Numbers =====
    Rule {
        keyword: r"\b\d+(\.\d+)?\b".to_string(),
        is_regex: true,
        ignore_case: false,
        color: Color::RGB { r: 181, g: 206, b: 168 },
    },

    // ===== File paths =====
    Rule {
        keyword: r"(/[^ \t\n]+)+".to_string(),
        is_regex: true,
        ignore_case: false,
        color: Color::RGB { r: 152, g: 195, b: 121 },
    },

    // ===== Quoted strings =====
    Rule {
        keyword: r#""([^"\\]|\\.)*""#.to_string(),
        is_regex: true,
        ignore_case: false,
        color: Color::RGB { r: 214, g: 157, b: 133 },
    },
]);

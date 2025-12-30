use crate::rules::{Color, Rule};
use once_cell::sync::Lazy;
use std::convert::Into;

pub(super) static LOGS: Lazy<Vec<Rule>> = Lazy::new(|| vec![
    // 1. ===== Timestamps =====
    // 最优先匹配，防止日期中的数字和横杠被后续规则拆散
    Rule {
        keyword: r"\b\d{4}-\d{2}-\d{2}[ T]\d{2}:\d{2}:\d{2}(\.\d+)?\b".to_string(),
        is_regex: true,
        ignore_case: false,
        color: Color::RGB { r: 180, g: 180, b: 180 },
    },

    // 2. ===== IP Addresses =====
    // 必须在普通数字之前，否则 192.168... 会被匹配成 4 个数字
    Rule {
        keyword: r"\b\d{1,3}(\.\d{1,3}){3}\b".to_string(),
        is_regex: true,
        ignore_case: false,
        color: Color::RGB { r: 255, g: 165, b: 0 },
    },
    Rule {
        keyword: r"\b([0-9a-fA-F]{0,4}:){1,7}[0-9a-fA-F]{0,4}\b".to_string(),
        is_regex: true,
        ignore_case: false,
        color: Color::RGB { r: 255, g: 165, b: 0 },
    },

    // 3. ===== URLs / Domains =====
    Rule {
        keyword: r"https?://[^\s/$.?#].[^\s]*".to_string(),
        is_regex: true,
        ignore_case: false,
        color: Color::RGB { r: 80, g: 200, b: 250 },
    },
    Rule {
        keyword: r"\b([a-zA-Z0-9-]+\.)+[a-zA-Z]{2,}\b".to_string(),
        is_regex: true,
        ignore_case: false,
        color: Color::RGB { r: 100, g: 150, b: 200 },
    },

    // 4. ===== JSON keys =====
    // 带有冒号的键，优先级高于普通引号字符串
    Rule {
        keyword: r#""[^"]+"\s*:"#.to_string(),
        is_regex: true,
        ignore_case: false,
        color: Color::RGB { r: 200, g: 100, b: 200 },
    },

    // 5. ===== Common fields (key=value) =====
    Rule {
        keyword: r"\b(user|uid|id|request_id|trace_id|span_id)=\S+\b".to_string(),
        is_regex: true,
        ignore_case: true,
        color: Color::RGB { r: 206, g: 145, b: 120 },
    },

    // 6. ===== Source / Module =====
    // 如 com.package.Class
    Rule {
        keyword: r"\b([A-Za-z_][\w$]*\.)+[A-Za-z_][\w$]*\b".to_string(),
        is_regex: true,
        ignore_case: false,
        color: Color::RGB { r: 86, g: 156, b: 214 },
    },

    // 7. ===== File paths =====
    Rule {
        keyword: r"(/[^ \t\n]+)+".to_string(),
        is_regex: true,
        ignore_case: false,
        color: Color::RGB { r: 152, g: 195, b: 121 },
    },

    // 8. ===== Log levels =====
    Rule {
        keyword: r"\b(FATAL|CRITICAL|FF)\b".to_string(),
        is_regex: true,
        ignore_case: true,
        color: Color::RGB { r: 255, g: 0, b: 0 },
    },
    Rule {
        keyword: r"\b(ERROR|EE)\b".to_string(),
        is_regex: true,
        ignore_case: true,
        color: Color::Preset { name: "Red".into() },
    },
    Rule {
        keyword: r"\b(WARN(ING)?|WW)\b".to_string(),
        is_regex: true,
        ignore_case: true,
        color: Color::Preset { name: "Yellow".into() },
    },
    Rule {
        // 适配 INFO, II
        keyword: r"\b(INFO|II)\b".to_string(),
        is_regex: true,
        ignore_case: true,
        color: Color::Preset { name: "Green".into() },
    },
    Rule {
        // 适配 DEBUG, DD
        keyword: r"\b(DEBUG|DD)\b".to_string(),
        is_regex: true,
        ignore_case: true,
        color: Color::Preset { name: "Cyan".into() },
    },
    Rule {
        // 适配 TRACE, VV (Verbose)
        keyword: r"\b(TRACE|VV)\b".to_string(),
        is_regex: true,
        ignore_case: true,
        color: Color::RGB { r: 160, g: 160, b: 160 },
    },

    // 9. ===== HTTP Methods / Status =====
    Rule {
        keyword: r"\b(GET|POST|PUT|DELETE|PATCH|OPTIONS|HEAD)\b".to_string(),
        is_regex: true,
        ignore_case: false,
        color: Color::RGB { r: 0, g: 200, b: 0 },
    },
    Rule {
        keyword: r"\b(1\d{2}|2\d{2}|3\d{2}|4\d{2}|5\d{2})\b".to_string(),
        is_regex: true,
        ignore_case: false,
        color: Color::RGB { r: 255, g: 140, b: 0 },
    },

    // 10. ===== Thread / PID =====
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

    // 11. ===== Exceptions / Stacktrace =====
    Rule {
        keyword: r"\b(Exception|Error|Traceback)\b".to_string(),
        is_regex: true,
        ignore_case: false,
        color: Color::RGB { r: 255, g: 50, b: 50 },
    },
    Rule {
        keyword: r"^\s+at\s+[^\s]+\([^\)]*\)".to_string(),
        is_regex: true,
        ignore_case: false,
        color: Color::RGB { r: 180, g: 180, b: 255 },
    },

    // 12. ===== SQL / Shell commands =====
    Rule {
        keyword: r"\b(SELECT|INSERT|UPDATE|DELETE|FROM|WHERE|JOIN|CREATE|DROP|ALTER)\b".to_string(),
        is_regex: true,
        ignore_case: true,
        color: Color::RGB { r: 0, g: 255, b: 200 },
    },
    Rule {
        keyword: r"(\$[a-zA-Z_][\w]*)".to_string(),
        is_regex: true,
        ignore_case: false,
        color: Color::RGB { r: 255, g: 200, b: 100 },
    },

    // 13. ===== Numbers =====
    // 放在倒数第二，作为剩余数字的保底匹配
    Rule {
        keyword: r"\b\d+(\.\d+)?\b".to_string(),
        is_regex: true,
        ignore_case: false,
        color: Color::RGB { r: 181, g: 206, b: 168 },
    },

    // 14. ===== Quoted strings =====
    // 放在最后，防止它吞掉 JSON key 或其他特定格式
    Rule {
        keyword: r#""([^"\\]|\\.)*""#.to_string(),
        is_regex: true,
        ignore_case: false,
        color: Color::RGB { r: 214, g: 157, b: 133 },
    },
]);
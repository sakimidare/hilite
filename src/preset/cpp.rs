use crate::rules::{Rule, Color, PresetColor};
use once_cell::sync::Lazy;

/// Built-in C++ syntax highlighting preset.
pub static CPP: Lazy<Vec<Rule>> = Lazy::new(|| {
    vec![
        // 1. 字符串
        Rule {
            keyword: r#""[^"\\]*(\\.[^"\\]*)*"|'[^'\\]*(\\.[^'\\]*)*'"#.into(),
            is_regex: true,
            ignore_case: false,
            color: Color::RGB { r: 206, g: 145, b: 120 },
        },
        // 2. 注释
        Rule {
            keyword: r"//.*|/\*.*\*/".into(),
            is_regex: true,
            ignore_case: false,
            color: Color::RGB { r: 106, g: 153, b: 85 },
        },
        // 3. 预处理宏
        Rule {
            keyword: r"^\s*#\s*(include|define|ifdef|ifndef|endif|if|else|pragma|line|error).*$".into(),
            is_regex: true,
            ignore_case: false,
            color: Color::Preset(PresetColor::Magenta),
        },
        // 4. 数字
        Rule {
            keyword: r"\b(0x[0-9a-fA-F]+|0b[01]+|\d+\.?\d*([eE][+-]?\d+)?|\d+)\b".into(),
            is_regex: true,
            ignore_case: false,
            color: Color::RGB { r: 181, g: 206, b: 168 },
        },
        // 5. 符号全集
        Rule {
            keyword: r"(->|::|<<=|>>=|==|!=|<=|>=|&&|\|\||\+\+|--|<<|>>|[\+\-\*\/%=&<>!&\|\^~\.\?:;])".into(),
            is_regex: true,
            ignore_case: false,
            color: Color::Preset(PresetColor::Red),
        },
        // 6. 括号
        Rule {
            keyword: r"[\(\)\{\}\[\]]".into(),
            is_regex: true,
            ignore_case: false,
            color: Color::RGB { r: 255, g: 215, b: 0 },
        },
        // 7. 控制流关键字
        Rule {
            keyword: r"\b(if|else|for|while|do|switch|case|default|return|break|continue|goto|throw|try|catch)\b".into(),
            is_regex: true,
            ignore_case: false,
            color: Color::RGB { r: 197, g: 134, b: 192 },
        },
        // 8. 类型与限定符
        Rule {
            keyword: r"\b(int|long|short|char|float|double|bool|void|size_t|u?int(8|16|32|64)_t|auto|unsigned|signed|const|static|inline|virtual|override|final|volatile|mutable|thread_local|explicit|enum|struct|class|union|typename|template)\b".into(),
            is_regex: true,
            ignore_case: false,
            color: Color::Preset(PresetColor::Blue),
        },
        // 9. 其他核心关键字
        Rule {
            keyword: r"\b(public|private|protected|using|namespace|friend|this|operator|new|delete|true|false|nullptr|constexpr|static_cast|dynamic_cast|reinterpret_cast|const_cast)\b".into(),
            is_regex: true,
            ignore_case: false,
            color: Color::Preset(PresetColor::Cyan),
        },
        // 10. std 命名空间
        Rule {
            keyword: r"\bstd::\w*".into(),
            is_regex: true,
            ignore_case: false,
            color: Color::Preset(PresetColor::Yellow),
        },
        // 11. PascalCase 类名
        Rule {
            keyword: r"\b[A-Z]\w*\b".into(),
            is_regex: true,
            ignore_case: false,
            color: Color::Preset(PresetColor::Green),
        },
    ]
});

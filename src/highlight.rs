use regex::Regex;

/// A compiled highlighting engine.
///
/// Internally, all rules are combined into a single regular expression
/// using named capture groups to efficiently determine which rule
/// produced a match.
pub struct HighlightingEngine {
    regex: Regex,
    cap_to_color: Vec<usize>,
    ansi_colors: Vec<String>,
}

impl HighlightingEngine {
    /// Creates a new highlighting engine from a list of rules.
    /// with case-insensitive matching, regardless of their individual
    /// `ignore_case` settings.
    ///
    /// All rules are compiled into a single regular expression to minimize
    /// matching overhead.
    ///
    /// # Errors
    ///
    /// Returns an error if the combined regular expression fails to compile.
    pub fn new(
        rules: &[crate::rules::Rule],
        force_ignore_case: bool,
    ) -> anyhow::Result<Self> {
        use regex::RegexBuilder;

        let mut patterns = Vec::with_capacity(rules.len());
        let mut ansi_colors = Vec::with_capacity(rules.len());

        // 1. 构造每条规则的正则片段
        for (i, rule) in rules.iter().enumerate() {
            let base_pat = if rule.is_regex {
                rule.keyword.clone()
            } else {
                regex::escape(&rule.keyword)
            };

            let effective_ignore_case = force_ignore_case || rule.ignore_case;

            let pat = if effective_ignore_case {
                // 使用 inline flag，做到 per-rule ignore_case
                format!("(?i:{})", base_pat)
            } else {
                base_pat
            };

            // 命名捕获组 r{i}
            patterns.push(format!("(?P<r{}>{})", i, pat));
            ansi_colors.push(rule.color.to_ansi());
        }

        // 2. 编译合并后的正则
        let regex = RegexBuilder::new(&patterns.join("|"))
            .multi_line(true)
            .dot_matches_new_line(false)
            .build()?;

        // 3. 建立 capture_index -> rule_index 的 O(1) 映射表
        //
        // cap_to_color[cap_idx] = rule_idx
        // 未使用的 capture index 用 usize::MAX 标记
        let mut cap_to_color = vec![usize::MAX; regex.captures_len()];

        for (cap_idx, name) in regex.capture_names().enumerate() {
            let Some(name) = name else { continue };
            let Some(idx) = name.strip_prefix('r') else { continue };
            let Ok(rule_idx) = idx.parse::<usize>() else { continue };

            cap_to_color[cap_idx] = rule_idx;
        }

        Ok(Self {
            regex,
            cap_to_color,
            ansi_colors,
        })
    }


    /// Renders a single line of input with highlighting applied.
    ///
    /// Matched segments are wrapped in ANSI color escape sequences.
    /// The output buffer is cleared before writing.
    ///
    /// # Examples
    ///
    /// This example is case-insensitive:
    /// ```rust
    /// # use highlite::highlight::HighlightingEngine;
    /// # use highlite::rules::{Rule, Color, PresetColor};
    /// let rules = vec![Rule {
    ///     keyword: "Ok".into(),
    ///     color: Color::Preset(PresetColor::Green),
    ///     is_regex: false,
    ///     ignore_case: true,
    /// }];
    ///
    /// let engine = HighlightingEngine::new(&rules, false).unwrap();
    /// let mut out = String::new();
    ///
    /// engine.render_line("Status: OK\n", &mut out);
    /// assert!(out.contains("\x1b[32mOK\x1b[0m"));
    /// ```
    ///
    /// But this is not:
    /// ```rust
    /// # use highlite::highlight::HighlightingEngine;
    /// # use highlite::rules::{Rule, Color, PresetColor};
    /// let rules = vec![Rule {
    ///     keyword: "Ok".into(),
    ///     color: Color::Preset(PresetColor::Green),
    ///     is_regex: false,
    ///     ignore_case: false,
    /// }];
    ///
    /// let engine = HighlightingEngine::new(&rules, false).unwrap();
    /// let mut out = String::new();
    ///
    /// engine.render_line("Status: OK\n", &mut out);
    /// assert!(!out.contains("\x1b[32mOK\x1b[0m"));
    /// ```
    pub fn render_line(&self, input: &str, output: &mut String) {
        output.clear();
        let mut last_match = 0;

        for caps in self.regex.captures_iter(input) {
            let m = caps.get(0).unwrap();

            output.push_str(&input[last_match..m.start()]);

            for (cap_idx, color_idx) in self.cap_to_color.iter().enumerate() {
                if *color_idx == usize::MAX {
                    continue;
                }
                if let Some(sub) = caps.get(cap_idx) {
                    output.push_str(&self.ansi_colors[*color_idx]);
                    output.push_str(sub.as_str());
                    output.push_str("\x1b[0m");
                    break;
                }
            }

            last_match = m.end();
        }
        // 写入剩余文本
        output.push_str(&input[last_match..]);
    }
}
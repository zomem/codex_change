use ratatui::style::Style;
use ratatui::style::Stylize;
use ratatui::text::Line;
use ratatui::text::Span;
use std::sync::OnceLock;
use tree_sitter_highlight::Highlight;
use tree_sitter_highlight::HighlightConfiguration;
use tree_sitter_highlight::HighlightEvent;
use tree_sitter_highlight::Highlighter;

// Ref: https://github.com/tree-sitter/tree-sitter-bash/blob/master/queries/highlights.scm
#[derive(Copy, Clone)]
enum GenericHighlight {
    Comment,
    Constant,
    Embedded,
    Function,
    Keyword,
    Number,
    Operator,
    Property,
    String,
}

impl GenericHighlight {
    const ALL: [Self; 9] = [
        Self::Comment,
        Self::Constant,
        Self::Embedded,
        Self::Function,
        Self::Keyword,
        Self::Number,
        Self::Operator,
        Self::Property,
        Self::String,
    ];

    const fn as_str(self) -> &'static str {
        match self {
            Self::Comment => "comment",
            Self::Constant => "constant",
            Self::Embedded => "embedded",
            Self::Function => "function",
            Self::Keyword => "keyword",
            Self::Number => "number",
            Self::Operator => "operator",
            Self::Property => "property",
            Self::String => "string",
        }
    }

    fn style(self) -> Style {
        match self {
            Self::Comment => Style::default().green().dim(),
            Self::Constant => Style::default().red(),
            Self::Embedded => Style::default().cyan(),
            Self::Function => Style::default().blue(),
            Self::Keyword => Style::default().magenta(),
            Self::Number => Style::default().yellow(),
            Self::Operator => Style::default().dim(),
            Self::Property => Style::default().blue(),
            Self::String => Style::default().green(),
        }
    }
}

static BASH_HIGHLIGHT_CONFIG: OnceLock<HighlightConfiguration> = OnceLock::new();
static RUST_HIGHLIGHT_CONFIG: OnceLock<HighlightConfiguration> = OnceLock::new();
static PYTHON_HIGHLIGHT_CONFIG: OnceLock<HighlightConfiguration> = OnceLock::new();
static JAVASCRIPT_HIGHLIGHT_CONFIG: OnceLock<HighlightConfiguration> = OnceLock::new();
static JSON_HIGHLIGHT_CONFIG: OnceLock<HighlightConfiguration> = OnceLock::new();
static TYPESCRIPT_HIGHLIGHT_CONFIG: OnceLock<HighlightConfiguration> = OnceLock::new();
static HTML_HIGHLIGHT_CONFIG: OnceLock<HighlightConfiguration> = OnceLock::new();
static CSS_HIGHLIGHT_CONFIG: OnceLock<HighlightConfiguration> = OnceLock::new();
static SQL_HIGHLIGHT_CONFIG: OnceLock<HighlightConfiguration> = OnceLock::new();

fn highlight_names() -> &'static [&'static str] {
    static NAMES: OnceLock<[&'static str; GenericHighlight::ALL.len()]> = OnceLock::new();
    NAMES
        .get_or_init(|| GenericHighlight::ALL.map(GenericHighlight::as_str))
        .as_slice()
}

fn bash_highlight_config() -> &'static HighlightConfiguration {
    BASH_HIGHLIGHT_CONFIG.get_or_init(|| {
        let language = tree_sitter_bash::LANGUAGE.into();
        #[expect(clippy::expect_used)]
        let mut config = HighlightConfiguration::new(
            language,
            "bash",
            tree_sitter_bash::HIGHLIGHT_QUERY,
            "",
            "",
        )
        .expect("load bash highlight query");
        config.configure(highlight_names());
        config
    })
}

const RUST_HIGHLIGHT_QUERY: &str = r#"
(line_comment) @comment
(block_comment) @comment
(string_literal) @string
(raw_string_literal) @string
(integer_literal) @number
(float_literal) @number
"#;

fn rust_highlight_config() -> &'static HighlightConfiguration {
    RUST_HIGHLIGHT_CONFIG.get_or_init(|| {
        let language = tree_sitter_rust::LANGUAGE.into();
        #[expect(clippy::expect_used)]
        let mut config = HighlightConfiguration::new(
            language,
            "rust",
            tree_sitter_rust::HIGHLIGHTS_QUERY,
            "",
            "",
        )
        .expect("load rust highlight query");
        config.configure(highlight_names());
        config
    })
}

fn python_highlight_config() -> &'static HighlightConfiguration {
    PYTHON_HIGHLIGHT_CONFIG.get_or_init(|| {
        let language = tree_sitter_python::LANGUAGE.into();
        #[expect(clippy::expect_used)]
        let mut config = HighlightConfiguration::new(
            language,
            "python",
            tree_sitter_python::HIGHLIGHTS_QUERY,
            "",
            "",
        )
        .expect("load python highlight query");
        config.configure(highlight_names());
        config
    })
}

fn javascript_highlight_config() -> &'static HighlightConfiguration {
    JAVASCRIPT_HIGHLIGHT_CONFIG.get_or_init(|| {
        let language = tree_sitter_javascript::LANGUAGE.into();
        #[expect(clippy::expect_used)]
        let mut config = HighlightConfiguration::new(
            language,
            "javascript",
            tree_sitter_javascript::HIGHLIGHT_QUERY,
            "",
            "",
        )
        .expect("load javascript highlight query");
        config.configure(highlight_names());
        config
    })
}

fn json_highlight_config() -> &'static HighlightConfiguration {
    JSON_HIGHLIGHT_CONFIG.get_or_init(|| {
        let language = tree_sitter_json::LANGUAGE.into();
        #[expect(clippy::expect_used)]
        let mut config = HighlightConfiguration::new(
            language,
            "json",
            tree_sitter_json::HIGHLIGHTS_QUERY,
            "",
            "",
        )
        .expect("load json highlight query");
        config.configure(highlight_names());
        config
    })
}

fn typescript_highlight_config() -> &'static HighlightConfiguration {
    TYPESCRIPT_HIGHLIGHT_CONFIG.get_or_init(|| {
        let language = tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into();
        #[expect(clippy::expect_used)]
        let mut config = HighlightConfiguration::new(
            language,
            "typescript",
            tree_sitter_typescript::HIGHLIGHTS_QUERY,
            "",
            "",
        )
        .expect("load typescript highlight query");
        config.configure(highlight_names());
        config
    })
}

fn html_highlight_config() -> &'static HighlightConfiguration {
    HTML_HIGHLIGHT_CONFIG.get_or_init(|| {
        let language = tree_sitter_html::LANGUAGE.into();
        #[expect(clippy::expect_used)]
        let mut config = HighlightConfiguration::new(
            language,
            "html",
            tree_sitter_html::HIGHLIGHTS_QUERY,
            "",
            "",
        )
        .expect("load html highlight query");
        config.configure(highlight_names());
        config
    })
}

fn css_highlight_config() -> &'static HighlightConfiguration {
    CSS_HIGHLIGHT_CONFIG.get_or_init(|| {
        let language = tree_sitter_css::LANGUAGE.into();
        #[expect(clippy::expect_used)]
        let mut config =
            HighlightConfiguration::new(language, "css", tree_sitter_css::HIGHLIGHTS_QUERY, "", "")
                .expect("load css highlight query");
        config.configure(highlight_names());
        config
    })
}

fn sql_highlight_config() -> &'static HighlightConfiguration {
    SQL_HIGHLIGHT_CONFIG.get_or_init(|| {
        let language = tree_sitter_sequel::LANGUAGE.into();
        #[expect(clippy::expect_used)]
        let mut config = HighlightConfiguration::new(
            language,
            "sql",
            tree_sitter_sequel::HIGHLIGHTS_QUERY,
            "",
            "",
        )
        .expect("load sql highlight query");
        config.configure(highlight_names());
        config
    })
}

fn highlight_for(highlight: Highlight) -> GenericHighlight {
    GenericHighlight::ALL[highlight.0]
}

fn push_segment(lines: &mut Vec<Line<'static>>, segment: &str, style: Option<Style>) {
    for (i, part) in segment.split('\n').enumerate() {
        if i > 0 {
            lines.push(Line::from(""));
        }
        if part.is_empty() {
            continue;
        }
        let span = match style {
            Some(style) => Span::styled(part.to_string(), style),
            None => part.to_string().into(),
        };
        if let Some(last) = lines.last_mut() {
            last.spans.push(span);
        }
    }
}

fn highlight_to_lines(script: &str, config: &HighlightConfiguration) -> Vec<Line<'static>> {
    let mut highlighter = Highlighter::new();
    let iterator = match highlighter.highlight(config, script.as_bytes(), None, |_| None) {
        Ok(iter) => iter,
        Err(_) => return vec![script.to_string().into()],
    };

    let mut lines: Vec<Line<'static>> = vec![Line::from("")];
    let mut highlight_stack: Vec<Highlight> = Vec::new();

    for event in iterator {
        match event {
            Ok(HighlightEvent::HighlightStart(highlight)) => highlight_stack.push(highlight),
            Ok(HighlightEvent::HighlightEnd) => {
                highlight_stack.pop();
            }
            Ok(HighlightEvent::Source { start, end }) => {
                if start == end {
                    continue;
                }
                let style = highlight_stack.last().map(|h| highlight_for(*h).style());
                push_segment(&mut lines, &script[start..end], style);
            }
            Err(_) => return vec![script.to_string().into()],
        }
    }

    if lines.is_empty() {
        return vec![Line::from("")];
    }

    lines
}

/// Convert a bash script into per-line styled content using tree-sitter's
/// bash highlight query. The highlighter is streamed so multi-line content is
/// split into `Line`s while preserving style boundaries.
pub(crate) fn highlight_bash_to_lines(script: &str) -> Vec<Line<'static>> {
    highlight_to_lines(script, bash_highlight_config())
}

pub(crate) fn highlight_rust_to_lines(code: &str) -> Vec<Line<'static>> {
    highlight_to_lines(code, rust_highlight_config())
}

pub(crate) fn highlight_python_to_lines(code: &str) -> Vec<Line<'static>> {
    highlight_to_lines(code, python_highlight_config())
}

pub(crate) fn highlight_javascript_to_lines(code: &str) -> Vec<Line<'static>> {
    highlight_to_lines(code, javascript_highlight_config())
}

pub(crate) fn highlight_json_to_lines(code: &str) -> Vec<Line<'static>> {
    highlight_to_lines(code, json_highlight_config())
}

pub(crate) fn highlight_typescript_to_lines(code: &str) -> Vec<Line<'static>> {
    highlight_to_lines(code, typescript_highlight_config())
}

pub(crate) fn highlight_html_to_lines(code: &str) -> Vec<Line<'static>> {
    highlight_to_lines(code, html_highlight_config())
}

pub(crate) fn highlight_css_to_lines(code: &str) -> Vec<Line<'static>> {
    highlight_to_lines(code, css_highlight_config())
}

pub(crate) fn highlight_sql_to_lines(code: &str) -> Vec<Line<'static>> {
    highlight_to_lines(code, sql_highlight_config())
}

pub(crate) fn highlight_log_to_lines(code: &str) -> Vec<Line<'static>> {
    code.split('\n')
        .map(|line| {
            if line.is_empty() {
                return Line::from("");
            }
            let lower = line.to_ascii_lowercase();
            if lower.contains("error") {
                Line::from(line.to_string().red().bold())
            } else if lower.contains("warn") {
                Line::from(line.to_string().yellow().bold())
            } else if lower.contains("info") {
                Line::from(line.to_string().cyan())
            } else if lower.contains("debug") {
                Line::from(line.to_string().magenta())
            } else if lower.contains("trace") {
                Line::from(line.to_string().dim())
            } else {
                Line::from(line.to_string())
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use ratatui::style::Modifier;

    fn reconstructed(lines: &[Line<'static>]) -> String {
        lines
            .iter()
            .map(|l| {
                l.spans
                    .iter()
                    .map(|sp| sp.content.clone())
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn dimmed_tokens(lines: &[Line<'static>]) -> Vec<String> {
        lines
            .iter()
            .flat_map(|l| l.spans.iter())
            .filter(|sp| sp.style.add_modifier.contains(Modifier::DIM))
            .map(|sp| sp.content.clone().into_owned())
            .map(|token| token.trim().to_string())
            .filter(|token| !token.is_empty())
            .collect()
    }

    #[test]
    fn dims_expected_bash_operators() {
        let s = "echo foo && bar || baz | qux & (echo hi)";
        let lines = highlight_bash_to_lines(s);
        assert_eq!(reconstructed(&lines), s);

        let dimmed = dimmed_tokens(&lines);
        assert!(dimmed.contains(&"&&".to_string()));
        assert!(dimmed.contains(&"|".to_string()));
        assert!(!dimmed.contains(&"echo".to_string()));
    }

    #[test]
    fn dims_redirects_and_strings() {
        let s = "echo \"hi\" > out.txt; echo 'ok'";
        let lines = highlight_bash_to_lines(s);
        assert_eq!(reconstructed(&lines), s);

        let dimmed = dimmed_tokens(&lines);
        assert!(dimmed.contains(&">".to_string()));
        assert!(dimmed.contains(&"\"hi\"".to_string()));
        assert!(dimmed.contains(&"'ok'".to_string()));
    }

    #[test]
    fn highlights_command_and_strings() {
        let s = "echo \"hi\"";
        let lines = highlight_bash_to_lines(s);
        let mut echo_style = None;
        let mut string_style = None;
        for span in &lines[0].spans {
            let text = span.content.as_ref();
            if text == "echo" {
                echo_style = Some(span.style);
            }
            if text == "\"hi\"" {
                string_style = Some(span.style);
            }
        }
        let echo_style = echo_style.expect("echo span missing");
        let string_style = string_style.expect("string span missing");
        assert!(echo_style.fg.is_none());
        assert!(!echo_style.add_modifier.contains(Modifier::DIM));
        assert!(string_style.add_modifier.contains(Modifier::DIM));
    }

    #[test]
    fn highlights_heredoc_body_as_string() {
        let s = "cat <<EOF\nheredoc body\nEOF";
        let lines = highlight_bash_to_lines(s);
        let body_line = &lines[1];
        let mut body_style = None;
        for span in &body_line.spans {
            if span.content.as_ref() == "heredoc body" {
                body_style = Some(span.style);
            }
        }
        let body_style = body_style.expect("missing heredoc span");
        assert!(body_style.add_modifier.contains(Modifier::DIM));
    }
}

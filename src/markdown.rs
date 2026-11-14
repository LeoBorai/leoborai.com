//! Markdown -> HTML with build-time syntax highlighting.
//!
//! Code blocks are emitted as `<pre class="shiki">` with per-token
//! `--shiki-light` / `--shiki-dark` CSS variables; `app.css` picks one
//! depending on the `.dark` class.

use pulldown_cmark::{CodeBlockKind, Event, Options, Parser, Tag, TagEnd, html};
use syntect::easy::HighlightLines;
use syntect::highlighting::{Color, Theme};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;
use two_face::theme::EmbeddedThemeName;

pub struct Markdown {
    syntaxes: SyntaxSet,
    light: Theme,
    dark: Theme,
}

impl Markdown {
    pub fn new() -> Self {
        let themes = two_face::theme::extra();

        Self {
            syntaxes: two_face::syntax::extra_newlines(),
            light: themes.get(EmbeddedThemeName::Github).clone(),
            dark: themes.get(EmbeddedThemeName::OneHalfDark).clone(),
        }
    }

    pub fn render(&self, source: &str) -> String {
        let options = Options::ENABLE_TABLES
            | Options::ENABLE_STRIKETHROUGH
            | Options::ENABLE_TASKLISTS
            | Options::ENABLE_FOOTNOTES;

        let mut events: Vec<Event> = Vec::new();
        let mut code: Option<(String, String)> = None;

        for event in Parser::new_ext(source, options) {
            match event {
                Event::Start(Tag::CodeBlock(kind)) => {
                    let lang = match kind {
                        CodeBlockKind::Fenced(lang) => lang.to_string(),
                        CodeBlockKind::Indented => String::new(),
                    };
                    code = Some((lang, String::new()));
                }
                Event::End(TagEnd::CodeBlock) => {
                    if let Some((lang, text)) = code.take() {
                        events.push(Event::Html(self.highlight(&text, &lang).into()));
                    }
                }
                Event::Text(text) if code.is_some() => {
                    if let Some((_, buf)) = code.as_mut() {
                        buf.push_str(&text);
                    }
                }
                other => events.push(other),
            }
        }

        let mut out = String::new();
        html::push_html(&mut out, events.into_iter());
        out
    }

    fn highlight(&self, code: &str, lang: &str) -> String {
        let token = match lang.split_whitespace().next().unwrap_or("") {
            "js" => "javascript",
            "ts" => "typescript",
            "yml" => "yaml",
            "svelte" => "html",
            "sh" | "shell" => "bash",
            other => other,
        };
        let syntax = self
            .syntaxes
            .find_syntax_by_token(token)
            .unwrap_or_else(|| self.syntaxes.find_syntax_plain_text());

        let mut light = HighlightLines::new(syntax, &self.light);
        let mut dark = HighlightLines::new(syntax, &self.dark);
        let mut out = String::from("<pre class=\"shiki\" tabindex=\"0\"><code>");

        for line in LinesWithEndings::from(code) {
            let (Ok(l), Ok(d)) = (
                light.highlight_line(line, &self.syntaxes),
                dark.highlight_line(line, &self.syntaxes),
            ) else {
                escape_into(&mut out, line);
                continue;
            };

            // Both themes share the same scopes, so tokens line up one to one.
            for ((ls, text), (ds, _)) in l.iter().zip(d.iter()) {
                if text.trim().is_empty() {
                    escape_into(&mut out, text);
                    continue;
                }

                out.push_str(&format!(
                    "<span style=\"--shiki-light:{};--shiki-dark:{}\">",
                    hex(ls.foreground),
                    hex(ds.foreground)
                ));
                escape_into(&mut out, text);
                out.push_str("</span>");
            }
        }

        out.push_str("</code></pre>\n");
        out
    }
}

fn hex(c: Color) -> String {
    format!("#{:02x}{:02x}{:02x}", c.r, c.g, c.b)
}

fn escape_into(out: &mut String, text: &str) {
    for ch in text.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            _ => out.push(ch),
        }
    }
}

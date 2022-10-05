use anyhow::Result;
use console::{style, Style};
use similar::{ChangeTag, TextDiff};
use std::fmt;
use string_builder::Builder;
use syntect::easy::HighlightLines;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};

struct Line(Option<usize>);

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            None => write!(f, "    "),
            Some(idx) => write!(f, "{:<4}", idx + 1),
        }
    }
}

pub fn text_diff(text1: &str, text2: &str) -> Result<String> {
    let mut output_builder = Builder::default();
    let diff = TextDiff::from_lines(text1, text2);

    for (idx, group) in diff.grouped_ops(3).iter().enumerate() {
        if idx > 0 {
            output_builder.append(format!("{:-^1$}\n", "-", 80));
        }
        for op in group {
            for change in diff.iter_inline_changes(op) {
                let (sign, s) = match change.tag() {
                    ChangeTag::Delete => ("-", Style::new().red()),
                    ChangeTag::Insert => ("+", Style::new().green()),
                    ChangeTag::Equal => (" ", Style::new().dim()),
                };
                output_builder.append(format!(
                    "{}{} |{}",
                    style(Line(change.old_index())).dim(),
                    style(Line(change.new_index())).dim(),
                    s.apply_to(sign).bold(),
                ));
                for (emphasized, value) in change.iter_strings_lossy() {
                    if emphasized {
                        output_builder
                            .append(format!("{}", s.apply_to(value).underlined().on_black()));
                    } else {
                        output_builder.append(format!("{}", s.apply_to(value)));
                    }
                }
                if change.missing_newline() {
                    output_builder.append("\n");
                }
            }
        }
    }
    Ok(output_builder.string()?)
}

pub fn hightlight_text(text: &str, extension: &str) -> Result<String> {
    let mut output = Builder::default();

    // Load these once at the start of your program
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    let syntax = ps
        .find_syntax_by_extension(extension)
        .ok_or_else(|| anyhow::anyhow!("Invalid"))?;
    let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);
    for line in LinesWithEndings::from(text) {
        // LinesWithEndings enables use of newlines mode
        let ranges = h.highlight_line(line, &ps)?;
        let escaped = as_24_bit_terminal_escaped(&ranges[..], false);
        //print!("{}", escaped);
        output.append(escaped);
    }
    Ok(output.string()?)
}

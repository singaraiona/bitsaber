use crate::parse::span::Span;
use crate::result::BSError;
use colored::Colorize;
use std::fmt;

/// Create diagnostic info with BSError to apply it's Span to an input
pub struct Diagnostic<'a> {
    pub name: &'a str,
    pub input: &'a str,
    pub error: BSError,
}

impl<'a> Diagnostic<'a> {
    pub fn new(name: &'a str, input: &'a str, error: BSError) -> Self {
        Self { name, input, error }
    }
}

impl fmt::Display for Diagnostic<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.error {
            BSError::ParseError { msg, desc, span } => match span {
                Some(span) => {
                    format_diagnostic(self.name, self.input, "ParseError", msg, desc, span, f)
                }
                None => write!(f, "{}", msg),
            },
            BSError::CompileError { msg, desc, span } => match span {
                Some(span) => {
                    format_diagnostic(self.name, self.input, "CompileError", msg, desc, span, f)
                }
                None => write!(f, "{}", msg),
            },
            _ => write!(f, "{}", ""),
        }
    }
}

fn format_diagnostic(
    name: &str,
    input: &str,
    tag: &str,
    msg: &str,
    desc: &str,
    span: &Span,
    f: &mut fmt::Formatter<'_>,
) -> fmt::Result {
    let line = &input[span.line_start..span.line_end];
    let lbl_start = span.label_start.wrapping_sub(span.line_start);
    let lbl_end = span.label_end.wrapping_sub(span.line_start);

    // write header
    writeln!(f, " {} {}: {}", "**".bold(), tag.red().bold(), msg.bold())?;

    // write line info
    writeln!(
        f,
        "  {} <{}>:{}:{}",
        "-->".blue().bold(),
        name,
        span.line_number,
        lbl_start
    )?;

    // create description marker
    let marker = format!(
        "{}{}",
        " ".repeat(lbl_start),
        "^".repeat(lbl_end.saturating_sub(lbl_start))
    )
    .red()
    .bold();

    // write line
    write!(
        f,
        "{:<3}{} {}   {} {} {}",
        format!("{}", span.line_number).blue().bold(),
        "|".blue().bold(),
        line,
        "|".blue().bold(),
        marker,
        desc.red().bold(),
    )
}

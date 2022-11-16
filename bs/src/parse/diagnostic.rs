use crate::parse::span::Span;
use crate::result::BSError;
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
    writeln!(f, "** {}: {}", tag, msg)?;

    // write line info
    writeln!(f, "- <{}>:{}:{}", name, span.line_number, lbl_start)?;

    // write line
    writeln!(f, "{}\n{}", line, desc)
}

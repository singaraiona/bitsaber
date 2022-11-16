#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Span {
    pub line_number: usize,
    pub line_start: usize,
    pub line_end: usize,
    pub label_start: usize,
    pub label_end: usize,
}

impl Span {
    pub fn new(
        line_number: usize,
        line_start: usize,
        line_end: usize,
        label_start: usize,
        label_end: usize,
    ) -> Span {
        Span {
            line_number,
            line_start,
            line_end,
            label_start,
            label_end,
        }
    }
}

impl Default for Span {
    fn default() -> Self {
        Span {
            line_number: 1,
            line_start: 0,
            line_end: 0,
            label_start: 0,
            label_end: 0,
        }
    }
}

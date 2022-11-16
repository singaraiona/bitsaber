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

pub mod i64_value;
use std::fmt;

pub struct Value {
    tp: i64,
    val: i64,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ {} {} }}", self.tp, self.val)
    }
}

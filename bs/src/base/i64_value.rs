use std::fmt;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct I64Value(i64);

impl From<i64> for I64Value {
    fn from(val: i64) -> Self {
        Self(val)
    }
}

impl From<I64Value> for i64 {
    fn from(val: I64Value) -> Self {
        val.0
    }
}

impl fmt::Display for I64Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

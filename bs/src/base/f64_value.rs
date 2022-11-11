use std::fmt;

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct F64Value(f64);

impl From<f64> for F64Value {
    fn from(val: f64) -> Self {
        Self(val)
    }
}

impl From<F64Value> for f64 {
    fn from(val: F64Value) -> Self {
        val.0
    }
}

impl fmt::Display for F64Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

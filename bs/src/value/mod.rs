use core::fmt;

#[derive(Debug)]
pub enum Value {
    I64(i64),
    F64(f64),
    VCHR(String),
    VI64(Vec<i64>),
    VF64(Vec<f64>),
    Null,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::I64(v) => write!(f, "{}", v),
            Self::F64(v) => write!(f, "{}", v),
            Self::VI64(v) => write!(f, "{:?}", v),
            Self::VF64(v) => write!(f, "{:?}", v),
            Self::VCHR(v) => write!(f, "{}", v),
            Self::Null => write!(f, "Null"),
        }
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Self::I64(value)
    }
}

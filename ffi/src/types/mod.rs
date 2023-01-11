pub mod fn_type;

use fn_type::FnType;
use std::fmt;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Type {
    Null,
    Bool,
    Int64,
    Float64,
    VecInt64,
    VecFloat64,
    List,
    Fn(FnType),
}

impl TryFrom<&str> for Type {
    type Error = ();
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "Null" => Ok(Type::Null),
            "Bool" => Ok(Type::Bool),
            "Int64" => Ok(Type::Int64),
            "Float64" => Ok(Type::Float64),
            "Int64[]" => Ok(Type::VecInt64),
            "Float64[]" => Ok(Type::VecFloat64),
            "[]" => Ok(Type::List),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Type::Null => write!(f, "Null"),
            Type::Bool => write!(f, "Bool"),
            Type::Int64 => write!(f, "Int64"),
            Type::Float64 => write!(f, "Float64"),
            Type::VecInt64 => write!(f, "Int64[]"),
            Type::VecFloat64 => write!(f, "Float64[]"),
            Type::List => write!(f, "[]"),
            Type::Fn(ref fn_type) => {
                write!(f, "Fn(")?;
                for (i, arg) in fn_type.args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", arg)?;
                }
                write!(f, ") -> {}", fn_type.ret)
            }
        }
    }
}

impl Type {
    pub fn is_scalar(&self) -> bool {
        match self {
            Type::Null | Type::Int64 | Type::Float64 | Type::Bool => true,
            _ => false,
        }
    }
}

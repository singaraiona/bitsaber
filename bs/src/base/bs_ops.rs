use crate::base::infer::infer_type;
use crate::base::Type as BSType;
use crate::result::*;
use llvm::builder::Builder;
use llvm::values::Value;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum Op {
    Add = 0,
    Sub,
    Mul,
    Div,
    Rem,
    Or,
    And,
    Xor,
}

impl TryFrom<&str> for Op {
    type Error = ();

    fn try_from(op: &str) -> Result<Self, Self::Error> {
        match op {
            "+" => Ok(Op::Add),
            "-" => Ok(Op::Sub),
            "*" => Ok(Op::Mul),
            "/" => Ok(Op::Div),
            op => Err(()),
        }
    }
}
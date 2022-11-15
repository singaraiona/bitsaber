#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum Op {
    Add = 0,
    Sub,
    Mul,
    Div,
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

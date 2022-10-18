use core::fmt;
use std::convert::Infallible;
use std::io;
use std::ops::{ControlFlow, FromResidual, Try};

pub enum BSError {
    ParseError { msg: &'static str, pos: usize },
    CompileError(String),
    RuntimeError(String),
    IOError(String),
}

pub fn parse_error<T>(msg: &'static str, pos: usize) -> BSResult<T> {
    BSResult::Err(BSError::ParseError { msg, pos })
}

pub fn compile_error<T>(msg: &str) -> BSResult<T> {
    BSResult::Err(BSError::CompileError(msg.to_string()))
}

pub fn runtime_error<T>(msg: String) -> BSResult<T> {
    BSResult::Err(BSError::RuntimeError(msg.to_string()))
}

pub fn io_error<T>(msg: String) -> BSResult<T> {
    BSResult::Err(BSError::IOError(msg.to_string()))
}

pub fn ok<T>(v: T) -> BSResult<T> {
    BSResult::Ok(v)
}

impl fmt::Display for BSError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ParseError { msg, pos } => {
                write!(f, "** ParseError: {}\n   at: {}", msg, pos)
            }
            Self::CompileError(v) => write!(f, "** CompileError: {}", v),
            Self::RuntimeError(v) => write!(f, "** RuntimeError: {}", v),
            Self::IOError(v) => write!(f, "** IOError: {}", v),
        }
    }
}

pub enum BSResult<T> {
    Ok(T),
    Err(BSError),
}

impl<T> BSResult<T> {
    pub fn ok(v: T) -> Self {
        Self::Ok(v)
    }

    pub fn err(e: BSError) -> Self {
        Self::Err(e)
    }
}

impl<T> fmt::Display for BSResult<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ok(_v) => write!(f, "Ok"),
            Self::Err(e) => write!(f, "{}", e),
        }
    }
}

impl<T> From<Result<T, BSError>> for BSResult<T> {
    fn from(res: Result<T, BSError>) -> Self {
        match res {
            Ok(v) => Self::Ok(v),
            Err(e) => Self::Err(e),
        }
    }
}

impl<T> From<io::Result<T>> for BSResult<T> {
    fn from(res: io::Result<T>) -> Self {
        match res {
            Ok(v) => Self::Ok(v),
            Err(e) => Self::Err(BSError::IOError(e.to_string())),
        }
    }
}

impl<T> Into<Result<T, BSError>> for BSResult<T> {
    fn into(self) -> Result<T, BSError> {
        match self {
            Self::Ok(v) => Ok(v),
            Self::Err(e) => Err(e),
        }
    }
}

impl<T> Into<io::Result<T>> for BSResult<T> {
    fn into(self) -> io::Result<T> {
        match self {
            Self::Ok(v) => Ok(v),
            Self::Err(e) => Err(io::Error::new(io::ErrorKind::Other, format!("{}", e))),
        }
    }
}

impl<T> BSResult<T> {
    pub fn expect(self, msg: &str) -> T {
        match self {
            Self::Ok(t) => t,
            Self::Err(e) => panic!("{msg}: {error}: ", msg = msg, error = e),
        }
    }
}

impl<T> Try for BSResult<T> {
    type Output = T;
    type Residual = BSError;

    fn from_output(output: Self::Output) -> Self {
        BSResult::Ok(output)
    }

    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match self {
            BSResult::Ok(v) => ControlFlow::Continue(v),
            BSResult::Err(e) => ControlFlow::Break(e),
        }
    }
}

impl<T> FromResidual<BSError> for BSResult<T> {
    fn from_residual(residual: BSError) -> Self {
        BSResult::Err(residual)
    }
}

impl<T> FromResidual<Result<Infallible, BSError>> for BSResult<T> {
    fn from_residual(residual: Result<Infallible, BSError>) -> Self {
        match residual {
            Err(e) => BSResult::Err(e),
            _ => unreachable!(),
        }
    }
}

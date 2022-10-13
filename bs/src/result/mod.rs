use core::fmt;
use std::convert::Infallible;
use std::io;
use std::ops::{ControlFlow, FromResidual, Try};

pub enum BSError {
    ParseError(String),
    CompileError(String),
    RuntimeError(String),
    IOError(String),
}

pub fn parse_error(msg: String) -> BSError {
    BSError::ParseError(msg)
}

pub fn compile_error(msg: String) -> BSError {
    BSError::CompileError(msg)
}

pub fn runtime_error(msg: String) -> BSError {
    BSError::RuntimeError(msg)
}

pub fn io_error(msg: String) -> BSError {
    BSError::IOError(msg)
}

impl fmt::Display for BSError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ParseError(v) => write!(f, "ParseError: {}", v),
            Self::CompileError(v) => write!(f, "CompileError: {}", v),
            Self::RuntimeError(v) => write!(f, "RuntimeError: {}", v),
            Self::IOError(v) => write!(f, "IOError: {}", v),
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

// impl<T> Try for BSResult<T> {
//     type Output = (Input<'a>, T);
//     type Residual = (Input<'a>, ErrorKind, ParseError);

//     fn from_output(output: Self::Output) -> Self {
//         Output::Ok(output)
//     }

//     fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
//         match self {
//             Output::Ok(v) => ControlFlow::Continue(v),
//             Output::Err(e) => ControlFlow::Break(e),
//         }
//     }
// }

// impl<'a, T> FromResidual<(Input<'a>, ErrorKind, ParseError)> for Output<'a, T> {
//     fn from_residual(residual: <Output<'a, T> as Try>::Residual) -> Self {
//         Output::Err(residual)
//     }
// }

impl<'a, T> FromResidual<Result<Infallible, &'static str>> for BSResult<T> {
    fn from_residual(residual: Result<Infallible, &'static str>) -> Self {
        match residual {
            Err(e) => BSResult::Err(BSError::ParseError(e.to_string())),
            _ => unreachable!(),
        }
    }
}

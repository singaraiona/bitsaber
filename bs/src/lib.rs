#![feature(try_trait_v2)]

pub mod builtins;
pub mod cc;
pub mod ops;
pub mod parse;
pub mod result;
pub mod rt;

#[macro_use]
extern crate lazy_static;
extern crate colored;
extern crate ffi;
extern crate llvm;
extern crate rand;

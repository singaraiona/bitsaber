#![feature(try_trait_v2)]

pub mod analysis;
pub mod base;
pub mod cc;
pub mod parse;
pub mod result;
pub mod rt;

extern crate colored;
extern crate llvm;
extern crate rand;

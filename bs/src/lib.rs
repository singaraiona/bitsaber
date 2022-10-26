#![feature(try_trait_v2)]

pub extern crate libc;
pub extern crate llvm_sys;

pub mod compile;
pub mod parse;
pub mod rt;
extern crate rand;
pub mod base;
pub mod result;

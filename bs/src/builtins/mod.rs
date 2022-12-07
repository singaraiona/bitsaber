use crate::ffi::external::*;
use crate::ffi::Type as BSType;

#[no_mangle]
pub extern "C" fn load_global() -> i64 {
    println!("IOUTYIYYUYOY");
    999
}

pub(crate) fn init() { register_external("load_global".into(), vec![], BSType::Int64, load_global as i64); }

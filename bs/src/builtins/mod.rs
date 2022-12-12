use crate::ffi::external::*;
use crate::ffi::Type as BSType;
use crate::ffi::Value as BSValue;
use crate::rt::runtime::get_runtime;
use ffi::NULL_VALUE;

#[no_mangle]
pub extern "C" fn test() -> BSValue { BSValue::from(vec![1, 2, 3]) }

#[no_mangle]
pub extern "C" fn dump_module() -> i64 {
    let module = get_runtime().unwrap().get_module("repl").unwrap();
    module.module.dump();
    NULL_VALUE
}

pub(crate) fn init() {
    register_external("test".into(), vec![], BSType::VecInt64, test as i64);
    register_external("dump_module".into(), vec![], BSType::Null, dump_module as i64);
}

use crate::ffi::external::*;
use crate::ffi::types::Type as BSType;
use crate::ffi::values::Value as BSValue;
use crate::rt::runtime::get_runtime;
use ffi::values::NULL_VALUE;

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub extern "C" fn test() -> BSValue { BSValue::from(vec![1, 2, 3]) }

#[no_mangle]
pub extern "C" fn dump_module() -> i64 {
    let module = get_runtime().unwrap().get_module("repl").unwrap();
    module.module.dump();
    NULL_VALUE
}

pub(crate) fn init() {
    register_external("test".into(), vec![], BSType::VecInt64, test as i64);
    register_external("dump_module".into(), vec![], BSType::Int64, dump_module as i64);
}

use crate::ffi::external::*;
use crate::ffi::types::fn_type::FnType;
use crate::ffi::types::Type as BSType;
use crate::ffi::values::Value as BSValue;
use crate::rt::runtime::get_runtime;
use ffi::values::OpaqueValue;

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub extern "C" fn test() -> OpaqueValue { BSValue::from(vec![1, 2, 3]).into() }

#[no_mangle]
pub extern "C" fn dump_module() -> OpaqueValue {
    let module = get_runtime().unwrap().get_module("repl").unwrap();
    module.module.dump();
    BSValue::from(()).into()
}

pub(crate) fn init() {
    register_external("test".into(), FnType::new(vec![], BSType::VecInt64).const_value(test as _));
    register_external("dump_module".into(), FnType::new(vec![], BSType::Null).const_value(dump_module as _));
}

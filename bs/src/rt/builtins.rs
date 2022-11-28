use std::ffi::c_void;

use llvm::llvm_sys::support::LLVMAddSymbol;
use llvm::utils::to_c_str;
use std::mem::transmute;

#[no_mangle]
pub extern "C" fn load_global() -> i64 {
    println!("IOUTYIYYUYOY");
    999
}

// #[used]
// static INTRINSICS: [i64; 1] = [load_global as _];

pub(crate) fn init() {
    unsafe {
        LLVMAddSymbol(
            to_c_str("load_global").as_ptr(),
            transmute(load_global as *const c_void),
        );
    }
}

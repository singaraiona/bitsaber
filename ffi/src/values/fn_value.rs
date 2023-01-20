use crate::types::fn_type::FnType;

#[derive(Debug, Clone)]
pub struct FnValue {
    ty: FnType,
    ptr: *const (),
}

impl FnValue {
    pub fn new(ty: FnType, ptr: *const ()) -> Self { Self { ty, ptr } }

    pub fn get_type(&self) -> &FnType { &self.ty }

    pub fn get_ptr(&self) -> *const () { self.ptr }
}

unsafe impl Send for FnValue {}
unsafe impl Sync for FnValue {}

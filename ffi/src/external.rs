use crate::values::fn_value::FnValue;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    static ref EXTERNAL_FNS: Mutex<HashMap<String, FnValue>> = Mutex::new(HashMap::new());
}

pub fn register_external(name: String, val: FnValue) { EXTERNAL_FNS.lock().unwrap().insert(name, val); }

pub fn with<F, R>(mut f: F) -> R
where
    F: FnMut(&mut HashMap<String, FnValue>) -> R,
{
    let mut map = EXTERNAL_FNS.lock().unwrap();
    f(&mut map)
}

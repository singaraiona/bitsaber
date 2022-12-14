use crate::types::Type as BSType;
use std::collections::HashMap;
use std::sync::Mutex;

pub struct ExternalFn {
    pub args: Vec<BSType>,
    pub ret: BSType,
    pub addr: i64,
}

lazy_static! {
    static ref EXTERNAL_FNS: Mutex<HashMap<String, ExternalFn>> = Mutex::new(HashMap::new());
}

pub fn register_external(name: String, args: Vec<BSType>, ret: BSType, addr: i64) {
    EXTERNAL_FNS
        .lock()
        .unwrap()
        .insert(name, ExternalFn { args, ret, addr });
}

pub fn with<F, R>(mut f: F) -> R
where
    F: FnMut(&mut HashMap<String, ExternalFn>) -> R,
{
    let mut map = EXTERNAL_FNS.lock().unwrap();
    f(&mut map)
}

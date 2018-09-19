use std::collections::HashMap;
use std::sync::{
    Mutex,
    Arc,
};

use super::IdType;
use super::extension::Extension;

lazy_static! {
    // ! Performance could be improved with a different hash function optimized for small integers (See HashMap docs)
    pub static ref EXTENSIONS: Arc<Mutex< HashMap<IdType, Extension> >> = {
        Arc::new(Mutex::new(HashMap::new()))
    };
}
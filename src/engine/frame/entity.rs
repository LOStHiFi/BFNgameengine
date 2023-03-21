use std::sync::Arc;
use crate::engine::component::Component;

pub struct Entity{
    id:u64,
    components:Vec<Arc<Component>>
}

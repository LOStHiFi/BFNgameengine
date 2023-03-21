use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use crate::engine::{entity::Entity,system::System};
use crate::engine::component::Manager;

struct World{
    entities:HashMap<u64,Entity>,
    systems:HashMap<u32,Box<dyn System>>,
    component_managers: HashMap<u32,Box<dyn Any>>
}

impl World {
    fn new()->World{
        World{
            entities:HashMap::new(),
            systems:HashMap::new(),
            component_managers:HashMap::new()
        }
    }
    async fn add_system(&mut self, system: Box<dyn System>){
        self.systems.insert(system.get_id(),system);
    }
    async fn new_component_manager<T:'static>(&mut self,id:u32){
        let m:Manager<T>=Manager::new();
        self.component_managers.insert(id,Box::new(m));
    }
    async fn run(world:Rc<RefCell<World>>){
        loop{
            for(key, value) in &world.borrow_mut().systems{
                value.operate(Context{
                    world: Rc::clone(&world)
                });
            }
        }
    }
}

pub struct Context{
    world:Rc<RefCell<World>>
}
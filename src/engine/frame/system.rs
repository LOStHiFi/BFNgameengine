use crate::engine::world::Context;


pub trait System{
    fn get_id(&self)->u32;
    fn get_running_mode(&self)->RunningMode;
    fn get_running_group(&self)->RunningGroup;
    fn operate(&self,_: Context)->bool;
}


enum RunningMode{
    Logical{gap:u8, need_component:Vec<u32>},
    Rendering,
    //Event{event:Event,priority:u8}
}
enum RunningGroup{
    All,
    Multiple(Vec<u32>),
    Exclude(Vec<u32>)
}
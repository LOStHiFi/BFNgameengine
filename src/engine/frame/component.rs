use std::any::Any;
use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::sync::Arc;


pub struct MetaData {
    pub id: u32,
    pub singleton: bool,
}

pub struct Component {
    meta_data: Arc<MetaData>,
    operate: Arc<dyn ComponentOperate>,
}


pub trait ComponentOperate {
    fn read(&self) -> Box<dyn Any>;
    fn write(&self, _: Box<dyn Any>);
}

pub trait ComponentManager {
    type T;
    fn traversal(&self, operate: fn(usize, &Self::T));
    fn write(&self, index: usize, operate: fn(&Self::T) -> Self::T);
    fn add(&self, data: Self::T);
    fn delete(&self, index: usize);
    fn execute(&mut self);
    fn component_operate(&mut self) -> Rc<dyn ComponentOperate>;
}


pub struct Manager<T> {
    data: Vec<T>,
    write: RefCell<Vec<(usize, T)>>,
    add: RefCell<Vec<T>>,
    delete: RefCell<Vec<usize>>,
}

impl<T> Manager<T> {
    pub(crate) fn new() -> Manager<T> {
        Manager {
            data: vec![],
            write: RefCell::new(vec![]),
            add: RefCell::new(vec![]),
            delete: RefCell::new(vec![]),
        }
    }
}

impl<T> ComponentManager for Manager<T> {
    type T = T;

    fn traversal(&self, operation: fn(usize, &T)) {
        for val in self.data.iter().enumerate() {
            operation(val.0, val.1);
        }
    }

    fn write(&self, index: usize, operate: fn(&T) -> T) {
        let mut write = self.write.borrow_mut();
        let old_data = write.get(index);
        if let Some(data) = old_data {
            let new_data = operate(&data.1);
            write.push((index, new_data));
        }
    }

    fn add(&self, data: T) {
        self.add.borrow_mut().push(data);
    }

    fn delete(&self, index: usize) {
        self.delete.borrow_mut().push(index);
    }

    fn execute(&mut self) {
        let mut l = self.write.borrow().len();
        while l > 0 {
            l -= 1;
            let write_data = self.write.borrow_mut().remove(l);
            let _ = std::mem::replace(&mut self.data[write_data.0], write_data.1);
        }

        if self.add.borrow().is_empty() {
            return;
        }

        self.delete.borrow_mut().drain_filter(|x| {
            if !self.add.borrow().is_empty() {
                let _ = std::mem::replace(&mut self.data[*x], self.add.borrow_mut().remove(0));
                true
            } else {
                false
            }
        });

        if self.add.borrow().is_empty() {
            return;
        }
        l = self.add.borrow().len();
        while l > 0 {
            l -= 1;
            let data = self.add.borrow_mut().remove(0);
            self.data.push(data);
        }
    }

    // fn component_operate(&mut self) -> Rc<dyn ComponentOperate> {
    //
    // }
}

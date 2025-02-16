use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use crate::scanner::LiteralType;


#[derive(Debug, Clone)]
pub struct Environemnt {
    pub values : HashMap<String, LiteralType>,
    pub previous : Option<Rc<RefCell<Environemnt>>>,
}

impl Environemnt {

    pub fn new (parent : Option<Rc<RefCell<Environemnt>>> ) -> Environemnt {
        Environemnt {
            values : HashMap::new(),
            previous : parent
        }
    }

    pub fn define (&mut self, name : String, value : LiteralType) {
        self.values.insert(name, value);
    }

    pub fn get_at (&mut self, distance : i32, name : String) -> Option<LiteralType> {
        if distance == 0 {
            return self.values.get(&name).cloned();
        }
        
        self.previous.as_ref().unwrap().borrow_mut().get_at(distance - 1, name)
    }

    pub fn assign_at (&mut self, distance : i32, name : String, value : LiteralType) {
        if distance == 0 {
            self.values.insert(name, value);
            return
        }
        
        self.previous.as_ref().unwrap().borrow_mut().assign_at(distance - 1, name, value)
    }
}
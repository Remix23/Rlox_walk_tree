use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use crate::scanner::LiteralType;


pub struct Environemnt {
    values : HashMap<String, LiteralType>,
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
    // * Important
    // todo: remember to add error handling
    pub fn get (&mut self, name : String) -> Option<LiteralType> {
        match self.values.get(&name) {
            Some(v) => Some(v.clone()),
            None => {
                match &self.previous {
                    Some(p) => p.borrow_mut().get(name),
                    None => None,
                }
            }
        }
    }

    pub fn assign (&mut self, name : String, value : LiteralType) -> Option<LiteralType> {
        match self.values.insert(name.clone(), value.clone()) {
            Some(v) => Some(v),
            None => {
                match &self.previous {
                    Some(p) => p.borrow_mut().assign(name, value),
                    None => {None}
                }
            }
        }
    }
}
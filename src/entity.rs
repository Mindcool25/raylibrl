use std::{cell::RefCell, rc::Rc};

use crate::cell::Cell;

#[derive(Clone, Debug)]
pub struct Entity {
    pub cell: Option<Rc<RefCell<Cell>>>,
    pub disp: char,
}

impl Entity {
    pub fn new() -> Rc<RefCell<Entity>> {
        let e = Entity {
            cell: None,
            disp: '@',
        };
        return Rc::new(RefCell::new(e));
    }

    pub fn set_cell(&mut self, c: Option<Rc<RefCell<Cell>>>) {
        self.cell = c;
    }
}

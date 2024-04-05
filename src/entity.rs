use std::{borrow::BorrowMut, cell::RefCell, rc::Rc};

use crate::{
    cell::Cell,
    util::{borrow, borrow_cell, mut_cell},
};

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

    pub fn move_up(&mut self) {
        if borrow_cell!(self.reference()).check_up() {
            println!("Can move up");
            mut_cell!(self.reference()).entity = None;
            self.set_cell(borrow_cell!(self.reference()).up.clone());
            mut_cell!(self.reference()).entity = Some(self.reference());
        }
    }
    pub fn move_down(&mut self) {
        if borrow_cell!(self.reference()).check_down() {
            println!("Can move down");
            mut_cell!(self.reference()).entity = None;
            self.set_cell(borrow_cell!(self.reference()).down.clone());
            mut_cell!(self.reference()).entity = Some(self.reference());
        }
    }
    pub fn move_left(&mut self) {
        if borrow_cell!(self.reference()).check_left() {
            println!("Can move left");
            mut_cell!(self.reference()).entity = None;
            self.set_cell(borrow_cell!(self.reference()).left.clone());
            mut_cell!(self.reference()).entity = Some(self.reference());
        }
    }
    pub fn move_right(&mut self) {
        if borrow_cell!(self.reference()).check_right() {
            println!("Can move right");
            mut_cell!(self.reference()).entity = None;
            self.set_cell(borrow_cell!(self.reference()).right.clone());
            mut_cell!(self.reference()).entity = Some(self.reference());
        }
    }

    pub fn reference(&self) -> Rc<RefCell<Entity>> {
        return Rc::new(RefCell::new(self.clone()));
    }
}

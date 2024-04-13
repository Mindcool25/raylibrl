use std::{cell::RefCell, rc::Rc};

use raylib::prelude::*;

use crate::{
    cell::Cell,
    util::{borrow_cell, mut_cell},
};

#[derive(Clone, Debug)]
pub struct Entity {
    pub cell: Option<Rc<RefCell<Cell>>>,
    pub disp: char,
    pub color: Color,
}

impl Entity {
    pub fn new() -> Rc<RefCell<Entity>> {
        let e = Entity {
            cell: None,
            disp: '@',
            color: Color::WHITE,
        };
        return Rc::new(RefCell::new(e));
    }

    pub fn create(disp: char, color: Color) -> Rc<RefCell<Entity>> {
        let e = Entity {
            cell: None,
            disp,
            color,
        };
        return Rc::new(RefCell::new(e));
    }

    pub fn set_cell(&mut self, c: Option<Rc<RefCell<Cell>>>) {
        self.cell = c;
    }

    pub fn move_north(&mut self) {
        if borrow_cell!(self.reference()).check_north() {
            mut_cell!(self.reference()).entity = None;
            self.set_cell(borrow_cell!(self.reference()).north.clone());
            mut_cell!(self.reference()).entity = Some(self.reference());
        }
    }
    pub fn move_south(&mut self) {
        if borrow_cell!(self.reference()).check_south() {
            mut_cell!(self.reference()).entity = None;
            self.set_cell(borrow_cell!(self.reference()).south.clone());
            mut_cell!(self.reference()).entity = Some(self.reference());
        }
    }
    pub fn move_west(&mut self) {
        if borrow_cell!(self.reference()).check_west() {
            mut_cell!(self.reference()).entity = None;
            self.set_cell(borrow_cell!(self.reference()).west.clone());
            mut_cell!(self.reference()).entity = Some(self.reference());
        }
    }
    pub fn move_east(&mut self) {
        if borrow_cell!(self.reference()).check_east() {
            mut_cell!(self.reference()).entity = None;
            self.set_cell(borrow_cell!(self.reference()).east.clone());
            mut_cell!(self.reference()).entity = Some(self.reference());
        }
    }

    pub fn move_ne(&mut self) {
        if borrow_cell!(self.reference()).check_ne() {
            mut_cell!(self.reference()).entity = None;
            self.set_cell(borrow_cell!(self.reference()).ne.clone());
            mut_cell!(self.reference()).entity = Some(self.reference());
        }
    }

    pub fn move_nw(&mut self) {
        if borrow_cell!(self.reference()).check_nw() {
            mut_cell!(self.reference()).entity = None;
            self.set_cell(borrow_cell!(self.reference()).nw.clone());
            mut_cell!(self.reference()).entity = Some(self.reference());
        }
    }

    pub fn move_se(&mut self) {
        if borrow_cell!(self.reference()).check_se() {
            mut_cell!(self.reference()).entity = None;
            self.set_cell(borrow_cell!(self.reference()).se.clone());
            mut_cell!(self.reference()).entity = Some(self.reference());
        }
    }

    pub fn move_sw(&mut self) {
        if borrow_cell!(self.reference()).check_sw() {
            mut_cell!(self.reference()).entity = None;
            self.set_cell(borrow_cell!(self.reference()).sw.clone());
            mut_cell!(self.reference()).entity = Some(self.reference());
        }
    }

    pub fn reference(&self) -> Rc<RefCell<Entity>> {
        return Rc::new(RefCell::new(self.clone()));
    }
}

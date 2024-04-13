use std::{cell::RefCell, rc::Rc};

use raylib::prelude::*;

use crate::{entity::Entity, CHAR_SIZE};

#[derive(Clone, Debug)]
pub struct Cell {
    pub entity: Option<Rc<RefCell<Entity>>>,
    pub disp: char,
    pub pos: Vector2,
    pub north: Option<Rc<RefCell<Cell>>>,
    pub south: Option<Rc<RefCell<Cell>>>,
    pub west: Option<Rc<RefCell<Cell>>>,
    pub east: Option<Rc<RefCell<Cell>>>,
    pub ne: Option<Rc<RefCell<Cell>>>,
    pub nw: Option<Rc<RefCell<Cell>>>,
    pub se: Option<Rc<RefCell<Cell>>>,
    pub sw: Option<Rc<RefCell<Cell>>>,
}

impl Cell {
    pub fn new(disp: char) -> Rc<RefCell<Cell>> {
        let c = Cell {
            entity: None,
            disp,
            pos: Vector2::new(0.0, 0.0),
            north: None,
            south: None,
            west: None,
            east: None,
            ne: None,
            nw: None,
            se: None,
            sw: None,
        };
        Rc::new(RefCell::new(c))
    }

    pub fn check_north(&self) -> bool {
        if self.north.is_some() {
            return self.north.clone().unwrap().as_ref().borrow().available();
        }
        return false;
    }
    pub fn check_south(&self) -> bool {
        if self.south.is_some() {
            return self.south.clone().unwrap().as_ref().borrow().available();
        }
        return false;
    }
    pub fn check_west(&self) -> bool {
        if self.west.is_some() {
            return self.west.clone().unwrap().as_ref().borrow().available();
        }
        return false;
    }
    pub fn check_east(&self) -> bool {
        if self.east.is_some() {
            return self.east.clone().unwrap().as_ref().borrow().available();
        }
        return false;
    }

    pub fn check_ne(&self) -> bool {
        if self.ne.is_some() {
            return self.ne.clone().unwrap().as_ref().borrow().available();
        }
        return false;
    }

    pub fn check_nw(&self) -> bool {
        if self.nw.is_some() {
            return self.nw.clone().unwrap().as_ref().borrow().available();
        }
        return false;
    }

    pub fn check_se(&self) -> bool {
        if self.se.is_some() {
            return self.se.clone().unwrap().as_ref().borrow().available();
        }
        return false;
    }

    pub fn check_sw(&self) -> bool {
        if self.sw.is_some() {
            return self.sw.clone().unwrap().as_ref().borrow().available();
        }
        return false;
    }

    pub fn available(&self) -> bool {
        if self.entity.is_some() {
            return false;
        }
        return true;
    }

    pub fn draw(&self, font: &Font, d: &mut RaylibDrawHandle) {
        if self.entity.is_some() {
            d.draw_text_ex(
                font,
                self.entity
                    .as_ref()
                    .unwrap()
                    .clone()
                    .as_ref()
                    .borrow()
                    .disp
                    .to_string()
                    .as_str(),
                self.pos.scale_by(CHAR_SIZE),
                CHAR_SIZE,
                0.0,
                Color::WHITE,
            );
        } else {
            d.draw_text_ex(
                font,
                self.disp.to_string().as_str(),
                self.pos.scale_by(CHAR_SIZE),
                CHAR_SIZE,
                0.0,
                Color::GRAY,
            );
        }
    }
}

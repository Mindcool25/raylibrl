use std::{cell::RefCell, rc::Rc};

use raylib::prelude::*;

use crate::util::{borrow, borrow_cell};
use crate::{entity::Entity, CHAR_SIZE};

#[derive(Clone, Debug)]
pub struct Cell {
    pub entity: Option<Rc<RefCell<Entity>>>,
    pub disp: char,
    pub pos: Vector2,
    pub up: Option<Rc<RefCell<Cell>>>,
    pub down: Option<Rc<RefCell<Cell>>>,
    pub left: Option<Rc<RefCell<Cell>>>,
    pub right: Option<Rc<RefCell<Cell>>>,
}

impl Cell {
    pub fn new(disp: char) -> Rc<RefCell<Cell>> {
        let c = Cell {
            entity: None,
            disp: disp,
            pos: Vector2::new(0.0, 0.0),
            up: None,
            down: None,
            left: None,
            right: None,
        };
        Rc::new(RefCell::new(c))
    }

    pub fn check_up(&self) -> bool {
        if self.up.is_some() {
            return self.up.clone().unwrap().as_ref().borrow().available();
        }
        return false;
    }
    pub fn check_down(&self) -> bool {
        if self.down.is_some() {
            return self.down.clone().unwrap().as_ref().borrow().available();
        }
        return false;
    }
    pub fn check_left(&self) -> bool {
        if self.left.is_some() {
            return self.left.clone().unwrap().as_ref().borrow().available();
        }
        return false;
    }
    pub fn check_right(&self) -> bool {
        if self.right.is_some() {
            return self.right.clone().unwrap().as_ref().borrow().available();
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
                borrow!(self.entity.clone().unwrap())
                    .disp
                    .to_string()
                    .as_str(),
                self.pos.scale_by(CHAR_SIZE),
                CHAR_SIZE,
                0.0,
                borrow!(self.entity.clone().unwrap()).color,
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

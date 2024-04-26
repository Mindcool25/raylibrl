use std::{cell::RefCell, rc::Rc};

use raylib::prelude::*;

use crate::{
    entity::Entity,
    util::{BorrowObj, Direction, GameObject},
    CHAR_SIZE,
};

#[derive(Clone)]
pub struct Cell {
    pub disp: char,
    pub pos: Vector2,
    pub entity: GameObject<Entity>,

    // Cell neighbors
    pub north: GameObject<Cell>,
    pub south: GameObject<Cell>,
    pub west: GameObject<Cell>,
    pub east: GameObject<Cell>,
    pub ne: GameObject<Cell>,
    pub nw: GameObject<Cell>,
    pub se: GameObject<Cell>,
    pub sw: GameObject<Cell>,
}

impl Cell {
    pub fn new() -> GameObject<Cell> {
        let c = Cell {
            entity: None,
            disp: '#',
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
        return Some(Rc::new(RefCell::new(c)));
    }

    pub fn place(pos: Vector2) -> GameObject<Cell> {
        let c = Cell {
            entity: None,
            disp: '#',
            pos,
            north: None,
            south: None,
            west: None,
            east: None,
            ne: None,
            nw: None,
            se: None,
            sw: None,
        };
        return Some(Rc::new(RefCell::new(c)));
    }

    pub fn draw(&self, font: &Font, d: &mut RaylibDrawHandle) {
        if self.entity.is_some() {
            println!("entity");
            d.draw_text_ex(
                font,
                self.entity.borrow_ref().disp.to_string().as_str(),
                self.pos.scale_by(CHAR_SIZE),
                CHAR_SIZE,
                0.0,
                Color::RED,
            );
        } else {
            d.draw_text_ex(
                font,
                self.disp.to_string().as_str(),
                self.pos.scale_by(CHAR_SIZE),
                CHAR_SIZE,
                0.0,
                Color::WHITE,
            );
        }
    }

    pub fn is_available(&self) -> bool {
        return self.entity.is_none();
    }

    pub fn get_dir(&self, dir: Direction) -> GameObject<Cell> {
        match dir {
            Direction::North => self.north.clone(),
            Direction::East => self.east.clone(),
            Direction::South => self.south.clone(),
            Direction::West => self.west.clone(),
            Direction::NE => self.ne.clone(),
            Direction::SE => self.se.clone(),
            Direction::SW => self.sw.clone(),
            Direction::NW => self.nw.clone(),
        }
    }

    pub fn check_dir(&self, dir: Direction) -> bool {
        match dir {
            Direction::North => {
                if self.north.is_some() {
                    return self.north.borrow_ref().is_available();
                } else {
                    return false;
                }
            }
            Direction::East => {
                if self.east.is_some() {
                    return self.east.borrow_ref().is_available();
                } else {
                    return false;
                }
            }
            Direction::South => {
                if self.south.is_some() {
                    return self.south.borrow_ref().is_available();
                } else {
                    return false;
                }
            }
            Direction::West => {
                if self.west.is_some() {
                    return self.west.borrow_ref().is_available();
                } else {
                    return false;
                }
            }
            Direction::NE => {
                if self.ne.is_some() {
                    return self.ne.borrow_ref().is_available();
                } else {
                    return false;
                }
            }
            Direction::SE => {
                if self.se.is_some() {
                    return self.se.borrow_ref().is_available();
                } else {
                    return false;
                }
            }
            Direction::SW => {
                if self.sw.is_some() {
                    return self.sw.borrow_ref().is_available();
                } else {
                    return false;
                }
            }
            Direction::NW => {
                if self.nw.is_some() {
                    return self.nw.borrow_ref().is_available();
                } else {
                    return false;
                }
            }
        }
    }
}

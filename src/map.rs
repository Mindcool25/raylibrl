use raylib::prelude::*;

use crate::util::{borrow, mut_ref};
use crate::{cell::Cell, CELL_AMOUNT};
use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
pub struct Map {
    pub map: Vec<Rc<RefCell<Cell>>>,
}

impl Map {
    pub fn new() -> Map {
        let mut new_map: Map = Map { map: Vec::new() };

        let mut curr_pos = Vector2::new(15.0, 15.0);

        for _ in 0..CELL_AMOUNT {
            let new_pos: Vector2 = match get_random_value(0, 3) {
                0 => {
                    if new_map.is_occupied(Vector2::new(curr_pos.x + 1.0, curr_pos.y)) {
                        curr_pos
                    } else {
                        Vector2::new(curr_pos.x + 1.0, curr_pos.y)
                    }
                }
                1 => {
                    if new_map.is_occupied(Vector2::new(curr_pos.x - 1.0, curr_pos.y)) {
                        curr_pos
                    } else {
                        Vector2::new(curr_pos.x - 1.0, curr_pos.y)
                    }
                }
                2 => {
                    if new_map.is_occupied(Vector2::new(curr_pos.x, curr_pos.y + 1.0)) {
                        curr_pos
                    } else {
                        Vector2::new(curr_pos.x, curr_pos.y - 1.0)
                    }
                }
                3 => {
                    if new_map.is_occupied(Vector2::new(curr_pos.x, curr_pos.y + 1.0)) {
                        curr_pos
                    } else {
                        Vector2::new(curr_pos.x, curr_pos.y + 1.0)
                    }
                }
                _ => curr_pos,
            };
            if new_pos != curr_pos {
                if !new_map.is_occupied(new_pos) {
                    let new_cell = Cell::new('#');
                    mut_ref!(new_cell).pos = new_pos;
                    new_map.map.push(new_cell);
                    curr_pos = new_pos;
                }
            }
        }
        println!("Map?? : {:?}", new_map);
        new_map
    }

    pub fn init_cells(&self) {
        for cell in &self.map {
            let pos = borrow!(cell).pos;
            mut_ref!(cell).north = self.get_cell(Vector2::new(pos.x, pos.y - 1.0));
            mut_ref!(cell).south = self.get_cell(Vector2::new(pos.x, pos.y + 1.0));
            mut_ref!(cell).east = self.get_cell(Vector2::new(pos.x + 1.0, pos.y));
            mut_ref!(cell).west = self.get_cell(Vector2::new(pos.x - 1.0, pos.y));
            mut_ref!(cell).ne = self.get_cell(Vector2::new(pos.x + 1.0, pos.y - 1.0));
            mut_ref!(cell).nw = self.get_cell(Vector2::new(pos.x - 1.0, pos.y - 1.0));
            mut_ref!(cell).se = self.get_cell(Vector2::new(pos.x + 1.0, pos.y + 1.0));
            mut_ref!(cell).sw = self.get_cell(Vector2::new(pos.x - 1.0, pos.y + 1.0));
        }
    }

    pub fn draw(&self, font: &Font, d: &mut RaylibDrawHandle) {
        for cell in &self.map {
            borrow!(cell).draw(font, d);
        }
    }

    pub fn is_occupied(&self, pos: Vector2) -> bool {
        if self.get_cell(pos).is_some() {
            return true;
        }
        return false;
    }

    pub fn get_cell(&self, pos: Vector2) -> Option<Rc<RefCell<Cell>>> {
        for cell in &self.map {
            if borrow!(cell).pos == pos {
                return Some(cell.clone());
            }
        }
        return None;
    }
}

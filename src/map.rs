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

        let mut curr_pos = Vector2::new(30.0, 30.0);

        for _ in 0..CELL_AMOUNT {
            let c = Cell::new('#');
            mut_ref!(c).pos = match get_random_value(0, 3) {
                0 => {
                    if new_map.is_occupied(Vector2::new(curr_pos.x + 1.0, curr_pos.y)) {
                        continue;
                    } else {
                        curr_pos.x += 1.0;
                        curr_pos
                    }
                }
                1 => {
                    if new_map.is_occupied(Vector2::new(curr_pos.x - 1.0, curr_pos.y)) {
                        continue;
                    } else {
                        curr_pos.x -= 1.0;
                        curr_pos
                    }
                }
                2 => {
                    if new_map.is_occupied(Vector2::new(curr_pos.x, curr_pos.y + 1.0)) {
                        continue;
                    } else {
                        curr_pos.y += 1.0;
                        curr_pos
                    }
                }
                3 => {
                    if new_map.is_occupied(Vector2::new(curr_pos.x, curr_pos.y + 1.0)) {
                        continue;
                    } else {
                        curr_pos.y -= 1.0;
                        curr_pos
                    }
                }
                _ => curr_pos,
            };
            new_map.map.push(c);
        }
        println!("Map?? : {:?}", new_map);
        new_map
    }

    pub fn init_cells(&self) {
        for cell in &self.map {
            let pos = borrow!(cell).pos;
            mut_ref!(cell).up = self.get_cell(Vector2::new(pos.x, pos.y - 1.0));
            mut_ref!(cell).down = self.get_cell(Vector2::new(pos.x, pos.y + 1.0));
            mut_ref!(cell).left = self.get_cell(Vector2::new(pos.x - 1.0, pos.y));
            mut_ref!(cell).right = self.get_cell(Vector2::new(pos.x + 1.0, pos.y));
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

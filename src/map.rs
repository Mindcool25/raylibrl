use raylib::prelude::*;

use crate::{
    cell::Cell,
    entity::Entity,
    util::{BorrowObj, GameObject},
    CELL_AMOUNT,
};

pub struct Map {
    pub map: Vec<GameObject<Cell>>,
}

impl Map {
    pub fn new() -> Map {
        let mut new_map = Map { map: Vec::new() };

        let mut curr_pos = Vector2::new(10.0, 10.0);

        for _ in 0..CELL_AMOUNT {
            let new_pos: Vector2 = match get_random_value(0, 3) {
                0 => Vector2::new(curr_pos.x + 1.0, curr_pos.y),
                1 => Vector2::new(curr_pos.x - 1.0, curr_pos.y),
                2 => Vector2::new(curr_pos.x, curr_pos.y - 1.0),
                3 => Vector2::new(curr_pos.x, curr_pos.y + 1.0),
                _ => curr_pos,
            };
            println!("{:?}", new_pos);
            if !new_map.is_cell(new_pos) {
                let new_cell = Cell::place(new_pos);
                new_map.map.push(new_cell);
                curr_pos = new_pos;
            }
        }
        new_map
    }

    pub fn is_cell(&self, pos: Vector2) -> bool {
        if self.get_cell(pos).is_some() {
            return true;
        }
        return false;
    }

    pub fn get_cell(&self, pos: Vector2) -> GameObject<Cell> {
        for cell in &self.map {
            if cell.borrow_ref().pos == pos {
                return Some(cell.clone()?);
            }
        }
        return None;
    }

    pub fn set_entity(&mut self, index: usize, entity: GameObject<Entity>) {
        self.map[index].clone().mut_ref().borrow_mut().entity = entity;
    }

    pub fn draw(&self, font: &Font, d: &mut RaylibDrawHandle) {
        for cell in &self.map {
            cell.borrow_ref().draw(font, d);
        }
    }
}

use std::{cell::RefCell, rc::Rc};

use crate::{
    cell::Cell,
    util::{BorrowObj, Direction, GameObject},
};

pub struct EntityHolder {
    pub entity: GameObject<Entity>,
}

#[derive(Clone)]
pub struct Entity {
    pub disp: char,
    pub cell: GameObject<Cell>,
}

impl Entity {
    pub fn new() -> GameObject<Entity> {
        let e = Entity {
            disp: '@',
            cell: None,
        };
        return Some(Rc::new(RefCell::new(e)));
    }

    pub fn move_dir(&mut self, dir: Direction) {
        if self.cell.borrow_ref().check_dir(dir.clone()) {
            let new_cell = self.cell.borrow_ref().get_dir(dir.clone());
            self.cell.clone().mut_ref().borrow_mut().entity = None;
            new_cell.clone().mut_ref().borrow_mut().entity = self.self_ref();
            self.cell = new_cell;
        }
    }

    pub fn self_ref(&self) -> GameObject<Entity> {
        return Some(Rc::new(RefCell::new(self.clone())));
    }
}

impl EntityHolder {
    fn new() -> EntityHolder {
        EntityHolder {
            entity: Entity::new(),
        }
    }

    fn set_cell(&self, cell: GameObject<Cell>) {
        self.entity.as_ref().unwrap().as_ref().borrow_mut().cell = cell;
    }
}

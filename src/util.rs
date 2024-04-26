use crate::cell::Cell;
use crate::entity::Entity;
use std::{cell::RefCell, rc::Rc};

macro_rules! mut_ref {
    ($a:expr) => {
        $a.as_ref().borrow_mut()
    };
}

macro_rules! borrow {
    ($a:expr) => {
        $a.as_ref().borrow()
    };
}

macro_rules! borrow_cell {
    ($a:expr) => {
        $a.as_ref().borrow().cell.clone().unwrap().as_ref().borrow()
    };
}

macro_rules! mut_cell {
    ($a:expr) => {
        $a.as_ref()
            .borrow_mut()
            .cell
            .clone()
            .unwrap()
            .as_ref()
            .borrow_mut()
    };
}
pub(crate) use {borrow, borrow_cell, mut_cell, mut_ref};

pub trait BorrowObj<T> {
    fn borrow_ref(self) -> T;
    fn mut_ref(self) -> T;
}

pub type GameObject<T> = Option<Rc<RefCell<T>>>;

impl BorrowCell for CellRef {
    fn borrow_ref(self) -> Cell {
        self.clone().unwrap().as_ref().borrow().clone()
    }

    fn mut_ref(self) -> Cell {
        self.clone().unwrap().as_ref().borrow_mut().clone()
    }
}

use std::{
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
};

#[derive(Clone, Copy)]
pub enum Direction {
    North,
    NE,
    East,
    SE,
    South,
    SW,
    West,
    NW,
}

pub trait BorrowObj<T>: Clone {
    fn borrow_ref(&self) -> T;
    fn mut_ref(self) -> RefCell<T>;
}

pub type GameObject<T> = Option<Rc<RefCell<T>>>;

impl<T: Clone> BorrowObj<T> for GameObject<T> {
    fn borrow_ref(&self) -> T {
        self.clone().unwrap().as_ref().borrow().clone()
    }
    fn mut_ref(self) -> RefCell<T> {
        self.clone().unwrap().as_ref().clone()
    }
}

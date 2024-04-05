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

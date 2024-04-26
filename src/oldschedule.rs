use crate::entity::Entity;
use crate::util::{BorrowEntity, EntityRef};

#[derive(Debug, Clone)]
pub struct Event {
    pub owner: EntityRef,
    pub tick: i32,
}

#[derive(Debug, Clone)]
pub struct Schedule {
    pub events: Vec<Event>,
    pub tick: i32,
}

impl Schedule {
    pub fn new() -> Self {
        Schedule {
            events: Vec::new(),
            tick: 0,
        }
    }

    pub fn next_tick(&mut self) {
        println!("GOOO");
        self.events.sort_by(|a, b| b.tick.cmp(&a.tick));
        let curr_event = self.events.pop().unwrap();
        curr_event.owner.mut_ref().move_east();
        self.tick = curr_event.tick;
        return;
    }
}

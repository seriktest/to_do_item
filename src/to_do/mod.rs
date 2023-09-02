use crate::to_do::enums::TaskStatus;
use crate::to_do::structs::done::Done;
use crate::to_do::structs::pending::Pending;

pub mod enums;
pub mod structs;
pub mod traits;

pub enum ItemTypes {
    Pending(Pending),
    Done(Done)
}

pub fn to_do_factory(title: &str, status :TaskStatus) -> ItemTypes {
    match status {
        TaskStatus::DONE => {
            ItemTypes::Done(Done::new(title))
        },
        TaskStatus::PENDING => {
            ItemTypes::Pending(Pending::new(title))
        }
    }
}
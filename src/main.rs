use crate::to_do::enums::TaskStatus;
use crate::to_do::{ItemTypes, to_do_factory};

use crate::to_do::traits::get::Get;
use crate::to_do::traits::delete::Delete;
use crate::to_do::traits::edit::Edit;

mod to_do;


fn main() {
    let to_do_item = to_do_factory("washing", TaskStatus::DONE);
    match to_do_item {
        ItemTypes::Pending(item) => {
            item.get(&item.super_struct.title);
            item.set_to_done(&item.super_struct.title);
        }
        ItemTypes::Done(item) => {
            item.get(&item.super_struct.title);
            item.delete(&item.super_struct.title);
        }
    }
}

use crate::to_do::enums::TaskStatus;
use crate::to_do::structs::base::Base;
use crate::to_do::traits::delete::Delete;
use crate::to_do::traits::edit::Edit;
use crate::to_do::traits::get::Get;

pub struct Done {
    pub super_struct: Base
}

impl Done {
    pub fn new(input_title: &str) -> Self {
        let base = Base {
            title: input_title.to_string(),
            status: TaskStatus::DONE
        };
        Done{super_struct: base}
    }
}

impl Get  for Done {

}
impl Delete for Done {

}

impl Edit for Done {

}
use serde_json::{Value, Map};

pub trait Get {
    fn get(&self, title: &str, state: &mut Map<String, Value>) {
        let item = state.get(title);
        match item {
            Some(result) =>{
                println!("\n\nItem: {}", title);
                println!("Status: {}\n\n", result);
            },
            None => println!("item,: {} was not found", title)
        }
    }
}
use crate::todo_list::todo_list_struct::*;
use std::io::prelude::*;
use std::{fs::File, io::Read};

pub fn save_todo_list(todo_list: &TodoList) {
    let mut file = File::create("db.json").unwrap();
    let json_str: String = serde_json::to_string(&todo_list).unwrap();
    file.write_all(json_str.as_bytes()).unwrap();
}

pub fn read_todo_list() -> TodoList {
    let mut file = File::open("db.json").unwrap();

    // Read the contents of the file into a string
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let v: TodoList = serde_json::from_str(&*contents).unwrap();
    return v;
}

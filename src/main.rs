mod db_file;
mod todo_list;
mod utils;

use crate::db_file::read_n_write::*;
use crate::utils::terminal_utils::clear_terminal;
use std::io;

fn main() {
    let mut todo_list = read_todo_list();

    clear_terminal();

    println!("Welcome to M.R.Z.G todo list!");

    loop {
        println!("Please please select menu item.");

        println!("1. Add new item");
        println!("2. Show all items");
        println!("Enter ':q' to quit without save.");
        println!("Enter ':wq' to quit and save.");

        let mut user_input = String::new();
        match io::stdin().read_line(&mut user_input) {
            Ok(_) => {}
            Err(err) => {
                eprintln!("Something went wrong - {}", err);
            }
        };

        let operation_code = user_input.trim().to_lowercase();

        match operation_code.as_str() {
            "1" => {
                clear_terminal();
                todo_list.add_new_todo_item();
                clear_terminal();
                continue;
            }
            "2" => {
                clear_terminal();
                todo_list.show_todo_item_options();
                clear_terminal();
                continue;
            }
            ":q" => {
                clear_terminal();
                println!("Quitting...");
                clear_terminal();
                break;
            }
            ":wq" => {
                save_todo_list(&todo_list);
                println!("Quitting...");
                clear_terminal();
                break;
            }
            _ => {
                clear_terminal();
                eprintln!("Invalid option: {}", operation_code);
                clear_terminal();
                continue;
            }
        };
    }
}

use crate::utils::terminal_utils::clear_terminal;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt::Display;
use std::io;

pub trait ToggleCheck {
    fn toggle_check(&mut self);
}

pub type TodoID = u32;
#[derive(Debug, Serialize, Deserialize)]

pub struct Todo {
    title: String,
    is_checked: bool,
}

impl Todo {
    pub fn new(title: String) -> Todo {
        Todo {
            title,
            is_checked: false,
        }
    }
}

impl ToggleCheck for Todo {
    fn toggle_check(&mut self) {
        self.is_checked = !self.is_checked;
    }
}

impl Display for Todo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} is {} done!",
            self.title,
            if self.is_checked { "" } else { "not" }
        )
    }
}

#[derive(Serialize, Deserialize)]
pub struct TodoList {
    pub title: String,
    pub items: BTreeMap<TodoID, Todo>,
}

impl TodoList {
    pub fn show_all_items(&self) {
        println!("{self}");
    }

    pub fn add_new_todo_item(&mut self) {
        loop {
            clear_terminal();
            self.show_all_items();
            println!("Please, enter the task's title");
            println!("Or, type ':b' to return to the menu!");
            let mut user_input = String::new();
            match io::stdin().read_line(&mut user_input) {
                Ok(_) => {}
                Err(err) => {
                    eprintln!("Something went wrong - {}", err);
                }
            };

            let cleanup_input = user_input.trim();
            if cleanup_input == ":b" {
                break;
            } else if cleanup_input == "" {
                println!("Please, enter a valid title");
                continue;
            } else {
                let new_todo = Todo::new(cleanup_input.to_string());
                let new_todo_id: TodoID = self.items.len() as u32 + 1;
                self.items.insert(new_todo_id, new_todo);
                continue;
            }
        }
    }

    pub fn show_todo_item_options(&mut self) {
        loop {
            println!("Please, select item # to toggle it's check!");
            println!("Or, type ':b' to return to the menu!");

            self.show_all_items();

            let mut user_input = String::new();
            match io::stdin().read_line(&mut user_input) {
                Ok(_) => {}
                Err(err) => {
                    eprintln!("Something went wrong - {}", err);
                }
            };
            let selected_number = user_input.trim().to_lowercase();

            if selected_number == ":b" {
                clear_terminal();
                break;
            }

            let selected_number: usize = match selected_number.parse() {
                Ok(v) => v,
                Err(err) => {
                    clear_terminal();
                    println!("Invalid #, {err}");
                    continue;
                }
            };

            let selected_item = match self.items.get_mut(&(selected_number as u32)) {
                Some(v) => v,
                None => {
                    clear_terminal();
                    println!("Invalid #");
                    continue;
                }
            };

            selected_item.toggle_check();
            self.show_all_items();
            clear_terminal();
        }
        clear_terminal();
    }
}

impl Display for TodoList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut print_statement = String::new();
        for (k, item) in self.items.iter() {
            let formatted_string = format!(
                "{}. {:<20}{} \n",
                k,
                item.title,
                if item.is_checked { "[*]" } else { "[ ]" }
            );

            print_statement = print_statement + &formatted_string;
        }

        write!(f, "Todo list items {}:\n{}", self.title, print_statement)
    }
}

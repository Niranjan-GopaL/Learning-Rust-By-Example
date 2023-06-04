use std::io::{self, Write};

struct TodoItem {
    task: String,
}

struct TodoList {
    items: Vec<TodoItem>,
}

impl TodoList {
    fn new() -> TodoList {
        TodoList { items: Vec::new() }
    }

    fn add_item(&mut self, task: String) {
        let item = TodoItem { task };
        self.items.push(item);
    }

    fn remove_item(&mut self, index: usize) {
        if index < self.items.len() {
            self.items.remove(index);
        } else {
            println!("Invalid index");
        }
    }

    fn display_items(&self) {
        if self.items.is_empty() {
            println!("No items in the to-do list");
        } else {
            for (index, item) in self.items.iter().enumerate() {
                println!("{}. {}", index + 1, item.task);
            }
        }
    }
}

fn main() {
    let mut todo_list = TodoList::new();

    loop {
        print!("Enter a command (add/remove/display/quit): ");
        io::stdout().flush().unwrap();

        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();
        let command = command.trim().to_lowercase();

        match command.as_str() {
            "add" => {
                print!("Enter a task: ");
                io::stdout().flush().unwrap();

                let mut task = String::new();
                io::stdin().read_line(&mut task).unwrap();
                let task = task.trim().to_string();

                todo_list.add_item(task);
                println!("Task added to the to-do list");
            }
            "remove" => {
                print!("Enter the index of the task to remove: ");
                io::stdout().flush().unwrap();

                let mut index = String::new();
                io::stdin().read_line(&mut index).unwrap();
                let index = index.trim().parse::<usize>().unwrap();

                todo_list.remove_item(index - 1);
            }
            "display" => {
                todo_list.display_items();
            }
            "quit" => break,
            _ => println!("Invalid command"),
        }
    }
}

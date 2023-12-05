#![allow(dead_code)]

enum Command {
    Todo(String),
    Done(usize),
    Purge,
    Quit,
}

impl Command {
    fn prompt() -> Self {
        loop {
            let line = ftkit::read_line();
    
            if line.is_empty() || line.trim() == "QUIT" {
                break Self::Quit;
            }
            else if let Some(task) = line.strip_prefix("TODO ") {
                break Self::Todo(task.trim().to_string());
            }
            else if let Some(index) = line.strip_prefix("DONE ") {
                if let Ok(index) = index.trim().parse() {
                    break Self::Done(index);
                }
            } 
            else if line.trim() == "PURGE" {
                break Self::Purge;
            }
        }
    }
}

struct TodoList {
    todos: Vec<String>,
    dones: Vec<String>,
}

impl TodoList {
    fn new() -> Self {
        Self {
            todos: Vec::new(),
            dones: Vec::new(),
        }
    } // optional

    fn display(&self) {
        println!("\n\nTodo:");
        for (index, todo) in self.todos.iter().enumerate() {
            println!("  {} [ ] {}", index, todo);
        }
        if self.dones.is_empty() {
            println!("Done: (none)");
            return;
        }
        println!("Done:");
        for (_index, done) in self.dones.iter().enumerate() {
            println!("  [x] {}", done);
        }
    }
    fn add(&mut self, todo: String) {
        self.todos.push(todo);
    }
    fn done(&mut self, index: usize) {
        if index < self.todos.len() {
            let task = self.todos.remove(index);
            self.dones.push(task);
        }
    }
    fn purge(&mut self) {
        self.dones.clear();
    }
}

fn main() {
    let mut todo_list = TodoList::new();
    loop {
        todo_list.display();
        match Command::prompt() {
            Command::Todo(task) => todo_list.add(task),
            Command::Done(index) => todo_list.done(index),
            Command::Purge => todo_list.purge(),
            Command::Quit => break,
        }
    }
}

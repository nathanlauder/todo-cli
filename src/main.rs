#![allow(non_camel_case_types)]
#![allow(dead_code)]

// 1:35:00 talking about enums

use std::env;

struct TodoItem {
  name: String,
  complete: char
}

impl TodoItem {
  fn new(name: String) -> TodoItem {
    return TodoItem{
      name: name, 
      complete: ' '
    };
  }
}

struct TodoList {
  list: Vec<TodoItem>
}

impl TodoList {
  fn new() -> TodoList {
    return TodoList{
      list: Vec::new()
    };
  }

  fn add(&mut self, name: String) {
    self.list.push(TodoItem::new(name))
  }

  fn print(&self) {
    for item in &self.list {
      println!("[{}] - {}", item.complete, item.name);
    }
  }
}

// enum Command {
//   get: "get"
// }

fn main() {
  let arguments: Vec<String> = env::args().collect();
  let command = arguments[1].clone();
  let todoItem = "Hey Nathan".to_string();
  let todoItem2 = "Hey Nathan2".to_string();
  let mut todoList = TodoList::new();
  todoList.add(todoItem);
  todoList.add(todoItem2);
  // println!("{:?}", todoItem);
  match command {
    "get".to => todoList.print(),
    "add" => {
      let task = arguments[2].clone();
      todoList.add(task);
      todoList.print();
    }
  }
  if command == "get" {
    todoList.print();
  }
  else if command == "add" {
    let task = arguments[2].clone();
    todoList.add(task);
    todoList.print();
  }
}

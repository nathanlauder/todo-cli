#![allow(non_snake_case)]

// 2:03 talking about marking un/done

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
    for (index, item) in self.list.iter().enumerate() {
      println!("{} [{}] - {}", index, item.complete, item.name);
    }
  }

  fn markDone (&mut self, index: usize) {
    if self.list[index].complete == ' '{
      self.list[index].complete = 'x';
    }
    else {
      self.list[index].complete = ' ';
    }
  }

}

enum Command {
  Get,
  Add(String),
  Done(usize)
}

fn main() {
  let arguments: Vec<String> = env::args().collect();
  let command = match arguments[1].as_str() {
    "get" => Command::Get,
    "add" => Command::Add(arguments[2].clone()),
    "done" => Command::Done(arguments[2].parse().expect("error converting to u32")),
    _ => panic!("Provide proper command")
  };
  let todoItem = "Hey Nathan".to_string();
  let todoItem2 = "Hey Nathan2".to_string();
  let mut todoList = TodoList::new();
  todoList.add(todoItem);
  todoList.add(todoItem2);
  // println!("{:?}", todoItem);
  match command {
    Command::Get => todoList.print(),
    Command::Add(task) => {
      todoList.add(task);
      todoList.print();
    },
    Command::Done(index) => {
      todoList.markDone(index);
      todoList.print();
    }
  }
  // if command == "get" {
  //   todoList.print();
  // }
  // else if command == "add" {
  //   let task = arguments[2].clone();
  //   todoList.add(task);
  //   todoList.print();
  // }
}

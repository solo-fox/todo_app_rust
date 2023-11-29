#[allow(unused_variables)]
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use crate::structions::todo_structions::Todo;
use crate::structions::todo_structions::TodoTrait;

pub mod read_todo {
  pub struct TodoReader;
  use super::*;

  pub trait TodoReaderTrait {
    fn new() -> TodoReader;
    fn get_todos(&self) -> HashMap<String,Todo>;
  }

  impl TodoReaderTrait for TodoReader {
    fn new() -> TodoReader {
      TodoReader {}
    }

    fn get_todos(&self) -> HashMap<String,Todo> {
      let mut todo_haspmap : HashMap<String,Todo> = HashMap::new();
      let mut contents: String = String::new();

      let _file = match File::open("todo.txt") {
        Ok(mut file) => {
          file.read_to_string(&mut contents).expect("Cannot read data!");
          file
        },
        Err(_) => {
          println!("There is not any Todos registred yet!");
          return todo_haspmap;
        }
      };

      let sperated_todos: Vec<&str> = contents.split(|c| c == '&').collect();
      for index in 0..sperated_todos.len() {
        let single_todo : Vec<&str> = sperated_todos[index].split(",").collect();
        for index in 0..single_todo.len(){
          if single_todo[index].to_string().contains("\n"){
            continue;
          }
          let mut todo_struct : Todo = Todo::new();
          todo_struct.name = single_todo[0].to_string();
          todo_struct.done = single_todo[1].to_string();
          todo_haspmap.insert(index.to_string(),todo_struct);
        }
      }
      return todo_haspmap
    }

  }
}
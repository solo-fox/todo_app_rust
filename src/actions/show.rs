use crate::gfs::read_todo:: {
  TodoReader
};
use crate::gfs::read_todo:: {
  TodoReaderTrait
};
use std::collections::HashMap;
use crate::structions::todo_structions::Todo;
use colored::*;

pub fn show_todos () {
  let todo_reader: TodoReader = TodoReader::new();
  let readed_todos: HashMap<String,
  Todo> = todo_reader.get_todos();
  println!("Name\t\t\tMarked as");
  for (_, value) in readed_todos {
    if value.done.trim() == "false" {
      let text = format!("{}\t\t\t{}", value.name, value.done);
      println!("{}" , text.red());
    }else{
      let text = format!("{}\t\t\t{}", value.name, value.done);
      println!("{}" , text.green().strikethrough());
    }
  }
}
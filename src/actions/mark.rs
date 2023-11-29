use crate::gfs::read_todo:: {
  TodoReader
};
use crate::gfs::read_todo:: {
  TodoReaderTrait
};
use crate::structions::todo_structions::Todo;
use crate::structions::todo_structions::TodoTrait;
use crate::gfs::write_todo:: {
  TodoWriter
};
use std::collections::HashMap;
use std::io;
use std::fs;

pub fn mark() {
  let todo_reader: TodoReader = TodoReader::new();
  let mut existing_todos: HashMap<String,
  Todo> = todo_reader.get_todos();
  let mut user_input: String = String::new();

  println!("Todo to mark: ");
  loop {
    match io::stdin().read_line(&mut user_input) {
      Err(e) => panic!("{}", e),
      Ok(_) if user_input.trim() != "" => break,
      _ => {}
    }
  }
  user_input = user_input.replace("\n", "");

  let keys_to_remove: Vec<String> = existing_todos
  .iter()
  .filter(|(_, value)| value.name.trim() == user_input)
  .map(|(key, _)| key.clone())
  .collect();

  for key in keys_to_remove {
    if let Some(todo) = existing_todos.remove(&key) {
      let mut modified_todo = todo.clone();
      modified_todo.done = "true".to_string();
      modified_todo.name = todo.name.to_string();
      let _ = &existing_todos.insert(key, modified_todo);
    } else {
      panic!("Internal Error please contact me!");
    }
  }
  if let Err(e) = fs::remove_file("todo.txt") {
    eprintln!("Error removing file: {}", e)
  }
  for (_, value) in existing_todos {
    let todo_writer = TodoWriter::new();

    let todo_data_string: String = value.to_saveable_string();

    todo_writer.create_todo(todo_data_string.as_str());
  }
  println!("Marked {} as completed!" , &user_input);
}
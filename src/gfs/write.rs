use std::fs:: {
  File,
  OpenOptions
};
use std::io::prelude::*;

pub mod write_todo {
  pub struct TodoWriter;
  use super::*;
  
  impl TodoWriter {
    pub fn new() -> TodoWriter {
      TodoWriter {}
    }

    fn create_todo_file(&self) -> std::io::Result<File> {
      File::create("todo.txt")
    }

    fn append_todo_file(&self) -> std::io::Result<File> {
      OpenOptions::new()
      .append(true)
      .open("todo.txt")
    }

    pub fn create_todo(&self, todo_text: &str) {
      if let Ok(mut data_file) = self.append_todo_file() {
        if let Err(e) = writeln!(data_file, "{}", todo_text.trim()) {
          eprintln!("Error writing to file: {}", e);
        }
      } else {
        if let Err(e) = self.create_todo_file().and_then(|mut file| writeln!(file, "{}", todo_text.trim())) {
          eprintln!("Error creating or writing to file: {}", e);
        }
      }
    }
  }
}
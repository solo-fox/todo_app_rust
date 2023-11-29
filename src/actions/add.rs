use std::io;
use crate::structions::todo_structions::Todo;
use crate::structions::todo_structions::TodoTrait;
use crate::gfs::write_todo:: {
  TodoWriter
};

pub fn add_todo() {

  let mut todo_struct: Todo = Todo::new();
  println!("What you want to do ?");
  let mut user_input: String = String::new();

  loop {
    match io::stdin().read_line(&mut user_input) {
      Ok(_) if user_input.trim() != "" => {
        todo_struct.name = user_input.trim().to_string();
        break;
      },

      Err(e) => println!("Error Occurred: {}", e),
      _ => {}
    }
  }


  println!("Really! You want to do this -> {}", user_input);

  let todo_writer = TodoWriter::new();

  let todo_data_string: String = todo_struct.to_saveable_string();

  todo_writer.create_todo(todo_data_string.as_str());

  println!("Anway! {} \n", todo_struct.to_string());

}
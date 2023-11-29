// Todo app cli intergrated with file system
// https://github.com/solo-fox

//imports
use std::env;
//Definig modules
mod actions;
mod gfs;
mod structions;
use actions::add::*;
use actions::show::*;
use actions::mark::*;

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() >= 2 && args[1] == "add" {
    add_todo();
  }else if args.len() >= 2 && args[1] == "show" {
    show_todos();
  }else if args.len() >= 2 && args[1] == "mark" {
    mark();
  }else {
    println!("Todo App Usage:<cmd> \n");
    println!("add : Add Todos");
    println!("mark: Mark Todos as completed");
    println!("show: Show all exsisting Todos!");
    println!("\n This software is not under warranty and made for educational purposes!");
  }
}
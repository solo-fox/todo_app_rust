pub mod todo_structions {
  #[derive(Debug)]
  #[derive(Clone)]
  pub struct Todo {
    pub name: String,
    pub done: String
  }
  
  pub trait TodoTrait {
    fn new() -> Todo;
    fn to_saveable_string(&self) -> String;
  }

  impl ToString for Todo {
    fn to_string(&self) -> String {
      return format!("The Todo with the name of  {} has registred and marked as {}", self.name, self.done);
    }
  }

  impl TodoTrait for Todo {
    fn new() -> Todo {
      Todo {
        name: String::from("No Todo"),
        done: String::from("false")
      }
    }
    
    fn to_saveable_string(&self) -> String{
      return format!("{} , {} & "  , self.name , self.done)
    }
  }
}
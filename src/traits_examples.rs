/*
  Traits are basically Abstract classes or Interfaces as in JAVA
*/

// Defining Trait ->
pub trait Summary{
  fn summarize(&self){
    println!("Default Summarize implementation");
  }
}

struct User{
  age: u32,
  name: String
}

impl Summary for User {
  fn summarize(&self) {
      println!("Summary of the user -> Name: {}, Age: {}", self.name, self.age);
  }
}

// Making a function that takes only those elements that implement a particular trait
// Ex -> println! does the same thing, needs the debug trait to be implemented
pub fn describe(value: impl Summary){
  value.summarize();
}

// Serves Same function as above, is more verbose
pub fn _describe_trait_bound<T: Summary>(value: T){
  value.summarize();
}

pub fn _example1(){
  let user1 = User{
    name: String::from("Aryan Rai"),
    age: 22
  };

  user1.summarize();

  // This function will accept arguments that implement the Summary Trait
  describe(user1);
}
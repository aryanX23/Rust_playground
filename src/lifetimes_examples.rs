/*
  Lifetimes are basically a means to keep a check whether the input to a function and the output of the function
  have intersecting time to live, ie -> they dont get cleared uptil the last variable(output) is used.
  It is a way to tell the compiler, the realtion of time to live between different input and output variables
*/
pub trait Summary {
  fn summarize(&self){}
}

struct User<'a>{
  first_name: &'a String,
  last_name: &'a String
}

impl<'a>  Summary for User<'a>{
  fn summarize(&self) {
    println!("Details of user -> FirstName: {}, LastName: {}", self.first_name, self.last_name);
}

}

pub fn example_1(){
  let first_name = String::from("Aryan");
  let user1;
  {
    let last_name = String::from("Rai");
    user1 = User{
      first_name: &first_name,
      last_name: &last_name,
    };
    user1.summarize(); // Can Do this
  }
  // user1.summarize(); Cannot do this | Compiler will throw error

}
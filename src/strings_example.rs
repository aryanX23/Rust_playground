pub fn _create_mod_string(){
  // Create String in rust
  let mut str =  String::from("Goodbye");
  println!("String creation Step -> {str}");

  // Mutate String in rust
  str.push_str(", Cruel World");
  println!("String mutation Step -> {str}");

  // Delete from string
  str.replace_range(0..7, "");
  println!("String Deletion Step -> {str}");

}
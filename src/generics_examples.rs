pub fn _compare_values(){
  let value1 = 1;
  let value2 = 2;

  let valuea = String::from("a");
  let valueb = String::from("b");

  println!("Comparison between two intergers -> {}", _generic_comparator(value1, value2));
  println!("Comparison between two String -> {}", _generic_comparator(valuea, valueb));

  // Note: We cant do this here since only ony one generic type T is taken
  // println!("Comparison between two String -> {}", generic_comparator(valuea, value2));
}

pub fn _generic_comparator<T: std::cmp::PartialOrd>(a: T, b: T) -> T{
  if a > b{
    a
  } else{
    b
  }
}
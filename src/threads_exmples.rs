/*
  Similiar to Thread pools concept in JAVA
*/

// Writing a multi-threaded code to calculate sum of factorials in a range
use std::{io, sync::mpsc, thread};

pub fn _calculate_factorial_sum(){
  let mut limit = String::new();
  io::stdin().read_line(&mut limit).expect("Failed to readline");

  let final_limit: u128 = match limit.trim().parse::<u128>() {
        Ok(value) => value,
        Err(error) => {
            println!("Invalid Input: {}", error);
            return;
        }
    };

  let (tx, rx) = mpsc::channel();
  let mut handle_list = Vec::new();
  
  for i in 0..final_limit+1{
    let producer = tx.clone();
    let handle = thread::spawn(move ||{
      let factorial_value = _calculate_factorial(i);
      println!("Factorial of {} is {}", i, factorial_value);
      producer.send(factorial_value).unwrap();
    });
    handle_list.push(handle);
  }
  drop(tx);

  for handle in handle_list{
    handle.join().unwrap();
  }

  let mut final_result = 0;
  for rec_value in rx{
    final_result += rec_value;
  }
  println!("Sum of Factorials => {}", final_result);
}

fn _calculate_factorial(value: u128) -> u128{
  let mut fact = 1;

  for i in 1..value+1{
    fact = fact * i;
  }

  return fact;
}
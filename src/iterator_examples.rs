pub fn _example1(){
  // Examples of Iterator type 1 ->

  let arr = vec![1,2,3,4,5,6,7,8,9];

  let arr_iter = arr.iter();

  for element in arr_iter{
    println!("{element}")
  }

  println!("{:?}", arr);
}

pub fn _example2(){
  // Examples of Iterator type 2 ->

  let arr = vec![1,2,3,4,5,6,7,8,9];

  for element in arr.iter(){
    println!("{element}")
  }

  println!("{:?}", arr);
}

pub fn _example3(){
  // Examples of Iterator type 3 ->

  let mut arr = vec![1,2,3,4,5,6,7,8,9];

  for element in arr.iter_mut(){
    *element = *element * 2;
    println!("{element}")
  }

  println!("{:?}", arr);
}

pub fn _example4(){
  // Examples of Iterator type 4 ->

  let arr = vec![1,2,3,4,5,6,7,8,9];

  let mut iter_type4 = arr.iter();

  while let Some(value) = iter_type4.next(){
    print!("{value} ");
  }

  println!("{:?}", arr);
}

pub fn _example5(){
  // Examples of Iterator type 4 ->

  let arr = vec![1,2,3,4,5,6,7,8,9];

  let mut iter_type4 = arr.iter();

  while let Some(value) = iter_type4.next(){
    print!("{value} ");
  }

  println!("{:?}", arr);
}

pub fn _example6(){
  // Example of a sepacial iterator -> IMP

  let arr = vec![1,2,3,4,5,6,7,8];

  let arr_iter = arr.into_iter();
  // Transfers Ownership to new variable
  // Best Performance among Iterators

  for element in arr_iter{
    print!("{element} ");
  }

  // Cannot do this here -> 
  // println!("{:?}", arr);
}

pub fn _example7(){

  let arr = vec![1,2,3,4,5,6,7,8,9];


  let iter1 = arr.iter();

  // This is consuming iterator adapter, cant use this iterator again
  let sum: i32 = iter1.sum();
  println!("Sum of Array -> {sum}");

  let iter2 = arr.iter();

  // Non Consuming Iterator adapters
  let iter3 = iter2.map(|x| x+1);
  let iter4 = arr.iter().map(|x| x+1);
  // More examples -> .filter()

  for element in iter3{
    print!("{element} ");
  }

  let new_arr: Vec<i32> = iter4.collect();
  println!("New Vector Produced -> {:?}", new_arr);

}
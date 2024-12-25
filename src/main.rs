use std::io;

mod string_slice;
mod vector_examples;

fn main() {
    // io::stdin().read_line(&mut input).expect("Unable to readline");
    // let out = string_slice::split_word(&input);

    println!("Enter number of input to be accepted -> ");
    let mut length_arr = String::new();

    io::stdin().read_line(&mut length_arr).expect("Error in reading line");
    let length_arr = length_arr.trim().parse().expect("Integer input accepted only!");
    let mut arr = Vec::new();

    for i in 0..length_arr{
        let mut input = String::new();
        print!("Enter element number {i}: ");
        io::stdin().read_line(&mut input).expect("Error in reading line");
        let input: i32 = input.trim().parse().expect("Integer input accepted only!");
        arr.push(input);
    }

    
}


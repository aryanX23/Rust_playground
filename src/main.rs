mod string_slice;
mod vector_examples;
mod hashmap_examples;
mod iterator_examples;
mod strings_example;
mod generics_examples;

fn main() {
    // Sample Code to take input >> 

    // io::stdin().read_line(&mut input).expect("Unable to readline");
    // let out = string_slice::split_word(&input);

    // println!("Enter number of input to be accepted -> ");
    // let mut length_arr = String::new();

    // io::stdin().read_line(&mut length_arr).expect("Error in reading line");
    // let length_arr = length_arr.trim().parse().expect("Integer input accepted only!");
    // let mut arr = Vec::new();

    // for _i in 0..length_arr{
    //     let mut input = String::new();
    //     io::stdin().read_line(&mut input).expect("Error in reading line");
    //     let input: i32 = input.trim().parse().expect("Integer input accepted only!");
    //     arr.push(input);
    // }

    generics_examples::compare_values();
}


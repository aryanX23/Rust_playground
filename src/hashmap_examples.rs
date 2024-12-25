use std::collections::HashMap;

pub fn _impl_hashmap(){
    let mut map: HashMap<String, i32> = HashMap::new();
    map.insert(String::from("one"), 1);
    map.insert(String::from("second"), 2);
    map.insert(String::from("third"), 3);
   
    let result = map.get("second");

    let result = match result {
        Some(value) =>{
            Some(*value)
        }
        None => {
            None
        }
    };

     match result{
        Some(value) => println!("{}", value),
        None => println!("No Value found"),
    }
    print!("{:?}", result);
}
pub fn _even_filter(arr: Vec<i32>) -> Vec<i32>{
  let mut res_arr = Vec::new();

  for ele in arr{
    if ele%2 == 0{
      res_arr.push(ele);
    }
  }

  res_arr
}
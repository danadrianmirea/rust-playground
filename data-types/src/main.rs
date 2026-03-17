fn main() 
{
  // int, float, bool, char
  // u8, u16, ... , u128  
  // i8, i16, ... , i128
  
  let x: i32 = -32;
  let y: u64 = 100;
  
  println!("x: {}", x);
  println!("y: {}", y);
  
  let max = i32::MAX;
  println!("max: {}", max);
  
  // compound data types
  // arrays, tuples, slices, strings
  
  // arrays
  let arr: [i32; 5] = [1, 2, 3, 4, 5];
  println!("arr: {:?}", arr);

  let mixed_arr: [i32; 5] = [0; 5]; // initializes all elements to 0
  println!("mixed_arr: {:?}", mixed_arr);

  // tuples
  let tup: (i32, f64, char) = (42, 3.14, 'R');
  println!("tup: {:?}", tup);

  // strings
  let mut s: String = String::from("Hello, Rust!");
  println!("s: {}", s);

  s += " Welcome to programming."; // concatenating strings
  println!("s: {}", s);

   // slices
  let slice: &[i32] = &arr[1..4]; // creates a slice of the array from index 1 to 3
  println!("slice: {:?}", slice);

  let fruits: [&str; 3] = ["apple", "banana", "cherry"];
  println!("fruits: {:?}", fruits);

  let human: (String, i32, bool) = (String::from("Alice"), 30, true);
  println!("human: {:?}", human);

  let yet_another_tuple = ("Example String", 23, true, [1, 2, 3]);
  println!("yet_another_tuple: {:?}", yet_another_tuple);

  let number_slices:&[i32] = &[10, 20, 30, 40];
  println!("number_slices: {:?}", number_slices);
}

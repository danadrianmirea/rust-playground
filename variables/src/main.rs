fn main() {
    let logical: bool = true;
    let a_float: f64 = 1.32;
    let my_integer = 3;
    let default_float = 3.29;
    let default_integer = 8;
    let my_array: [i32; 5] = [1, 2, 3, 4, 5];
    let mut letter: char = 'A';
   
    println!("logical: {}", logical);
    println!("a_float: {}", a_float);
    println!("my_integer: {}", my_integer);
    println!("default_float: {}", default_float);
    println!("default_integer: {}", default_integer);
    println!("my_array: {:?}", my_array);
    println!("letter: {}", letter);
    letter = 'B';
    println!("letter: {}", letter);

    let mut stone_cold: String = String::from("Stone Cold Steve Austin");
    println!("stone_cold: {}", stone_cold);
    stone_cold.push_str(" Test");
    println!("stone_cold: {}", stone_cold);

    let hw: String = String::from("Hello World");
    println!("hw: {}", hw);
    let slice: &str = &hw[0..5];
    println!("slice: {}", slice);

}

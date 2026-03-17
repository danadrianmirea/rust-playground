fn main() {
    let s1 = String::from("Hello");
    let len = calculate_length(&s1); // send a reference, ownership remains with s1
    //let len = calculate_length(s1); -> this gives an ownership error
    println!("The length of the string is: {}", len);

    let s2 = s1; // this moves ownership of the string from s1 to s2
    //println!("s1: {}", s1); -> this gives an ownership error because s1 is no longer valid after the move
    println!("s2: {}", s2);
}

fn calculate_length(s: &String) -> usize {
    s.len()
} 


fn main() {
    let s1 = String::from("Hello");
    let len = calculate_length(&s1); // send a reference, ownership remains with s1
    //let len = calculate_length(s1); -> this gives an ownership error
    println!("The length of the string is: {}", len);

    let s2 = s1; // this moves ownership of the string from s1 to s2
    //println!("s1: {}", s1); -> this gives an ownership error because s1 is no longer valid after the move
    println!("s2: {}", s2);

    let s3 = s2.clone(); // this creates a deep copy of the string, so both s2 and s3 have ownership of their own string data
    println!("s2: {}, s3: {}", s2, s3);

    let mut x: i32 = 5;
    let y: &mut i32 = &mut x; // this creates a mutable reference to x, but since i32 is a Copy type, we can still use x after this without any issues
    *y += 1; // this modifies the value of x through the mutable reference y
    
    println!("x: {}", x);
    //println!("y: {}", y); // can only have one mutable reference to a value at a time, so this would give an error if we tried to use y after modifying x through it


}

fn calculate_length(s: &String) -> usize {
    s.len()
} 


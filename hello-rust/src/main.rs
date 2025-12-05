fn test() {
    println!("Test fn")
}

fn add(a: u32, b: u32) -> u32 {
    return a + b;
}

fn for_test() {
    for i in 0..10 {
        print!("{} ", i);
    }
    println!();
}

fn while_loop() {
    let mut c = 1;

    while c <= 5 {
        print!("{} ", c);
        c = c + 1;
    }
    println!();
}

fn main() {
    //test();

    //let res = add(2, 3);
    //println!("res is: {}", res);

    //for_test();
    while_loop();
}

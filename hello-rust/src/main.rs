#![allow(dead_code)]
#![allow(unused_variables)]

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

fn loop_test() {
    let mut c = 1;
    loop {
        print!("{} ", c);
        c = c + 1;
        if c > 10 {
            break;
        }
    }
    println!();
}

// static variables use screaming snake case (caps)
static RECT2: Rectangle = Rectangle { width: 20, height: 30 };

// typedefs use camel case
struct Rectangle {
    width: u32,
    height: u32,
}

// function names use snake case
fn struct_test() {
    // local variables use snake case
    let rect = Rectangle { width: 10, height: 20 };
    println!("rect: {}", rect.area());
    struct Rectangle {
        width: u32,
        height: u32,
    }
    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }
}



fn main() {
    //test();

    //let res = add(2, 3);
    //println!("res is: {}", res);

    //for_test();
    //while_loop();
    //loop_test();
    struct_test();
}

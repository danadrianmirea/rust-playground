fn helloworld() {
    println!("Hello, world!");
}

fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

fn main() {
    helloworld();
    let sum = add(5, 10);
    println!("The sum of 5 and 10 is: {}", sum);

    let x : i32 = {
        let y = 3;
        let z = y + 2;
        y+z
    };
    println!("The value of x is: {}", x);
}

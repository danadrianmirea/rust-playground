fn helloworld() {
    println!("Hello, world!");
}

fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

fn calculate_bmi(weight: f32, height: f32) -> f32 {
    return weight / (height * height);
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

    let weight = 70.0; // in kilograms
    let height = 1.75; // in meters
    let bmi = calculate_bmi(weight, height);
    println!("The BMI is: {:.2}", bmi);
}

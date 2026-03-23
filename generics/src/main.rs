use std::fmt::Display;

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

#[derive(Debug)]
struct Pair<T, U> {
    first: T,
    second: U,
}

impl<T, U> Pair<T, U> {
    fn mixup<V, W>(self, other: Pair<V, W>) -> Pair<T, W> {
        Pair {
            first: self.first,
            second: other.second,
        }
    }
}

fn largest<T: PartialOrd + Copy>(items: &[T]) -> T {
    let mut max = items[0];
    for &item in items {
        if item > max {
            max = item;
        }
    }
    max
}

fn print_value<T: Display>(value: T) {
    println!("value: {}", value);
}

fn main() {
    let nums = [4, 10, 2, 99, 23];
    let chars = ['a', 'z', 'm', 'k'];
    println!("largest num: {}", largest(&nums));
    println!("largest char: {}", largest(&chars));

    let int_point = Point { x: 3, y: 7 };
    println!("int point: {:?}, x = {}", int_point, int_point.x());

    let float_point = Point { x: 3.0, y: 4.0 };
    println!(
        "float point: {:?}, distance = {}",
        float_point,
        float_point.distance_from_origin()
    );

    let p1 = Pair {
        first: "left",
        second: 100,
    };
    let p2 = Pair {
        first: true,
        second: "right",
    };
    let mixed = p1.mixup(p2);
    println!("mixed pair: {:?}", mixed);

    print_value("hello generics");
    print_value(42);
}

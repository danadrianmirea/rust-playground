// Rust Enums Basics
// Enums (enumerations) allow you to define a type by enumerating its possible variants.

// Import our library module
mod lib;
use lib::{TrafficLight, Coin, Shape};

fn main() {
    println!("=== Rust Enums Basics ===\n");
    
    // Example 1: Basic Enum
    println!("1. BASIC ENUM EXAMPLE:");
    basic_enum_example();
    
    println!("\n2. ENUM WITH DATA:");
    enum_with_data_example();
    
    println!("\n3. MATCH EXPRESSIONS:");
    match_expressions_example();
    
    println!("\n4. OPTION ENUM:");
    option_enum_example();
    
    println!("\n5. RESULT ENUM:");
    result_enum_example();
    
    println!("\n6. ENUM METHODS:");
    enum_methods_example();
    
    println!("\n7. PATTERN MATCHING:");
    pattern_matching_example();
    
    println!("\n8. IF LET SYNTAX:");
    if_let_example();
    
    println!("\n9. PRACTICAL EXAMPLES:");
    practical_examples();
}

// ============================================
// 1. Basic Enum
// ============================================
fn basic_enum_example() {
    let light = TrafficLight::Red;
    
    println!("   Current light: {:?}", light);
    println!("   Duration: {} seconds", light.duration());
    println!("   Next light: {:?}", light.next());
    
    let lights = vec![TrafficLight::Red, TrafficLight::Yellow, TrafficLight::Green];
    for light in lights {
        println!("   {:?} -> {} seconds -> next: {:?}", 
                 light, light.duration(), light.next());
    }
}

// ============================================
// 2. Enum with Data (Associated Values)
// ============================================
enum WebEvent {
    PageLoad,                    // No data
    PageUnload,                  // No data
    KeyPress(char),              // Single character
    Paste(String),               // String data
    Click { x: i64, y: i64 },    // Named fields like a struct
}

fn enum_with_data_example() {
    let events = vec![
        WebEvent::PageLoad,
        WebEvent::KeyPress('a'),
        WebEvent::Paste(String::from("Hello, Rust!")),
        WebEvent::Click { x: 100, y: 200 },
        WebEvent::PageUnload,
    ];
    
    for event in events {
        match event {
            WebEvent::PageLoad => println!("   Page loaded"),
            WebEvent::PageUnload => println!("   Page unloaded"),
            WebEvent::KeyPress(c) => println!("   Key pressed: '{}'", c),
            WebEvent::Paste(s) => println!("   Pasted text: \"{}\"", s),
            WebEvent::Click { x, y } => println!("   Clicked at ({}, {})", x, y),
        }
    }
}

// ============================================
// 3. Match Expressions (Exhaustive Checking)
// ============================================
fn match_expressions_example() {
    let coins = vec![Coin::Penny, Coin::Nickel, Coin::Dime, Coin::Quarter];
    
    for coin in coins {
        let value = coin.value_in_cents();
        println!("   {:?} is worth {} cents", coin, value);
    }
}

// ============================================
// 4. Option Enum (Built-in)
// ============================================
// Option<T> is Rust's way of handling optional values
// Some(T) - A value exists
// None - No value exists

fn divide(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

fn option_enum_example() {
    let result1 = divide(10.0, 2.0);
    let result2 = divide(10.0, 0.0);
    
    println!("   Using match:");
    match result1 {
        Some(value) => println!("   10 / 2 = {}", value),
        None => println!("   Division by zero!"),
    }
    
    match result2 {
        Some(value) => println!("   10 / 0 = {}", value),
        None => println!("   10 / 0 = Division by zero!"),
    }
    
    println!("\n   Using unwrap (be careful - can panic!):");
    println!("   Result1 unwrapped: {}", result1.unwrap());
    
    println!("\n   Using unwrap_or for default value:");
    println!("   Result2 unwrap_or: {}", result2.unwrap_or(0.0));
    
    println!("\n   Using if let syntax:");
    if let Some(value) = result1 {
        println!("   Result1 using if let: {}", value);
    }
    
    println!("\n   Using map to transform:");
    let doubled = result1.map(|x| x * 2.0);
    println!("   Doubled result: {:?}", doubled);
}

// ============================================
// 5. Result Enum (Built-in)
// ============================================
// Result<T, E> is for operations that can fail
// Ok(T) - Operation succeeded
// Err(E) - Operation failed with error E

fn parse_age(input: &str) -> Result<u8, String> {
    match input.parse::<u8>() {
        Ok(age) => {
            if age > 120 {
                Err(format!("Age {} is unrealistic!", age))
            } else {
                Ok(age)
            }
        }
        Err(_) => Err("Invalid number format".to_string()),
    }
}

fn result_enum_example() {
    let test_cases = vec!["25", "150", "abc", "30"];
    
    println!("   Testing age parsing:");
    for input in test_cases {
        match parse_age(input) {
            Ok(age) => println!("   Valid age: {}", age),
            Err(error) => println!("   Error: {} -> {}", input, error),
        }
    }
    
    println!("\n   Using ? operator for error propagation:");
    let ages = vec!["20", "30", "40"];
    match sum_ages(&ages) {
        Ok(total) => println!("   Sum of valid ages: {}", total),
        Err(error) => println!("   Error summing ages: {}", error),
    }
    
    let bad_ages = vec!["20", "abc", "30"];
    match sum_ages(&bad_ages) {
        Ok(total) => println!("   Sum of valid ages: {}", total),
        Err(error) => println!("   Error summing ages: {}", error),
    }
}

fn sum_ages(inputs: &[&str]) -> Result<u32, String> {
    let mut total = 0;
    
    for input in inputs {
        let age = parse_age(input)?; // ? returns early if error
        total += age as u32;
    }
    
    Ok(total)
}

// ============================================
// 6. Enum Methods
// ============================================
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("   Quit message received"),
            Message::Move { x, y } => println!("   Move to ({}, {})", x, y),
            Message::Write(text) => println!("   Write: \"{}\"", text),
            Message::ChangeColor(r, g, b) => println!("   Change color to RGB({}, {}, {})", r, g, b),
        }
    }
    
    fn is_quit(&self) -> bool {
        matches!(self, Message::Quit)
    }
}

fn enum_methods_example() {
    let messages = vec![
        Message::Quit,
        Message::Move { x: 10, y: 20 },
        Message::Write(String::from("Hello from enum method!")),
        Message::ChangeColor(255, 0, 0),
    ];
    
    println!("   Calling enum methods:");
    for message in messages {
        message.call();
        println!("   Is quit message? {}", message.is_quit());
    }
}

// ============================================
// 7. Pattern Matching with Guards
// ============================================
enum Temperature {
    Celsius(f64),
    Fahrenheit(f64),
    Kelvin(f64),
}

impl Temperature {
    fn to_celsius(&self) -> f64 {
        match self {
            Temperature::Celsius(c) => *c,
            Temperature::Fahrenheit(f) => (*f - 32.0) * 5.0 / 9.0,
            Temperature::Kelvin(k) => *k - 273.15,
        }
    }
    
    fn describe(&self) -> String {
        match self {
            Temperature::Celsius(c) if *c < 0.0 => format!("{}°C (Freezing!)", c),
            Temperature::Celsius(c) if *c > 30.0 => format!("{}°C (Hot!)", c),
            Temperature::Celsius(c) => format!("{}°C", c),
            Temperature::Fahrenheit(f) if *f < 32.0 => format!("{}°F (Freezing!)", f),
            Temperature::Fahrenheit(f) if *f > 86.0 => format!("{}°F (Hot!)", f),
            Temperature::Fahrenheit(f) => format!("{}°F", f),
            Temperature::Kelvin(k) if *k < 273.15 => format!("{}K (Below freezing!)", k),
            Temperature::Kelvin(k) => format!("{}K", k),
        }
    }
}

fn pattern_matching_example() {
    let temperatures = vec![
        Temperature::Celsius(-5.0),
        Temperature::Celsius(20.0),
        Temperature::Celsius(35.0),
        Temperature::Fahrenheit(20.0),
        Temperature::Fahrenheit(75.0),
        Temperature::Fahrenheit(95.0),
        Temperature::Kelvin(250.0),
        Temperature::Kelvin(300.0),
    ];
    
    println!("   Temperature descriptions:");
    for temp in temperatures {
        println!("   {}", temp.describe());
        println!("   -> {}°C", temp.to_celsius());
    }
}

// ============================================
// 8. if let Syntax (Concise Pattern Matching)
// ============================================
enum Status {
    Success(String),
    Error(String),
    Loading,
}

fn if_let_example() {
    let statuses = vec![
        Status::Success("Operation completed".to_string()),
        Status::Error("Something went wrong".to_string()),
        Status::Loading,
    ];
    
    println!("   Using match (exhaustive):");
    for status in &statuses {
        match status {
            Status::Success(msg) => println!("   Success: {}", msg),
            Status::Error(msg) => println!("   Error: {}", msg),
            Status::Loading => println!("   Loading..."),
        }
    }
    
    println!("\n   Using if let (when you only care about one pattern):");
    for status in &statuses {
        if let Status::Success(msg) = status {
            println!("   Great! {}", msg);
        }
    }
    
    println!("\n   Practical example with Option:");
    let maybe_number: Option<i32> = Some(42);
    let no_number: Option<i32> = None;
    
    if let Some(number) = maybe_number {
        println!("   Found number: {}", number);
    }
    
    if let Some(number) = no_number {
        println!("   This won't print: {}", number);
    } else {
        println!("   No number found (using if let)");
    }
}

// ============================================
// 9. Practical Examples
// ============================================
fn practical_examples() {
    println!("   Shape calculations:");
    
    let shapes = vec![
        Shape::Circle(2.0),
        Shape::Rectangle(3.0, 4.0),
        Shape::Square(5.0),
    ];
    
    for shape in shapes {
        // We can't print Shape directly (it doesn't implement Debug)
        // But we can calculate with it
        println!("   Area: {:.2}, Perimeter: {:.2}", 
                 shape.area(), shape.perimeter());
    }
    
    println!("\n   File type example:");
    
    #[derive(Debug)]
    enum FileType {
        Text(String),      // content
        Image(u32, u32),   // width, height
        Audio(f64),        // duration in seconds
    }
    
    let files = vec![
        FileType::Text("Hello Rust!".to_string()),
        FileType::Image(800, 600),
        FileType::Audio(180.5),
    ];
    
    for file in files {
        match file {
            FileType::Text(content) => println!("   Text file: \"{}\"", content),
            FileType::Image(width, height) => println!("   Image: {}x{} pixels", width, height),
            FileType::Audio(duration) => println!("   Audio: {:.1} seconds", duration),
        }
    }
    
    println!("\n   HTTP Status Code example:");
    
    enum HttpStatus {
        Ok,
        NotFound,
        InternalServerError,
        Custom(u16, String),
    }
    
    impl HttpStatus {
        fn code(&self) -> u16 {
            match self {
                HttpStatus::Ok => 200,
                HttpStatus::NotFound => 404,
                HttpStatus::InternalServerError => 500,
                HttpStatus::Custom(code, _) => *code,
            }
        }
        
        fn message(&self) -> String {
            match self {
                HttpStatus::Ok => "OK".to_string(),
                HttpStatus::NotFound => "Not Found".to_string(),
                HttpStatus::InternalServerError => "Internal Server Error".to_string(),
                HttpStatus::Custom(_, message) => message.clone(),
            }
        }
    }
    
    let statuses = vec![
        HttpStatus::Ok,
        HttpStatus::NotFound,
        HttpStatus::InternalServerError,
        HttpStatus::Custom(418, "I'm a teapot".to_string()),
    ];
    
    for status in statuses {
        println!("   HTTP {}: {}", status.code(), status.message());
    }
}

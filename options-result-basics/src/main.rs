// main.rs - Options and Result Demo
// This demonstrates the use of Option and Result types in Rust

fn main() {
    println!("=== Rust Options and Result Demo ===\n");
    
    // ========== OPTION DEMO ==========
    println!("1. OPTION TYPE DEMONSTRATION");
    println!("{}", "-".repeat(40));
    
    // Option represents a value that may or may not exist
    // Some(value) - contains a value
    // None - no value
    
    // Example 1: Basic Option usage
    let some_number: Option<i32> = Some(42);
    let no_number: Option<i32> = None;
    
    println!("Some number: {:?}", some_number);
    println!("No number: {:?}", no_number);
    
    // Example 2: Unwrapping Options
    println!("\nUnwrapping Options:");
    println!("some_number.unwrap(): {}", some_number.unwrap());
    // println!("no_number.unwrap(): {}", no_number.unwrap()); // This would panic!
    
    // Safer ways to handle Options
    match some_number {
        Some(value) => println!("Matched Some({})", value),
        None => println!("Matched None"),
    }
    
    // Using if let
    if let Some(value) = some_number {
        println!("if let matched: {}", value);
    }
    
    // Using unwrap_or for default values
    println!("no_number.unwrap_or(0): {}", no_number.unwrap_or(0));
    println!("some_number.unwrap_or(0): {}", some_number.unwrap_or(0));
    
    // Example 3: Option with functions
    let result1 = divide_option(10.0, 2.0);
    let result2 = divide_option(10.0, 0.0);
    
    println!("\nDivision with Option:");
    println!("10.0 / 2.0 = {:?}", result1);
    println!("10.0 / 0.0 = {:?}", result2);
    
    // ========== RESULT DEMO ==========
    println!("\n\n2. RESULT TYPE DEMONSTRATION");
    println!("{}", "-".repeat(40));
    
    // Result represents either success (Ok) or failure (Err)
    // Ok(value) - successful operation
    // Err(error) - failed operation
    
    // Example 1: Basic Result usage
    let success_result: Result<i32, String> = Ok(42);
    let error_result: Result<i32, String> = Err(String::from("Something went wrong"));
    
    println!("Success result: {:?}", success_result);
    println!("Error result: {:?}", error_result);
    
    // Example 2: Handling Results
    println!("\nHandling Results:");
    
    match success_result {
        Ok(value) => println!("Success: {}", value),
        Err(ref e) => println!("Error: {}", e),
    }
    
    match error_result {
        Ok(value) => println!("Success: {}", value),
        Err(ref e) => println!("Error: {}", e),
    }
    
    // Using unwrap (panics on Err)
    println!("\nUsing unwrap:");
    println!("success_result.unwrap(): {}", success_result.unwrap());
    // println!("error_result.unwrap(): {}", error_result.unwrap()); // This would panic!
    
    // Using unwrap_or_else for error handling
    let value = error_result.unwrap_or_else(|e| {
        println!("Handling error: {}", e);
        0 // default value
    });
    println!("error_result.unwrap_or_else: {}", value);
    
    // Example 3: Result with functions
    let parse_result1 = parse_positive_number("42");
    let parse_result2 = parse_positive_number("-5");
    let parse_result3 = parse_positive_number("not a number");
    
    println!("\nParsing numbers:");
    println!("parse_positive_number(\"42\"): {:?}", parse_result1);
    println!("parse_positive_number(\"-5\"): {:?}", parse_result2);
    println!("parse_positive_number(\"not a number\"): {:?}", parse_result3);
    
    // Example 4: Using ? operator for error propagation
    println!("\nError propagation with ? operator:");
    
    let test_cases = vec!["42", "-5", "100", "abc"];
    
    for input in test_cases {
        match process_number(input) {
            Ok(value) => println!("Successfully processed '{}': {}", input, value),
            Err(e) => println!("Failed to process '{}': {}", input, e),
        }
    }
    
    // ========== PRACTICAL EXAMPLE ==========
    println!("\n\n3. PRACTICAL EXAMPLE: USER VALIDATION");
    println!("{}", "-".repeat(40));
    
    let users = vec![
        User::new("Alice".to_string(), 25),
        User::new("Bob".to_string(), 17), // Underage
        User::new("".to_string(), 30),    // Empty name
        User::new("Charlie".to_string(), 150), // Invalid age
    ];
    
    for user in users {
        match user.validate() {
            Ok(valid_user) => println!("✅ Valid user: {}", valid_user),
            Err(e) => println!("❌ Invalid user: {}", e),
        }
    }
    
    println!("\n=== Demo Complete ===");
}

// ========== HELPER FUNCTIONS ==========

// Function returning Option
fn divide_option(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

// Function returning Result
fn parse_positive_number(s: &str) -> Result<i32, String> {
    match s.parse::<i32>() {
        Ok(num) => {
            if num >= 0 {
                Ok(num)
            } else {
                Err(format!("'{}' is not positive", num))
            }
        }
        Err(_) => Err(format!("'{}' is not a valid number", s)),
    }
}

// Function using ? operator for error propagation
fn process_number(s: &str) -> Result<i32, String> {
    let num = parse_positive_number(s)?;
    // If we get here, num is valid
    Ok(num * 2) // Double the number
}

// ========== STRUCT EXAMPLE ==========

#[derive(Debug)]
struct User {
    name: String,
    age: i32,
}

impl User {
    fn new(name: String, age: i32) -> Self {
        User { name, age }
    }
    
    fn validate(&self) -> Result<&Self, String> {
        if self.name.is_empty() {
            return Err("Name cannot be empty".to_string());
        }
        
        if self.age < 0 {
            return Err("Age cannot be negative".to_string());
        }
        
        if self.age > 120 {
            return Err("Age is unrealistic".to_string());
        }
        
        if self.age < 18 {
            return Err("User must be at least 18 years old".to_string());
        }
        
        Ok(self)
    }
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({} years old)", self.name, self.age)
    }
}

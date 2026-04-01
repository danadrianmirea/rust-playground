# Rust Enums Basics

A comprehensive tutorial on Rust enums (enumerations) covering basic to intermediate concepts.

## What are Enums?

Enums (enumerations) allow you to define a type by enumerating its possible variants. They're one of Rust's most powerful features for creating type-safe APIs.

## Topics Covered

### 1. Basic Enums
- Simple enum definitions
- Variants without data
- Basic usage patterns

### 2. Enums with Data
- Variants with associated values
- Tuple-style variants
- Struct-style variants
- Mixed variant types

### 3. Match Expressions
- Exhaustive pattern matching
- Match arms and expressions
- Returning values from match

### 4. Option Enum
- Handling optional values
- Some(T) vs None
- unwrap, unwrap_or, if let
- Common Option patterns

### 5. Result Enum
- Handling operations that can fail
- Ok(T) vs Err(E)
- Error propagation with ?
- Custom error types

### 6. Enum Methods
- Implementing methods on enums
- Using `impl` blocks
- Helper methods and utilities

### 7. Pattern Matching
- Match guards (if conditions)
- Complex pattern matching
- Nested patterns
- Wildcard patterns (_)

### 8. if let Syntax
- Concise pattern matching
- When to use if let vs match
- while let patterns

## Running the Examples

```bash
cd enums-basics
cargo run
```

## Code Examples

### Basic Enum
```rust
enum TrafficLight {
    Red,
    Yellow,
    Green,
}
```

### Enum with Data
```rust
enum WebEvent {
    PageLoad,
    KeyPress(char),
    Click { x: i64, y: i64 },
}
```

### Pattern Matching
```rust
match coin {
    Coin::Penny => 1,
    Coin::Nickel => 5,
    Coin::Dime => 10,
    Coin::Quarter => 25,
}
```

### Option Enum
```rust
fn divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None
    } else {
        Some(a / b)
    }
}
```

### Result Enum
```rust
fn parse_age(input: &str) -> Result<u8, String> {
    input.parse().map_err(|_| "Invalid age".to_string())
}
```

## Key Concepts

1. **Type Safety** - Enums ensure you handle all possible cases
2. **Pattern Matching** - Exhaustive checking prevents bugs
3. **Associated Data** - Variants can carry different types of data
4. **Methods** - Enums can have methods like structs
5. **Standard Library Enums** - Option and Result are everywhere in Rust

## Common Patterns

### Handling Option
```rust
// Using match
match maybe_value {
    Some(value) => println!("Got: {}", value),
    None => println!("No value"),
}

// Using if let
if let Some(value) = maybe_value {
    println!("Got: {}", value);
}

// Using unwrap_or
let value = maybe_value.unwrap_or(default_value);
```

### Handling Result
```rust
// Using match
match result {
    Ok(value) => println!("Success: {}", value),
    Err(error) => println!("Error: {}", error),
}

// Using ? for propagation
fn process() -> Result<(), String> {
    let value = might_fail()?; // Returns early if error
    Ok(())
}
```

## Exercises to Try

1. Add a new variant to `TrafficLight` called `BlinkingYellow`
2. Create an enum for different file types (Text, Image, Audio) with metadata
3. Implement a simple calculator using enums for operations
4. Create an enum for HTTP status codes with appropriate data
5. Implement a state machine using enums (e.g., vending machine states)

## Learning Resources

- [The Rust Book: Enums](https://doc.rust-lang.org/book/ch06-00-enums.html)
- [Rust by Example: Enums](https://doc.rust-lang.org/rust-by-example/custom_types/enum.html)
- [Standard Library: Option](https://doc.rust-lang.org/std/option/enum.Option.html)
- [Standard Library: Result](https://doc.rust-lang.org/std/result/enum.Result.html)
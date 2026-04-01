fn main() {
    println!("=== Rust String Manipulation Demo ===\n");

    // 1. Creating strings
    println!("1. Creating strings:");
    let s1 = String::from("Hello, Rust!"); // From string literal
    let s2 = "Hello, World!".to_string(); // Using to_string()
    let s3 = String::new(); // Empty string
    println!("   s1 = \"{}\"", s1);
    println!("   s2 = \"{}\"", s2);
    println!("   s3 = \"{}\" (empty)", s3);
    println!();

    // 2. String concatenation
    println!("2. String concatenation:");
    // Using + operator (note: moves ownership of first string)
    let hello = String::from("Hello, ");
    let world = String::from("World!");
    let hello_world = hello + &world; // hello is moved, world borrowed
    println!("   Using + operator: \"{}\"", hello_world);
    // hello is no longer accessible here
    
    // Using format! macro (doesn't move ownership)
    let name = "Alice";
    let greeting = format!("Hello, {}! Welcome to Rust.", name);
    println!("   Using format! macro: \"{}\"", greeting);
    
    // Using push_str (mutates in-place)
    let mut mutable_string = String::from("Hello");
    mutable_string.push_str(", mutable world!");
    println!("   Using push_str: \"{}\"", mutable_string);
    println!();

    // 3. Slicing and indexing (UTF-8 aware)
    println!("3. Slicing and indexing (UTF-8 aware):");
    let text = "Rust 🦀 is awesome!";
    println!("   Original text: \"{}\"", text);
    println!("   Length in bytes: {}", text.len());
    println!("   Length in chars: {}", text.chars().count());
    
    // Safe slicing with char boundaries
    if let Some(slice) = text.get(0..4) {
        println!("   Slice 0..4: \"{}\"", slice); // "Rust"
    }
    
    // Demonstrate that Rust prevents invalid UTF-8 slicing at compile time/runtime
    // text[0..5] would panic because 5 is not a char boundary
    println!("   Trying to slice at char boundaries only.");
    println!();

    // 4. Common string methods
    println!("4. Common string methods:");
    let sentence = "   Rust is a systems programming language.   ";
    println!("   Original: \"{}\"", sentence);
    println!("   Trimmed: \"{}\"", sentence.trim());
    println!("   Contains 'systems': {}", sentence.contains("systems"));
    println!("   Find position of 'programming': {:?}", sentence.find("programming"));
    
    // Replace
    let replaced = sentence.replace("systems", "safe");
    println!("   Replace 'systems' with 'safe': \"{}\"", replaced);
    
    // Split
    println!("   Split by whitespace:");
    for word in sentence.split_whitespace() {
        println!("     - \"{}\"", word);
    }
    
    // Uppercase/Lowercase
    println!("   Uppercase: \"{}\"", sentence.to_uppercase());
    println!("   Lowercase: \"{}\"", sentence.to_lowercase());
    println!();

    // 5. Iteration over characters and bytes
    println!("5. Iteration over characters and bytes:");
    let sample = "Rust🦀";
    println!("   String: \"{}\"", sample);
    
    println!("   Characters:");
    for (i, ch) in sample.chars().enumerate() {
        println!("     {}: '{}' (U+{:04X})", i, ch, ch as u32);
    }
    
    println!("   Bytes (UTF-8 encoding):");
    for (i, byte) in sample.bytes().enumerate() {
        println!("     {}: 0x{:02X}", i, byte);
    }
    println!();

    // 6. UTF-8 handling examples
    println!("6. UTF-8 handling examples:");
    let emoji_string = "Hello 👋 World 🌍!";
    println!("   String with emojis: \"{}\"", emoji_string);
    println!("   Number of characters: {}", emoji_string.chars().count());
    println!("   Number of bytes: {}", emoji_string.len());
    
    // Demonstrate that emojis are multiple bytes
    let wave_emoji = "👋";
    println!("   Emoji '{}' details:", wave_emoji);
    println!("     Bytes: {:?}", wave_emoji.as_bytes());
    println!("     Char count: {}", wave_emoji.chars().count());
    println!();

    // 7. Converting between String and &str
    println!("7. Converting between String and &str:");
    let string_val = String::from("I am a String");
    let str_slice: &str = &string_val; // Borrow as &str
    println!("   String: \"{}\"", string_val);
    println!("   Borrowed as &str: \"{}\"", str_slice);
    
    let another_str = "I am a &str";
    let converted_string = another_str.to_string(); // &str to String
    println!("   &str: \"{}\"", another_str);
    println!("   Converted to String: \"{}\"", converted_string);
    println!();

    // 8. Ownership and borrowing examples
    println!("8. Ownership and borrowing examples:");
    let owner = String::from("owned string");
    let borrower = &owner; // immutable borrow
    println!("   Owner: \"{}\"", owner);
    println!("   Borrower: \"{}\"", borrower);
    // Can still use owner because borrow is immutable
    
    let mut mutable_owner = String::from("mutable ");
    mutable_owner.push_str("string");
    println!("   Mutable owner after push: \"{}\"", mutable_owner);
    
    // Multiple immutable borrows allowed
    let borrow1 = &mutable_owner;
    let borrow2 = &mutable_owner;
    println!("   First borrow: \"{}\"", borrow1);
    println!("   Second borrow: \"{}\"", borrow2);
    println!();

    // 9. Practical example: Building a CSV line
    println!("9. Practical example: Building a CSV line:");
    let name = "John Doe";
    let age = "30";
    let city = "New York";
    let csv_line = format!("{},{},{}", name, age, city);
    println!("   CSV line: \"{}\"", csv_line);
    
    // Parsing the CSV line back
    let parts: Vec<&str> = csv_line.split(',').collect();
    println!("   Parsed parts: {:?}", parts);
    println!();

    println!("=== Demo Complete ===");
    println!("\nKey takeaways:");
    println!("• Rust strings are UTF-8 encoded by default");
    println!("• Use &str for string slices, String for owned mutable strings");
    println!("• Indexing is byte-based, not character-based (for safety)");
    println!("• Many convenient methods: trim, replace, split, contains, etc.");
    println!("• Iteration with .chars() and .bytes() for UTF-8 awareness");
}
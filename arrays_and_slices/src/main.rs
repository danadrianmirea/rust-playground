fn main() {
    println!("=== Rust Arrays and Slices Demo ===\n");
    
    // 1. Array Basics
    println!("1. ARRAY BASICS");
    println!("---------------");
    
    // Fixed-size array
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Fixed-size array: {:?}", numbers);
    
    // Array with same values
    let zeros: [i32; 8] = [0; 8];
    println!("Array of 8 zeros: {:?}", zeros);
    
    // Type inference
    let inferred = [10, 20, 30];
    println!("Inferred array: {:?} (type: [i32; 3])", inferred);
    
    println!();
    
    // 2. Array Operations
    println!("2. ARRAY OPERATIONS");
    println!("-------------------");
    
    // Accessing elements
    println!("First element: {}", numbers[0]);
    println!("Last element: {}", numbers[numbers.len() - 1]);
    
    // Mutable array
    let mut mutable_array = [1, 2, 3, 4, 5];
    println!("Before modification: {:?}", mutable_array);
    mutable_array[2] = 100;
    println!("After modifying index 2: {:?}", mutable_array);
    
    // Array length
    println!("Array length: {}", numbers.len());
    
    println!();
    
    // 3. Multi-dimensional Arrays
    println!("3. MULTI-DIMENSIONAL ARRAYS");
    println!("---------------------------");
    
    let matrix: [[i32; 3]; 2] = [
        [1, 2, 3],
        [4, 5, 6],
    ];
    println!("2x3 matrix: {:?}", matrix);
    println!("Element at [1][2]: {}", matrix[1][2]);
    
    // 3D array
    let cube: [[[i32; 2]; 2]; 2] = [
        [[1, 2], [3, 4]],
        [[5, 6], [7, 8]],
    ];
    println!("2x2x2 cube: {:?}", cube);
    
    println!();
    
    // 4. SLICES
    println!("4. SLICES");
    println!("---------");
    
    // Creating slices from arrays
    let slice1: &[i32] = &numbers[1..4]; // elements 2, 3, 4
    println!("Slice of numbers[1..4]: {:?}", slice1);
    
    let slice2 = &numbers[..3]; // first three elements
    println!("Slice of numbers[..3]: {:?}", slice2);
    
    let slice3 = &numbers[2..]; // from index 2 to end
    println!("Slice of numbers[2..]: {:?}", slice3);
    
    let full_slice = &numbers[..]; // whole array as slice
    println!("Full slice of array: {:?}", full_slice);
    
    println!();
    
    // 5. Slice Operations
    println!("5. SLICE OPERATIONS");
    println!("-------------------");
    
    // Slice length
    println!("Slice length: {}", slice1.len());
    
    // Check if slice is empty
    let empty_slice: &[i32] = &[];
    println!("Empty slice: {:?}, is_empty: {}", empty_slice, empty_slice.is_empty());
    
    // First and last elements of slice
    if !slice1.is_empty() {
        println!("First element of slice1: {}", slice1[0]);
        println!("Last element of slice1: {}", slice1[slice1.len() - 1]);
    }
    
    // Iterating over slices
    print!("Iterating over slice1: ");
    for &num in slice1 {
        print!("{} ", num);
    }
    println!();
    
    println!();
    
    // 6. Mutable Slices
    println!("6. MUTABLE SLICES");
    println!("-----------------");
    
    let mut data = [10, 20, 30, 40, 50];
    println!("Original data: {:?}", data);
    
    let mutable_slice = &mut data[1..4];
    println!("Mutable slice (before modification): {:?}", mutable_slice);
    
    // Modify the slice
    for elem in mutable_slice.iter_mut() {
        *elem *= 2;
    }
    println!("Mutable slice (after modification): {:?}", mutable_slice);
    println!("Original array after slice modification: {:?}", data);
    
    println!();
    
    // 7. String Slices
    println!("7. STRING SLICES");
    println!("----------------");
    
    let message = String::from("Hello, Rust!");
    let string_slice: &str = &message[0..5];
    println!("String: '{}'", message);
    println!("String slice [0..5]: '{}'", string_slice);
    
    // String literals are slices
    let literal = "Hello, World!";
    println!("String literal: '{}' (type: &str)", literal);
    
    // Getting bytes as slice
    let bytes_slice = literal.as_bytes();
    println!("Bytes slice: {:?}", bytes_slice);
    
    println!();
    
    // 8. Common Patterns
    println!("8. COMMON PATTERNS");
    println!("------------------");
    
    // Passing slices to functions
    fn sum_slice(slice: &[i32]) -> i32 {
        slice.iter().sum()
    }
    
    let test_array = [1, 2, 3, 4, 5];
    println!("Sum of {:?}: {}", test_array, sum_slice(&test_array));
    
    // Getting middle of array
    let middle = &test_array[1..test_array.len() - 1];
    println!("Middle of array (excluding ends): {:?}", middle);
    
    // Pattern matching with slices
    match middle {
        [2, 3, 4] => println!("Pattern matched: middle is [2, 3, 4]"),
        _ => println!("Pattern not matched"),
    }
    
    println!();
    
    // 9. Safety Features
    println!("9. SAFETY FEATURES");
    println!("------------------");
    
    // Bounds checking (commented out to avoid panic during demo)
    println!("Bounds checking is performed at runtime.");
    println!("Uncomment the line below to see panic:");
    println!("// let out_of_bounds = numbers[10]; // This would panic!");
    
    // Using get() for safe access
    match numbers.get(2) {
        Some(&value) => println!("Safe access with get(2): {}", value),
        None => println!("Index 2 is out of bounds"),
    }
    
    match numbers.get(10) {
        Some(&value) => println!("Index 10 contains: {}", value),
        None => println!("Safe access with get(10): Index out of bounds (returns None)"),
    }
    
    println!();
    println!("=== Demo Complete ===");
}
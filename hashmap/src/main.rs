use std::collections::HashMap;

fn main() {
    let mut scores: HashMap<&str, i32> = HashMap::new();
    scores.insert("Alice", 90);
    scores.insert("Bob", 75);
    scores.insert("Carol", 82);
    println!("initial scores: {:?}", scores);

    // Get values safely.
    println!("Alice score: {:?}", scores.get("Alice"));
    println!("Dave score: {:?}", scores.get("Dave"));

    // Update an existing key.
    scores.insert("Bob", 80);
    println!("Bob updated: {:?}", scores.get("Bob"));

    // Use entry() for insert-if-missing and in-place mutation.
    scores.entry("Dave").or_insert(70);
    scores.entry("Alice").and_modify(|s| *s += 5);
    println!("after entry updates: {:?}", scores);

    // Iterate key/value pairs.
    println!("all students:");
    for (name, score) in &scores {
        println!("  {name}: {score}");
    }

    // Check for key existence and removal.
    println!("contains Bob? {}", scores.contains_key("Bob"));
    let removed = scores.remove("Carol");
    println!("removed Carol: {:?}, now: {:?}", removed, scores);

    // Build a frequency map.
    let words = ["apple", "banana", "apple", "orange", "banana", "apple"];
    let mut freq: HashMap<&str, i32> = HashMap::new();
    for w in words {
        *freq.entry(w).or_insert(0) += 1;
    }
    println!("word frequency: {:?}", freq);

    // Collect from iterators.
    let pairs = [("x", 10), ("y", 20), ("z", 30)];
    let from_pairs: HashMap<&str, i32> = pairs.into_iter().collect();
    println!("from pairs: {:?}", from_pairs);
}

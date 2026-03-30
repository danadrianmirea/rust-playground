fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    // HashMap to store the value and its index
    let mut num_map = std::collections::HashMap::new();
    
    // Iterate through the list
    for (i, &num) in nums.iter().enumerate() {
        // Calculate the complement needed to reach the target
        let complement = target - num;
        
        // Check if the complement exists in the HashMap
        if let Some(&index) = num_map.get(&complement) {
            return vec![index, i as i32];
        }
        
        // Store the number and its index in the HashMap
        num_map.insert(num, i as i32);
    }
    
    // If no solution is found, return an empty vector
    vec![]
}

fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let result = two_sum(nums, target);
    println!("{:?}", result);  // Output: [0, 1]
}
fn main() {
    let mut nums: Vec<i32> = Vec::new();
    nums.push(10);
    nums.push(20);
    nums.push(30);
    println!("nums after push: {:?}", nums);

    let popped = nums.pop();
    println!("popped: {:?}, nums: {:?}", popped, nums);

    let first_index = nums[0];
    let second_safe = nums.get(1);
    let missing_safe = nums.get(99);
    println!(
        "index access: {}, safe access existing: {:?}, safe missing: {:?}",
        first_index, second_safe, missing_safe
    );

    for n in &nums {
        print!("{} ", n);
    }
    println!();

    for n in &mut nums {
        *n *= 2;
    }
    println!("after mutate with iter_mut: {:?}", nums);

    let from_macro = vec![5, 1, 5, 3, 2, 3];
    let mut sorted_unique = from_macro.clone();
    sorted_unique.sort();
    sorted_unique.dedup();
    println!(
        "from vec! macro: {:?}, sorted+dedup: {:?}",
        from_macro, sorted_unique
    );

    let evens_squared: Vec<i32> = from_macro
        .iter()
        .copied()
        .filter(|x| x % 2 == 0)
        .map(|x| x * x)
        .collect();
    println!("filter/map/collect: {:?}", evens_squared);

    let mut a = vec![1, 2];
    let mut b = vec![3, 4];
    a.extend([5, 6]);
    a.append(&mut b);
    println!("extend+append a: {:?}, b after append: {:?}", a, b);

    let mut stack = vec!["a", "b", "c"];
    while let Some(top) = stack.pop() {
        print!("{} ", top);
    }
    println!();
}

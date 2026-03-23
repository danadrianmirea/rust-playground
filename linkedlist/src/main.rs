use std::collections::LinkedList;

fn main() {
    // Build a list from both ends.
    let mut list: LinkedList<i32> = LinkedList::new();
    list.push_back(20);
    list.push_back(30);
    list.push_front(10);
    println!("after push_front/push_back: {:?}", list);

    // Peek at first/last values.
    println!("front: {:?}, back: {:?}", list.front(), list.back());

    // Pop from both ends.
    let first = list.pop_front();
    let last = list.pop_back();
    println!(
        "pop_front: {:?}, pop_back: {:?}, remaining: {:?}",
        first, last, list
    );

    // Extend from an iterator.
    list.extend([1, 2, 3, 4, 5]);
    println!("after extend: {:?}", list);

    // Iterate and compute a sum.
    let sum: i32 = list.iter().sum();
    println!("sum via iter(): {}", sum);

    // Mutate elements in place.
    for n in list.iter_mut() {
        *n *= 10;
    }
    println!("after iter_mut multiply: {:?}", list);

    // Append one list into another (moves nodes, no element cloning).
    let mut a: LinkedList<&str> = LinkedList::new();
    a.extend(["A", "B"]);
    let mut b: LinkedList<&str> = LinkedList::new();
    b.extend(["C", "D"]);
    a.append(&mut b);
    println!("append -> a: {:?}, b: {:?}", a, b);

    // split_off keeps the first part in `nums` and returns the tail.
    let mut nums: LinkedList<i32> = (1..=7).collect();
    let tail = nums.split_off(4);
    println!("split_off(4) -> left: {:?}, right: {:?}", nums, tail);

    // Queue pattern: push_back + pop_front.
    let mut queue: LinkedList<&str> = LinkedList::new();
    queue.push_back("job-1");
    queue.push_back("job-2");
    queue.push_back("job-3");
    println!("queue start: {:?}", queue);
    while let Some(job) = queue.pop_front() {
        print!("processing {} ", job);
    }
    println!();

    // Stack pattern: push_front + pop_front.
    let mut stack: LinkedList<&str> = LinkedList::new();
    stack.push_front("first");
    stack.push_front("second");
    stack.push_front("third");
    print!("stack pop order: ");
    while let Some(top) = stack.pop_front() {
        print!("{} ", top);
    }
    println!();
}

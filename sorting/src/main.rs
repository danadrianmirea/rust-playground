// import the rand crate for generating random numbers
use rand::Rng;

fn bubble_sort(arr: &mut [i32]) {
    let n = arr.len();
    for i in 0..n {
        for j in 0..n - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

fn test_bubble_sort() {
    println!("Testing bubble sort...");
    let mut arr = generate_random_array(10);
    println!("Original array: {:?}", arr);
    bubble_sort(&mut arr);
    println!("Sorted array: {:?}", arr);
}

fn insertion_sort(arr: &mut [i32]) {
    let n = arr.len();
    for i in 1..n {
        let key = arr[i];
        let mut j = i as i32 - 1;
        while j >= 0 && arr[j as usize] > key {
            arr[(j + 1) as usize] = arr[j as usize];
            j -= 1;
        }
        arr[(j + 1) as usize] = key;
    }
}

fn generate_random_array(size: usize) -> Vec<i32> {
    let mut arr = Vec::with_capacity(size);
    for _ in 0..size {
        arr.push(rand::random::<i32>() % 100);
    }
    arr
}

fn test_insertion_sort() {
    println!("Testing insertion sort...");
    let mut arr = generate_random_array(10);
    println!("Original array: {:?}", arr);
    insertion_sort(&mut arr);
    println!("Sorted array: {:?}", arr);
}

fn selection_sort(arr: &mut [i32]) {
    let n = arr.len();
    for i in 0..n {
        let mut min_index = i;
        for j in (i + 1)..n {
            if arr[j] < arr[min_index] {
                min_index = j;
            }
        }
        arr.swap(i, min_index);
    }
}

fn test_selection_sort() {
    println!("Testing selection sort...");
    let mut arr = generate_random_array(10);
    println!("Original array: {:?}", arr);
    selection_sort(&mut arr);
    println!("Sorted array: {:?}", arr);
}

fn quick_sort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }
    let pivot_index = partition(arr);
    quick_sort(&mut arr[0..pivot_index]);
    quick_sort(&mut arr[pivot_index + 1..]);
}

fn partition(arr: &mut [i32]) -> usize {
    let pivot = arr[arr.len() - 1];
    let mut i = 0;
    for j in 0..arr.len() - 1 {
        if arr[j] < pivot {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, arr.len() - 1);
    i
}

fn test_quick_sort() {
    println!("Testing quick sort...");
    let mut arr = generate_random_array(10);
    println!("Original array: {:?}", arr);
    quick_sort(&mut arr);
    println!("Sorted array: {:?}", arr);
}

fn main() {
    test_bubble_sort();
    test_insertion_sort();
    test_selection_sort();
    test_quick_sort();
}   


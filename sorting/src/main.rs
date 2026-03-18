// import the rand crate for generating random numbers
use log::{info, debug};
use env_logger;
use std::time::Instant;

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

fn test_bubble_sort(arr: &mut [i32]) {
    debug!("Testing bubble sort...");
    debug!("Original array: {:?}", arr);
    let start = Instant::now();
    bubble_sort(arr);
    let duration = start.elapsed();
    info!("Bubble sort took: {:?}", duration);
    debug!("Sorted array: {:?}", arr);
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

fn test_insertion_sort(arr: &mut [i32]) {
    debug!("Testing insertion sort...");
    debug!("Original array: {:?}", arr);
    let start = Instant::now();
    insertion_sort(arr);
    let duration = start.elapsed();
    info!("Insertion sort took: {:?}", duration);
    debug!("Sorted array: {:?}", arr);
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

fn test_selection_sort(arr: &mut [i32]) {
    debug!("Testing selection sort...");
    debug!("Original array: {:?}", arr);
    let start = Instant::now();
    selection_sort(arr);
    let duration = start.elapsed();
    info!("Selection sort took: {:?}", duration);
    debug!("Sorted array: {:?}", arr);
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

fn test_quick_sort(arr: &mut [i32]) {
    debug!("Testing quick sort...");
    debug!("Original array: {:?}", arr);
    let start = Instant::now();
    quick_sort(arr);
    let duration = start.elapsed();
    info!("Quick sort took: {:?}", duration);
    debug!("Sorted array: {:?}", arr);
}

fn heap_sort(arr: &mut [i32]) {
    let n = arr.len();
    for i in (0..n / 2).rev() {
        heapify(arr, n, i);
    }
    for i in (1..n).rev() {
        arr.swap(0, i);
        heapify(arr, i, 0);
    }
}

fn heapify(arr: &mut [i32], n: usize, i: usize) {
    let mut largest = i;
    let left = 2 * i + 1;
    let right = 2 * i + 2;

    if left < n && arr[left] > arr[largest] {
        largest = left;
    }
    if right < n && arr[right] > arr[largest] {
        largest = right;
    }
    if largest != i {
        arr.swap(i, largest);
        heapify(arr, n, largest);
    }
}

fn test_heap_sort(arr: &mut [i32]) {
    debug!("Testing heap sort...");
    debug!("Original array: {:?}", arr);
    let start = Instant::now();
    heap_sort(arr);
    let duration = start.elapsed();
    info!("Heap sort took: {:?}", duration);
    debug!("Sorted array: {:?}", arr);
}

fn merge_sort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }
    let mid = arr.len() / 2;
    merge_sort(&mut arr[0..mid]);
    merge_sort(&mut arr[mid..]);
    merge(arr, mid);
}

fn merge(arr: &mut [i32], mid: usize) {
    let mut left = arr[0..mid].to_vec();
    let mut right = arr[mid..].to_vec();
    left.push(i32::MAX);
    right.push(i32::MAX);
    let mut i = 0;
    let mut j = 0;
    for k in 0..arr.len() {
        if left[i] <= right[j] {
            arr[k] = left[i];
            i += 1;
        } else {
            arr[k] = right[j];
            j += 1;
        }
    }
}

fn test_merge_sort(arr: &mut [i32]) {
    debug!("Testing merge sort...");
    debug!("Original array: {:?}", arr);
    let start = Instant::now();
    merge_sort(arr);
    let duration = start.elapsed();
    info!("Merge sort took: {:?}", duration);
    debug!("Sorted array: {:?}", arr);
}

fn main() {
    env_logger::init();
    
    let size = 10000;
    info!("Generating random array of size {}...", size);
    let arr = generate_random_array(size);

    test_bubble_sort(&mut arr.clone());
    test_insertion_sort(&mut arr.clone());
    test_selection_sort(&mut arr.clone());
    test_quick_sort(&mut arr.clone());
    test_heap_sort(&mut arr.clone());
    test_merge_sort(&mut arr.clone());
}
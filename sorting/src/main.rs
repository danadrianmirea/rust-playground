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
    let mut arr = [64, 34, 25, 12, 22, 11, 90];
    println!("Original array: {:?}", arr);
    bubble_sort(&mut arr);
    println!("Sorted array: {:?}", arr);
}

fn main() {
    test_bubble_sort();
}

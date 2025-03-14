fn bubble_sort(arr: &mut [i32]) {
    let n = arr.len();
    for i in 0..n {
        let mut swapped = false; // Optimization: Track if a swap happened
        for j in 0..n - 1 - i {
            if arr[j] > arr[j + 1] {
                // Manual swapping without using swap()
                let temp = arr[j];
                arr[j] = arr[j + 1];
                arr[j + 1] = temp;
                
                swapped = true;
            }
        }
        if !swapped {
            break; // Stop early if already sorted
        }
    }
}

fn main() {
    let mut arr = [2, 4, 1, 9, 4, 6, 3, 7, 22, 12];
    bubble_sort(&mut arr);
    println!("Sorted array: {:?}", arr);
}

pub fn counting_sort(arr: &mut Vec<u32>, exp: u32) {
    let mut output = vec![0; arr.len()];
    let mut count = vec![0; 10];

    // Count occurrences
    for &num in arr.iter() {
        let index = (num / exp) % 10;
        count[index as usize] += 1;
    }

    // Change count[i] so it now contains actual position
    for i in 1..10 {
        count[i] += count[i - 1];
    }

    // Build the output array
    for &num in arr.iter().rev() {
        let index = (num / exp) % 10;
        output[count[index as usize] as usize - 1] = num;
        count[index as usize] -= 1;
    }

    // Copy sorted array back to original array
    arr.copy_from_slice(&output);
}

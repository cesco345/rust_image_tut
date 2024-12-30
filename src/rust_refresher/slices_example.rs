fn find_max(slice: &[i32]) -> i32 {
    let mut max = slice[0]; // Initialize with the first element
    for &num in slice.iter() {
        if num > max {
            max = num;
        }
    }
    max
}

fn main() {
    let numbers = [3, 7, 9, 2, 8, 10, 1];

    // Use slices to analyze subarrays without copying
    let part1 = &numbers[0..3]; // Slice of first 3 elements
    let part2 = &numbers[3..]; // Slice from index 3 to the end

    println!("Max in part1: {}", find_max(part1)); // Output: 9
    println!("Max in part2: {}", find_max(part2)); // Output: 10
}

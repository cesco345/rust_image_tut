fn main() {
    // Fixed-size array of test scores
    let scores: [i32; 5] = [88, 92, 79, 85, 90]; // 5 elements

    let mut sum = 0;
    for score in scores.iter() {
        sum += score;
    }

    let average = sum as f32 / scores.len() as f32; // Calculate average
    println!("Average score: {:.2}", average);
}

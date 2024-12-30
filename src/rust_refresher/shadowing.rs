fn main() {
    let input = "42"; // Initial input as a string

    // Shadowing allows transforming the variable step-by-step
    let input: i32 = input
        .trim() // Remove any surrounding whitespace
        .parse() // Parse it into an integer
        .expect("Invalid number!"); // Handle parsing errors

    println!("Parsed number: {}", input);

    // Shadowing again to apply further transformations
    let input = input * 2; // Double the value

    println!("Doubled number: {}", input);
}

use std::io;

fn main() {
    println!("Temperature Converter: Fahrenheit to Celsius");

    // Prompt the user to enter a temperature
    println!("Enter temperature in Fahrenheit:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input) // Read user input
        .expect("Failed to read input");

    // Parse the input into a floating-point number
    let fahrenheit: f64 = input
        .trim() // Remove whitespace
        .parse() // Convert string to number
        .expect("Please enter a valid number!");

    // Perform the conversion
    let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;

    // Display the result
    println!("{:.2}°F is {:.2}°C", fahrenheit, celsius);
}

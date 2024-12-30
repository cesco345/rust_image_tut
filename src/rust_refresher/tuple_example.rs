fn circle_properties(radius: f64) -> (f64, f64) {
    let area = std::f64::consts::PI * radius * radius;
    let circumference = 2.0 * std::f64::consts::PI * radius;
    (area, circumference) // Return a tuple
}

fn main() {
    let radius = 5.0;

    // Destructure the tuple into variables
    let (area, circumference) = circle_properties(radius);

    println!("Radius: {}", radius);
    println!("Area: {:.2}", area);
    println!("Circumference: {:.2}", circumference);
}

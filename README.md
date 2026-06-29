# rust-variables
Rust 10 most common variable examples

fn main() {
    // 1. Integer
    let age: i32 = 25;

    // 2. Floating-point number
    let price: f64 = 19.99;

    // 3. Boolean
    let is_logged_in: bool = true;

    // 4. Character
    let grade: char = 'A';

    // 5. String slice (&str)
    let language: &str = "Rust";

    // 6. Owned String
    let message: String = String::from("Hello, Rust!");

    // 7. Array
    let numbers: [i32; 5] = [10, 20, 30, 40, 50];

    // 8. Tuple
    let person: (&str, u8, bool) = ("Alice", 30, true);

    // 9. Vector (growable array)
    let fruits = vec!["Apple", "Banana", "Orange"];

    // 10. Mutable variable
    let mut score = 100;
    score += 50;

    println!("Age: {}", age);
    println!("Price: {}", price);
    println!("Logged in: {}", is_logged_in);
    println!("Grade: {}", grade);
    println!("Language: {}", language);
    println!("Message: {}", message);
    println!("First number: {}", numbers[0]);
    println!("Person: {:?}", person);
    println!("Fruits: {:?}", fruits);
    println!("Score: {}", score);
}

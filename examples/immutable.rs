// ==========================================================
// immutable.rs
// Rust Tutorial: Immutable Variables
// ==========================================================
//
// By default, every variable in Rust is immutable.
// This means once a value is assigned, it cannot be changed.
//
// Syntax:
//
// let variable_name = value;
//
// ==========================================================

fn main() {
    println!("==============================");
    println!("Rust Immutable Variables");
    println!("==============================\n");

    // ------------------------------------------------------
    // Example 1 - Creating an immutable variable
    // ------------------------------------------------------

    let name = "Alice";

    println!("Example 1");
    println!("Name: {}\n", name);

    // The following line would NOT compile because
    // 'name' is immutable.
    //
    // name = "Bob";
    //
    // Error:
    // cannot assign twice to immutable variable

    // ------------------------------------------------------
    // Example 2 - Different immutable types
    // ------------------------------------------------------

    let age = 25;
    let height = 1.75;
    let student = true;
    let grade = 'A';

    println!("Example 2");
    println!("Age     : {}", age);
    println!("Height  : {}", height);
    println!("Student : {}", student);
    println!("Grade   : {}\n", grade);

    // ------------------------------------------------------
    // Example 3 - Rust infers the type automatically
    // ------------------------------------------------------

    let number = 100;
    let price = 29.99;
    let city = "Tokyo";

    println!("Example 3");
    println!("Number : {}", number);
    println!("Price  : {}", price);
    println!("City   : {}\n", city);

    // ------------------------------------------------------
    // Example 4 - Explicit type annotation
    // ------------------------------------------------------

    let id: u32 = 1001;
    let temperature: f64 = 36.5;

    println!("Example 4");
    println!("ID          : {}", id);
    println!("Temperature : {}\n", temperature);

    // ------------------------------------------------------
    // Example 5 - Variables can be used in expressions
    // ------------------------------------------------------

    let width = 10;
    let height = 5;

    let area = width * height;

    println!("Example 5");
    println!("Width  : {}", width);
    println!("Height : {}", height);
    println!("Area   : {}\n", area);

    // Notice:
    // width and height never changed.
    // Instead, we created a NEW immutable variable called "area".

    println!("Tutorial completed!");
}

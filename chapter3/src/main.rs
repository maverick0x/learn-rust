// Common Programming Concepts

fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // floating point numbers
    let _y = 2.0; // defaults to f64
    let _z: f32 = 3.0; // specified f32 directly

    // working with tuples
    let tup: (i32, f64, u8) = (600, 5.90, 255);
    let (a, b, c) = tup; // destructuring the tuple into individual variables
    // let (x, y, z) = (tup.0, tup.1, tup.2); // another way to destructure the tuple
    println!("The value of a is: {a}");
    println!("The value of b is: {b}");
    println!("The value of c is: {c}");

    // working with arrays; arrays have a fixed length and types
    let arr = [1, 2, 3, 4, 5];
    let first = arr[0];
    println!("The first element of the array is: {first}");

    // working with functions
    let result = add(5, 10);
    println!("The result of adding 5 and 10 is: {result}");

    // working with control flow
    let number = 7;
    if number < 5 {
        println!("The number is less than 5");
    } else if number == 5 {
        println!("The number is equal to 5");
    } else {
        println!("The number is greater than 5");
    }
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}
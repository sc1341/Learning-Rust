// Typical Rust imports
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};






/*
 * Pattern matching - `match` is an expression that allows you to compare a value against pattersna
 * dn execute code based on the matching patter. 
 *
 * */

/*
enum TrafficLight{

    Red, 
    Yellow,
    Green,

}

fn action(light: TrafficLight){
    match light {

        TrafficLight::Red => println!("STOP"),
        TrafficLight::Green => println!("GO"),
        TrafficLight::Yellow => println!("SPEED UP"),
    }

}

fn action_demo(){

    let light = TrafficLight::Yellow;
    action(light);

}

*/

/*
 * The two functions below demonstrate Lifetimes in Rust. Lifetimes ensure that references are
 * valid as long as needed, and they are a part of Rust's borrow checker and prevent dangling
 * references. In this example, the lifetime `'a` ensures that the reference returned by `longest`
 * is valid for as long as both inputs are valid. 
 *
 * */
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn lifetime_demo() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}




/*
 * The code below demonstrates the differences between borrowing and taking ownership of objects in
 * memory. This is important in rust, and is definied by taking smart pointers in as arguments.
 * Rust is smart and knows where variables are going to be most of the time, but explicit
 * declarations are also appropriate sometimes. 
 *
 * */
fn sample_borrowing_ownership() {
    let s1 = String::from("hello");
    take_ownership(s1);
    // s1 is no longer valid here

    let s2 = String::from("world");
    let s3 = borrow_string(&s2);
    // s2 remains valid here
    println!("s2 is still: {}", s2);
}

fn take_ownership(s: String) {
    println!("String: {}", s);
} // s goes out of scope and 'drop' is called to free memory

fn borrow_string(s: &String) -> &String {
    println!("Borrowed string: {}", s);
    s
}




/*
 * File Handling in Rust
 *
 * */
fn file_handling() {
    // Create and write to the file, print error message if it fails
    if let Err(e) = File::create("output.txt").and_then(|mut f| f.write_all(b"Hello rust!")) {
        eprintln!("Failed to create or write to file: {}", e);
        return;
    }

    // Read from the file, print error message if it fails
    let mut contents = String::new();
    if let Err(e) = File::open("output.txt").and_then(|mut f| f.read_to_string(&mut contents)) {
        eprintln!("Failed to open or read from file: {}", e);
        return;
    }

    println!("File Contains: {}", contents);
}



fn add_numbers(a: i32, b:i32) -> i32{
    return a + b;
    // OR just a + b and skip the return -> Compiler doesn't like return (a + b); because the
    // paranthesis is unnecessary 
}



#[derive(Debug)]
enum TrafficLight {

    Red,
    Yellow,
    Green,
}

impl TrafficLight {

    fn next(&self) -> TrafficLight {

        use TrafficLight::*;
        match self {
            Red => Green,
            Green => Yellow,
            Yellow => Red,
        }
    }

}


fn test_light_system(){
    let mut light = TrafficLight::Red;
    for _ in 0..5 {
        light = light.next();
        println!("{:?}", light);
    }
}

fn main() {
    // Data types
    let result: i32 = add_numbers(5, 10);
    println!("The result of 5 + 10 is: {}", result);
    //file_handling();
    test_light_system();

}

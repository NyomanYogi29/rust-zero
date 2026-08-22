mod basic;
/*
    THE BEGINNING BEFORE THE STORM HIT
*/

fn main() {
    println!("Hello, world!");

    println!("Hello, Nyoman Yogi!");
}

#[test]
fn variable_test() {
    let name = "Nyoman Yogi"; // <- This variable is immutable, means that it cannot be overwritten by a new value

    // Try to remove the "mut" keyword and see if the compiler like it
    let mut age = 19; // <- And this one is mutable

    age = age + 1; // I can rewrite the value by using the same variable's name, and adding it to 1 to make it 20.

    print!("Hello, {}, your age is {} years old\n", name, age);
}

#[test]
fn static_typing_test() {
    /* Like Golang, Rust also is a static type programming language. 
    Meaning, if you try to overwrite a variable's value with different data type, 
    the compiler will complaint.
    */
    let name = "Nyoman Yogi";

    /* Try to undo this multi-line comment
    name = 10; // <- Expected string, but found integer instead
    */

    println!("Hi, {}!", name);
}

#[test]
fn shadowing_test() {
    /* Basically, these two variables are different from what the compiler's judge, 
    but definitely will confusing the one who read our code 
    */

    // Not the best practice if doing too often
    let name = "Nyoman Yogi";
    println!("First name, {name}");
    
    let name = 10;
    println!("Second name, {name}");

    /* They just happen to be sitting at a different memory location
    if you declare two variable with the same identical name. 
    */
}
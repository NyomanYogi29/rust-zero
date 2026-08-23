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

/*
Scalar data type: integer, float, boolean, char
 */
#[test]
fn scalar_data_type_test() {
    let age: i32 = 19;

    let name: &str = "Nyoman Yogi";

    println!("{}, {}", age, name);
}

/*
Compound data type:
- Tuple (can have multiple data type inside a list of data) ["str", int, bool]
- Array (cannot having different data type inside the list) [int, int, int]
*/


#[test]
fn number_conversion_test() {
    let a: i8 = 10;
    println!("{}",a);
    
    let b: i16 = a as i16;
    println!("{}",b);
    
    let c: i32 = a as i32;
    println!("{}",c);

    let d: i64 = 1000000000;
    println!("{}", d);

    let e: i8 = d as i8;
    println!("{}", e);
}

// Numeric operations
#[test]
fn numeric_operations() {
    let a = 10;
    let b = 23;
    let c = 20;

    let mut d = a * b * c;

    d += 23;

    println!("{}", d);
}

#[test]
fn comparion_operator() {
    let a = 10;
    let age = 23;

    let result = a < age;

    println!("{result}");
}

#[test]
fn boolean_operation() {
    let absen = 80;
    let nilai_akhir = 100;

    let lulus = absen >= 75;
    let lulus_nilai_akhir = nilai_akhir >= 75;

    let lulus_final = lulus && lulus_nilai_akhir;

    println!("{}", lulus_final)
}

#[test]
fn char() {
    let a: char = 'a';
    let b: char = 'b';

    println!("{}, {}", a, b);
}

#[test]
fn unit() {
    println!("Hello");
}

#[test]
fn tuple() {
    let mut data: (i32, f64, bool) = (10, 40.0, true);
    println!("{:?}", data);

    let first_data = data.1;
    println!("{}", first_data);

    let (a, b, _) = data;
    println!("{}, {}", a, b);

    data.1 = 23.0;
    let (_, a, _) = data;
    print!("{a}\n");

    let unit = unit();
    println!("{:?}", unit);
}

// Array
#[test]
fn array() {
    let mut array: [i32; 5] = [1, 2, 3, 4, 5];

    array[0] = 10;
    println!("{:?}", array);

    let array_len = array.len();
    println!("{}", array_len);

    let mut two_dimensional = [
        [1, 2],
        [3, 4],
        [5, 6]
    ];

    two_dimensional[1][1] = 10;

    println!("{:?}", two_dimensional);
    println!("{}", two_dimensional[1][1]);
}

const MAXIMUM: i32 = 10;

#[test]
fn contant() {
    println!("{}", MAXIMUM);
}

// Memory management
#[test]
fn stack_heap() {
    function_a();
    function_b();
}

fn function_a() {
    let a = 10;
    let b = String::from("Nyoman");

    println!("{}, {}", a, b);
}

fn function_b() {
    let a = 10;
    let b = String::from("Nyoman");

    println!("{}, {}", a, b);
}

// String
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
    let a:i8 = 10;
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
fn additional_test() {

}
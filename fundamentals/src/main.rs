fn main() {
    var_mutability();
    compound_types();
    println!("{}", parameter_inputs(69, 'p'));
    control_flow();
}

// scalar data types and mutability
fn var_mutability() {
    // variable to be tested on the shadow
    let mut x = 6;
    // consts
    const CAPPED_CONSTS: u8 = 13;
    // shadowing
    {
        x *= 2;
        println!("inner scope {x}")
    }
    println!("variable = {x}, constant variable = {CAPPED_CONSTS}");

    // determine number of characters and vectors
    let y = "hello world".len();
    let z = vec!["hi", "yo", "hey"].len();
    println!("string = {}, vector = {:?}", y, z)
}

// compound data types TODO: add vecs
fn compound_types() {
    // tuple
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("{five_hundred} {six_point_four} {one}");
    // arrays
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    println!("{first} {second}");
}

// parameters
fn parameter_inputs(value: i32, unit_label: char) -> &'static str {
    println!("The measurement is: {value}{unit_label}");
    "hello"
}

// if, TODO: match control flow
fn control_flow() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

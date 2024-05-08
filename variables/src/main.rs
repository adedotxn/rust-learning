fn main() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6; // mutated x
    println!("The value of x is: {x} \n");

    let y = 4;
    println!("The value of y is: {y}");
    let y = y + 1; // shadowed y
    println!("The value of shadowed y is: {y}");


    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y in the global scope is: {y}");


    let tup : (i32, f64, u8) = (500, 6.4, 1);
    let (a, b, c) = tup; // pattern matching to destructure the tuple
    println!("The value of a is: {a}");

    let value_of_b = tup.1;
    println!("The value of b is: {value_of_b}");

    let array: [i32; 5] = [1, 2, 3, 4, 5];
    let array_of_same_values = [3; 5]; // [3, 3, 3, 3, 3]

}


// Path: variables/src/main.rs
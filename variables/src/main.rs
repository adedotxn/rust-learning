use std::collections::HashMap;

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

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (a, b, c) = tup; // pattern matching to destructure the tuple
    println!("The value of a is: {a}");

    let value_of_b = tup.1;
    println!("The value of b is: {value_of_b}");

    let array: [i32; 5] = [1, 2, 3, 4, 5];
    let array_of_same_values = [3; 5]; // [3, 3, 3, 3, 3]

    // Vectors
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    for i in &v {
        println!("The value of i is: {i}");
    }

    for i in &mut v {
        *i += 50;
        println!("The value of i is: {i}");
    }

    let v_2 = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v_2[2];
    println!("The third element is: {third}");

    let third: Option<&i32> = v_2.get(2);
    match third {
        Some(third) => println!("The third element is: {third}"),
        None => println!("There is no third element."),
    }

    // HashMaps
    let mut scores = HashMap::new();
    scores.insert(String::from("Man_City"), 10);
    scores.insert(String::from("Liverpool"), 9);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }
    scores.entry(String::from("Chelsea")).or_insert(50);

    let score = scores.get("Liverpool").copied().unwrap_or(0);
    // println!("The score of Liverpool is: {score}");

    let team_name = "Man_City";
    let score = scores.get(team_name).copied().unwrap_or(0);
    // println!("The score of {team_name} is: {score}");

    for (key, value) in &scores {
        println!("\t{key}: {value}");
    }

    let text = "what a wonderful world, wonderful";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");
}

// Path: variables/src/main.rs

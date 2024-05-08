use std::io;


fn main() {
    println!("Hello, world!");

    let y = {
        let x = 3;
        x + 1
    }; // this block, {},  is an expression that evaluates to 4. It is not terminated by a semicolon

    another_function(5);
    print_sum(5, 6);

    let x = get_five();
    println!("The value of x is: {x}");

    let added_two = plus_two(5);
    println!("The value of added_two is: {added_two}");

    control_flow();
    repition();

    
    let command = "celcius";
    let value: f64 = 32.0;
    let conversion = convert(command, value);
    println!("\nConverting {value} to {command} gives {conversion} ");

    println!("\n TIME FOR INTERACTIVE CONVERSION");
    take_input_to_convert();
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_sum(x: i32, y: i32) {
    let result = x + y;
    println!("The sum of x and y is: {result}");
}


//  but the body of the function is a lonely 5 with no semicolon because it’s an expression whose value we want to return.
fn get_five() -> i32 {
    5
}

fn plus_two(x: i32) -> i32 {
    x + 2
}

fn control_flow() {
    let number = 3;

    if number < 5 {
        println!("The condition was true");
    } else {
        println!("The condition was false");
    }

    let condition = false;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");


}

fn repition() {
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2
        }
    };

    println!("The result is: {result}");

    let mut number = 3;

    while number != 0 {
        println!("{number}!");
        number -= 1;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }


    println!("LIFTOFF!!!");
}


fn convert(command: &str, value: f64) -> f64 {
    if command == "fahrenheit" {
        (value * 1.8) + 32.0
    } else if command == "celcius" {
        (value - 32.0) / 1.8
    } else {
        value
    }
}


fn take_input_to_convert() {
    println!("What type of conversion would you like to do?");
    println!("If to celcius, write 'celcius'");
    println!("If fahrenheit, write 'fahrenheit'");
    println!("any other command would result in zero. YOU HAVE BEEN WARNED");

            println!("\nPlease input your command.");
    
    let mut command:String = String::new();

    io::stdin()
        .read_line(&mut command)
        .expect("Failed to read line");

    let command = command.trim();

    println!("\n Now, enter the value you want to convert as a floating point number");

    let mut value = String::new();
    io::stdin()
        .read_line(&mut value)
        .expect("Failed to read line");

   
    let value: f64 = if value.trim().is_empty() {
        0.0
    } else {
        match value.trim().parse() {
            Ok(num) => num,
            Err(_) => 0.0,
        }
    };
    
    println!("\t You want to convert {value} to {command}? LFG!!");

    let conversion = convert(command, value);

    println!("\n Here's the result of your interactive conversion: {conversion} degrees {command}");

}
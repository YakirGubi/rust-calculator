use std::io::{self, Write};

fn main() {
    let mut first_number = String::new();
    let mut second_number = String::new();
    let mut operation = String::new();

    println!("Rust calculator");

    print!("Enter your first number: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut first_number).unwrap();
    let first_number: i32 = first_number
        .trim()
        .parse()
        .expect("Can't convert from String to i32");

    print!("Enter your second number: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut second_number).unwrap();
    let second_number: i32 = second_number
        .trim()
        .parse()
        .expect("Can't convert from String to i32");

    print!("Enter your operation number (+,-,*,/): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut operation).unwrap();
    let operation: char = operation
        .trim()
        .parse()
        .expect("Can't convert from String to char");

    match operation {
        '+' => println!(
            "Result:\n\t{first_number} + {second_number} = {}",
            first_number + second_number
        ),
        '-' => println!(
            "Result:\n\t{first_number} - {second_number} = {}",
            first_number - second_number
        ),
        '*' => println!(
            "Result:\n\t{first_number} * {second_number} = {}",
            first_number * second_number
        ),
        '/' => println!(
            "Result:\n\t{first_number} / {second_number} = {}",
            first_number / second_number
        ),
        _ => println!("You don't enter leagual operation!"),
    }
}

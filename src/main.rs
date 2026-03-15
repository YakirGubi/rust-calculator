use std::{
    io::{self, Write},
    str::FromStr,
};

fn main() {
    println!("Rust calculator");

    print!("Enter your first number: ");
    io::stdout().flush().unwrap();
    let first_number: i32 = get_input().expect("Can't convert from String to i32");

    print!("Enter your second number: ");
    io::stdout().flush().unwrap();
    let second_number: i32 = get_input().expect("Can't convert from String to i32");

    print!("Enter your operation number (+,-,*,/): ");
    io::stdout().flush().unwrap();
    let operation: char = get_input().expect("Can't convert from String to char");

    let result = calculate(first_number, second_number, operation);

    match result {
        Ok(result) => println!("Result:\n\t{first_number} {operation} {second_number} = {result}"),
        Err(err) => println!("{err}"),
    }
}

fn get_input<T: FromStr>() -> Result<T, T::Err> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse()
}

fn calculate(first_number: i32, second_number: i32, operation: char) -> Result<i32, String> {
    match operation {
        '+' => match first_number.checked_add(second_number) {
            Some(result) => Ok(result),
            None => Err(String::from("Integer Overflow!")),
        },
        '-' => match first_number.checked_sub(second_number) {
            Some(result) => Ok(result),
            None => Err(String::from("Integer Overflow!")),
        },
        '*' => match first_number.checked_mul(second_number) {
            Some(result) => Ok(result),
            None => Err(String::from("Integer Overflow!")),
        },
        '/' => match first_number.checked_div(second_number) {
            Some(result) => Ok(result),
            None => Err(String::from("Integer Overflow or You divided by zero!")),
        },
        _ => Err(String::from("You don't enter leagual operation!")),
    }
}

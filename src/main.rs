//! Rust calculator
//!
//! This is a program that take three inputs from the user, two are numbers (i32) and one is the
//! operation to uplay on them.
//!
//! It will look somthing like that:
//! first_number operation second_number
//!
//! and then it will print the result like this:
//! {first_number} {operation} {second_number} = {result}
//!
use std::{
    error::Error,
    io::{self, Write},
    str::FromStr,
};

fn main() {
    println!("Rust calculator");

    print!("Enter your first number: ");
    io::stdout().flush().expect("Faild to print!");
    let first_number: i32 = get_input().expect("Can't convert from String to i32");

    print!("Enter your second number: ");
    io::stdout().flush().expect("Faild to print!");
    let second_number: i32 = get_input().expect("Can't convert from String to i32");

    print!("Enter your operation number (+,-,*,/): ");
    io::stdout().flush().expect("Faild to print!");
    let operation: char = get_input().expect("Can't convert from String to char");

    let result = calculate(first_number, second_number, operation);

    match result {
        Ok(result) => println!("Result:\n\t{first_number} {operation} {second_number} = {result}"),
        Err(err) => println!("{err}"),
    }
}

/// Get input from the user via the CLI and return it generecly.
///
/// #Returns
/// generic valu by the type that call the function. (unles it an Err)
///
/// #Errors
/// Can't read the input
/// Can't convert from String to <T>
///
fn get_input<T>() -> Result<T, Box<dyn Error>>
where
    T: FromStr,
    T::Err: Error + 'static,
{
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().parse()?)
}

/// Calculator
///
/// #Arguments
/// first_number: i32
/// second_number: i32
/// operation: char: ('+' or '-' or '*' or '/')
///
/// #Returns
/// Ok(result: i32)
/// Err(String)
///
/// #Errors
/// You don't enter leagual operation
/// Integer Overflow
/// Divided by zero
///
/// #Examples
///```
///println!(calculate(3,5,'+'));
///println!(calculate(4,1,'-'));
///println!(calculate(7,6,'*'));
///println!(calculate(12,2,'/'));
///```
fn calculate(first_number: i32, second_number: i32, operation: char) -> Result<i32, String> {
    let result = match operation {
        '+' => first_number.checked_add(second_number),
        '-' => first_number.checked_sub(second_number),
        '*' => first_number.checked_mul(second_number),
        '/' => first_number.checked_div(second_number),
        _ => return Err(String::from("You don't enter leagual operation!")),
    };

    match result {
        Some(result) => Ok(result),
        None => match operation {
            '/' => Err(String::from("Integer Overflow or divided by zero!")),
            _ => Err(String::from("Integer Overflow!")),
        },
    }
}

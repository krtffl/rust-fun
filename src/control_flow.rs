use std::{io, num::ParseIntError};

// conditional statements
pub fn integer_sign() {
    println!("enter an integer: ");
    let mut x = String::new();
    io::stdin().read_line(&mut x).unwrap();

    let x: i32 = x.trim().parse().unwrap();

    if x == 0 {
        println!("your number is 0!");
    } else if x > 0 {
        println!("your number is positive!");
    } else {
        println!("your number is negative!");
    }
}

pub fn even_or_odd() {
    println!("enter an integer: ");
    let mut x = String::new();
    io::stdin().read_line(&mut x).unwrap();

    let x: i32 = x.trim().parse().unwrap();

    if x % 2 == 0 {
        println!("your number is even");
    } else {
        println!("your number is odd");
    }
}

// looping constructs
pub fn count_to_ten() {
    for number in 1..11 {
        println!("{}", number);
    }
}

pub fn factorial() -> i32 {
    println!("enter an integer: ");
    let mut x = String::new();
    io::stdin().read_line(&mut x).unwrap();

    let mut x = x.trim().parse::<i32>().unwrap();

    let mut factorial = 1;

    while x > 1 {
        factorial = factorial * x;
        x -= 1;
    }

    factorial
}

// pattern matching
pub fn integer_sign_match() {
    println!("enter an integer: ");
    let mut x = String::new();
    io::stdin().read_line(&mut x).unwrap();

    let x = x.trim().parse::<i32>().unwrap();

    match x {
        x if x > 0 => println!("positive!"),
        x if x < 0 => println!("negative!"),
        _ => println!("it's zero!"),
    }
}

pub fn vowel_or_consonant() {
    println!("enter an lowercase letter: ");
    let mut x = String::new();
    io::stdin().read_line(&mut x).unwrap();

    let input_char = x.chars().next().unwrap();

    match input_char {
        'a' | 'e' | 'i' | 'u' | 'o' => println!("it's a vowel!"),
        _ => println!("it's a consonant!"),
    }
}

pub fn print_user_operations() -> Result<(), ParseIntError> {
    println!("enter a first number: ");
    let mut x = String::new();
    io::stdin().read_line(&mut x).unwrap();

    println!("enter a second number: ");
    let mut y = String::new();
    io::stdin().read_line(&mut y).unwrap();

    let x = x.trim().parse::<i32>()?;
    let y = y.trim().parse::<i32>()?;

    println!("{} + {} = {}", x, y, x + y);
    println!("{} * {} = {}", x, y, x * y);
    println!("{} - {} = {}", x, y, x - y);

    Ok(())
}

pub fn factorial_result() -> Result<i32, ParseIntError> {
    println!("enter a positive integer: ");
    let mut x = String::new();
    io::stdin().read_line(&mut x).unwrap();

    let mut x = x.trim().parse::<i32>()?;
    let mut factorial = 1;

    while x > 1 {
        factorial = factorial * x;
        x -= 1;
    }

    Ok(factorial)
}

use std::io;

// hello world
pub fn hello_world() {
    println!("hello world!");
}

// variables and data types
pub fn print_variables() {
    let x = -42;
    let y = 3.14;

    println!("x = {}, y = {}", x, y);
}

pub fn print_operations(x: i32, y: i32) {
    println!("{} + {} = {}", x, y, x + y);
    println!("{} * {} = {}", x, y, x * y);
    println!("{} - {} = {}", x, y, x - y);
}

// basic input - output
pub fn greeting() {
    println!("what is your name?");
    let mut user_name = String::new();
    io::stdin().read_line(&mut user_name).unwrap();
    println!("hallo {}!", user_name.trim());
}

pub fn print_user_operations() {
    println!("enter a first number: ");
    let mut x = String::new();
    io::stdin().read_line(&mut x).unwrap();

    println!("enter a second number: ");
    let mut y = String::new();
    io::stdin().read_line(&mut y).unwrap();

    let x: i32 = x.trim().parse().unwrap();
    let y: i32 = y.trim().parse().unwrap();

    println!("{} + {} = {}", x, y, x + y);
    println!("{} * {} = {}", x, y, x * y);
    println!("{} - {} = {}", x, y, x - y);
}

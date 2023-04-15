// use control_flow::{
//     count_to_ten, even_or_odd, factorial, factorial_result, integer_sign, vowel_or_consonant,
// };

// use crate::control_flow::integer_sign_match;
// use introduction::{
//     greeting, hello_world, print_operations, print_user_operations, print_variables,
// };

// use functions::add;
// use structs::divide;

// use crate::functions::{add_generic, apply_twice, is_even, is_prime};

// use collections::sum_vec;

// use crate::collections::{remove_duplicates, reverse_strings, sum_even_numbers};

mod collections;
mod control_flow;
mod functions;
mod introduction;
mod structs;
fn main() {
    // L E S S O N  1
    // hello_world();
    // print_variables();
    // print_operations(1, 10);
    // greeting();
    // print_user_operations();

    // L E S S O N  2
    // integer_sign();
    // even_or_odd();
    // count_to_ten();
    // println!("{}", factorial(4));
    // integer_sign_match();
    // vowel_or_consonant();
    // println!("{}", factorial_result().unwrap());

    // L E S S O N  3
    // println!("{}", add(1, 6));
    // println!("{}", is_prime(11));
    // println!("{}", is_prime(15));
    // println!("{}", is_even(10));
    // println!("{}", add_generic(10, 11));
    // println!("{}", add_generic(1.1, 2.2));
    // println!("{}", add_generic(-1.1, 2.2));
    // println!("{}", apply_twice(|x| x + 1, 1));

    // L E S S O N  4
    // println!("{:?}", divide(1.0, 1.0));
    // println!("{:?}", divide(1152.1, 0.0));

    // L E S S O N  5
    // println!("{}", sum_vec(vec![1, 2, 3, 4, 5]));
    // println!("{:?}", remove_duplicates(vec![1, 2, 3, 4, 5, 5, 1, 2]));
    // println!("{:?}", sum_even_numbers(vec![1, 2, 3, 4, 5, 5, 1, 2]));
    // println!(
    //     "{:?}",
    //     reverse_strings(vec!["Hello".to_string(), "World".to_string()])
    // );
}

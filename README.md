# Rust Fun

This repository contains a collection of Rust exercises built by [@perezdid](https://github.com/perezdid) with the help of ChatGPT, an AI language model by OpenAI. The exercises are organized by lessons, and each exercise has been solved by the author with the assistance of ChatGPT, which provided feedback and corrections.

## Lesson 1: Introduction

**Topics**: Printing, variables, basic operations, user input, and basic error handling.

1.1.1: Write a `hello_world` function that prints "hello world!".
1.2.1: Write a `print_variables` function that declares two variables, an integer and a floating-point number, and prints their values.
1.3.1: Write a `print_operations` function that takes two integer arguments and prints the sum, product, and difference of those integers.
1.4.1: Write a `greeting` function that asks the user for their name and prints a greeting message with their name.
1.4.2: Write a `print_user_operations` function that asks the user for two integer inputs and prints the sum, product, and difference of those integers.

## Lesson 2: Control Flow

**Topics**: if/else statements, loops, pattern matching, error handling with Result.

2.1.1: Write an `integer_sign` function that asks the user for an integer input and prints if it's positive, negative, or zero.
2.1.2: Write an `even_or_odd` function that asks the user for an integer input and prints if it's even or odd.
2.2.1: Write a `count_to_ten` function that prints the numbers from 1 to 10 using a loop.
2.2.2: Write a `factorial` function that takes an integer input and returns its factorial using a loop.
2.3.1: Write an `integer_sign_match` function that asks the user for an integer input and prints if it's positive, negative, or zero using pattern matching.
2.3.2: Write a `vowel_or_consonant` function that asks the user for a lowercase letter input and prints if it's a vowel or a consonant using pattern matching.
2.4.1: Modify the `print_user_operations` function to use Result for error handling.
2.4.2: Modify the `factorial` function to use Result for error handling.

## Lesson 3: Functions and Closures

**Topics**: Functions, function overloading, traits, closures.

3.1: Write an `add` function that takes two integers and returns their sum.
3.2: Write an `is_prime` function that takes an integer input and returns a boolean indicating if it's a prime number.
3.3: Write an `is_even` function that takes an integer input and returns a boolean indicating if it's even.
3.4: Write an `add_generic` function that takes two generic arguments and returns their sum using the Add trait.
3.5: Write an `apply_twice` function that takes a closure and an integer input, applies the closure twice, and returns the result.

## Lesson 4: Structs, Enums, and Error Handling

**Topics**: Structs, enums, methods, error handling with Option and Result.

4.1: Define a `Point` struct and implement a `distance_to_origin` method.
4.2: Define a `Rectangle` struct and implement `is_square` and `area` methods.
4.3: Define a `Shape` enum and implement an `area` method for it.
4.4: Write a `divide` function that takes two floating-point numbers and returns their division using Option for error handling.
4.5: Modify the `is_prime` function to use Result for error handling.

## Lesson 5: Collections

**Topics**: Vectors, iterators, and common methods.

5.1: Write a `sum_vec` function that takes a vector of integers and returns their sum.
5.2: Write a `remove_duplicates` function that takes a vector of integers and returns a new vector without duplicates.
5.3: Write a `sum_even_numbers` function that takes a vector of integers and returns the sum of even numbers.
5.4: Write a `reverse_strings` function that takes a vector of strings and returns a new vector with each string reversed.

## Lesson 6: Error Handling and Advanced Topics

**Topics**: Advanced error handling, custom error types, working with the standard library, and refactoring code.

6.1: Write a `first_character` function that takes a string and returns an Option containing its first character.
6.2: Write a `find_char` function that takes a string and a character and returns an Option containing the index of the first occurrence of the character.
6.3: Define a `ParseNameError` enum with two variants, Empty and TooLong. Write a `length_check` function that takes a string and returns a Result with either the string itself or a ParseNameError.
6.4: Refactor the following functions to use the Result type for error handling: `first_character_result`, `find_char_result`.

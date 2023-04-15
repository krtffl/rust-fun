// functions
pub fn add(x: i32, y: i32) -> i32 {
    x + y
}

pub fn is_prime(x: i32) -> bool {
    if x <= 1 {
        return false;
    }
    for i in 2..((x as f64).sqrt() as i32 + 1) {
        if x % i == 0 {
            return false;
        }
    }
    true
}
pub fn is_even(x: i32) -> bool {
    x % 2 == 0
}

// functions overloading (traits)
use std::ops::Add;
pub fn add_generic<X: Add>(x: X, y: X) -> <X as Add>::Output {
    x + y
}

// closures
pub fn apply_twice<F>(f: F, x: i32) -> i32
where
    F: Fn(i32) -> i32,
{
    f(f(x))
}

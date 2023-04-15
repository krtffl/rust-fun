// collections
pub fn sum_vec(x: Vec<i32>) -> i32 {
    let mut sum = 0;
    for val in x.iter() {
        sum += val
    }
    sum
}

pub fn remove_duplicates(x: Vec<i32>) -> Vec<i32> {
    let mut y: Vec<i32> = vec![];

    for val in x.iter() {
        if y.contains(val) {
            continue;
        } else {
            y.push(*val)
        }
    }

    y
}

pub fn sum_even_numbers(v: Vec<i32>) -> i32 {
    v.iter().filter(|x| *x % 2 == 0).sum()
}

pub fn reverse_strings(v: Vec<String>) -> Vec<String> {
    v.iter().map(|s| s.chars().rev().collect()).collect()
}

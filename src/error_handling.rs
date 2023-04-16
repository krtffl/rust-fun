use std::io::Empty;

pub fn first_character(x: String) -> Option<char> {
    x.chars().rev().last()
}

pub fn find_char(s: String, x: char) -> Option<usize> {
    s.chars().enumerate().find(|c| c.1 == x).map(|c| c.0)
}

#[derive(Debug)]
pub enum ParseNameError {
    Empty,
    TooLong,
}

pub fn lenght_check(s: String) -> Result<String, ParseNameError> {
    if s.is_empty() {
        return Err(ParseNameError::Empty);
    } else if s.len() > 10 {
        return Err(ParseNameError::TooLong);
    }

    Ok(s)
}

pub fn first_character_result(x: String) -> Result<char, String> {
    let first_char = x.chars().rev().last();
    match first_char {
        None => Err("input string is empty".to_string()),
        Some(c) => Ok(c),
    }
}

pub fn find_char_result(s: String, x: char) -> Result<usize, String> {
    let index = s.chars().enumerate().find(|c| c.1 == x).map(|c| c.0);
    match index {
        None => Err("character not found".to_string()),
        Some(i) => Ok(i),
    }
}

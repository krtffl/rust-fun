use std::f64::consts::PI;

// structs
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn distance_to_origin(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

pub struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn is_square(&self) -> bool {
        self.width == self.height
    }

    fn area(&self) -> f64 {
        self.width * self.height
    }
}

// enums
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64),
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Self::Rectangle(w, h) => w * h,
            Self::Circle(r) => PI * r * r,
            Self::Triangle(b, h) => b * h / 2 as f64,
        }
    }
}

pub fn divide(x: f64, y: f64) -> Option<f64> {
    if y != 0 as f64 {
        Some(x / y)
    } else {
        None
    }
}

pub fn is_prime(x: i32) -> Result<bool, String> {
    if x <= 1 {
        return Err("input is not valid".to_string());
    }
    for i in 2..((x as f64).sqrt() as i32 + 1) {
        if x % i == 0 {
            return Ok(false);
        }
    }
    Ok(true)
}

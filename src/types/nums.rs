use std::{
    fmt::Display,
    ops::{Add, Div, Mul},
    str::FromStr,
};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct Value(i32); // Store 12.3 as 123

impl Value {
    pub fn to_float(&self) -> f32 {
        self.0 as f32 / 10.0
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Value((value.parse::<f64>().unwrap() * 10.0) as i32)
    }
}

impl FromStr for Value {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let val = s.parse::<f64>();
        if let Ok(v) = val {
            Ok(Value((v) as i32))
        } else {
            Err(format!("{} is not a valid number", s))
        }
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value((value * 10.0) as i32)
    }
}

impl Mul for Value {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Value(self.0 * rhs.0)
    }
}

impl Mul<i32> for Value {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Value(self.0 * rhs)
    }
}

impl Div<i32> for Value {
    type Output = Self;

    fn div(self, rhs: i32) -> Self::Output {
        Value(self.0 / rhs)
    }
}

impl Add for Value {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Value(self.0 + rhs.0)
    }
}

impl Add<i32> for Value {
    type Output = Self;

    fn add(self, rhs: i32) -> Self::Output {
        Value(self.0 + rhs)
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

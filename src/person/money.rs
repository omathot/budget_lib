use serde::{Deserialize, Serialize};
use std::{
    fmt,
    ops::{Add, Div, Mul, Sub},
    str::FromStr,
};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Money(f32);

impl Money {
    pub fn new(amount: f32) -> Self {
        Money((amount * 100.0).round() / 100.0)
    }
    pub fn value(&self) -> f32 {
        self.0
    }
    pub fn edit_amount(&mut self, amount: f32) {
        self.0 = amount;
    }
    pub fn round(&self) -> Self {
        Money((self.0 * 100.0).round() / 100.0)
    }
}

impl Add for Money {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Money::new(self.0 + other.0)
    }
}

impl Sub for Money {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Money::new(self.0 - other.0)
    }
}

impl Mul for Money {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Money::new(self.0 * other.0)
    }
}

impl Div for Money {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        Money::new(self.0 / other.0)
    }
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "${:.2}", self.0)
    }
}

impl FromStr for Money {
    type Err = std::num::ParseFloatError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let without_dollar = s.trim_start_matches('$');
        let amount = without_dollar.parse::<f32>()?;
        Ok(Money::new(amount))
    }
}

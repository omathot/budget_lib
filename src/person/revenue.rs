use chrono::{Local, NaiveDate};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::Money;

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Revenue {
    name: String,
    is_recurring: bool,
    amount: Money,
    #[serde(with = "NaiveDate")]
    time: NaiveDate,
}

#[derive(Debug, Error)]
pub enum RevenueError {
    #[error("Revenue name cannot be empty")]
    EmptyName,
    #[error("Revenue amount cannot be negative")]
    NegativeValue,
}

impl Revenue {
    pub fn new(amount: f32, is_recurring: bool, name: &str) -> Result<Revenue, RevenueError> {
        if amount.is_sign_negative() {
            return Err(RevenueError::NegativeValue);
        }
        if name.is_empty() {
            return Err(RevenueError::EmptyName);
        }
        let now = Local::now().date_naive();
        Ok(Revenue {
            name: name.to_string(),
            is_recurring,
            amount: Money::new(amount),
            time: now,
        })
    }
    pub fn edit_name(&mut self, name: &str) -> Result<(), RevenueError> {
        if name.is_empty() {
            return Err(RevenueError::EmptyName);
        }
        self.name = name.to_string();
        Ok(())
    }
    pub fn edit_amount(&mut self, amount: f32) -> Result<(), RevenueError> {
        if amount.is_sign_negative() {
            return Err(RevenueError::NegativeValue);
        }
        self.amount.edit_amount(amount);
        Ok(())
    }
    pub fn set_recurring(&mut self, is_recurring: bool) {
        self.is_recurring = is_recurring;
    }

    // getters
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn amount(&self) -> f32 {
        self.amount.value()
    }
    pub fn time(&self) -> &NaiveDate {
        &self.time
    }
    pub fn recurring(&self) -> bool {
        self.is_recurring
    }
}

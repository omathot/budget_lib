use chrono::{Local, NaiveDate};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::Money;

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Expense {
    name: String,
    amount: Money,
    is_recurring: bool,
    #[serde(with = "NaiveDate")]
    time: NaiveDate,
}

#[derive(Debug, Error)]
pub enum ExpenseError {
    #[error("Expense name cannot be empty")]
    EmptyName,
    #[error("Expense amount cannot be negative")]
    NegativeValue,
}

impl Expense {
    pub fn new(amount: f32, name: &str, is_recurring: bool) -> Result<Self, ExpenseError> {
        if name.is_empty() {
            return Err(ExpenseError::EmptyName);
        }
        if amount.is_sign_negative() {
            return Err(ExpenseError::NegativeValue);
        }
        let now = Local::now().date_naive();
        Ok(Expense {
            name: name.to_string(),
            amount: Money::new(amount),
            is_recurring,
            time: now,
        })
    }
    pub fn edit_amount(&mut self, amount: f32) -> Result<(), ExpenseError> {
        if amount.is_sign_negative() {
            return Err(ExpenseError::NegativeValue);
        }
        self.amount.edit_amount(amount);
        Ok(())
    }
    pub fn edit_name(&mut self, name: &str) -> Result<(), ExpenseError> {
        if name.is_empty() {
            return Err(ExpenseError::EmptyName);
        }
        self.name = name.to_string();
        Ok(())
    }
    /// returns inverse of current
    pub fn set_recurring(&mut self, is_recurring: bool) {
        self.is_recurring = is_recurring;
    }
    pub fn edit_time(&mut self, time: NaiveDate) {
        self.time = time;
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

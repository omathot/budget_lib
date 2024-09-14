pub use crate::person::{Expense, Revenue};
use chrono::{Datelike, Local};
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize, Default)]
pub struct Budget {
    expenses: Vec<Expense>,
    revenues: Vec<Revenue>,
}

impl Budget {
    /// Returns an empty Budget, to be manually filled it.
    pub fn new() -> Self {
        Budget {
            expenses: Vec::new(),
            revenues: Vec::new(),
        }
    }
    /// Builds a budget given the inputs.
    pub fn build(expenses: Vec<Expense>, revenues: Vec<Revenue>) -> Self {
        Budget { expenses, revenues }
    }
    /// Create a new expense given a name and amount
    pub fn new_expense(&mut self, name: &str, amount: f32) {
        if let Ok(expense) = Expense::new(amount, name, false) {
            self.expenses.push(expense);
        }
    }
    /// Create a new revenue given a name and amount
    pub fn new_revenue(&mut self, name: &str, amount: f32) {
        if let Ok(revenue) = Revenue::new(amount, false, name) {
            self.revenues.push(revenue);
        }
    }
    pub fn remove_expense(&mut self, idx: usize) {
        self.expenses.remove(idx);
    }
    pub fn remove_revenue(&mut self, idx: usize) {
        self.revenues.remove(idx);
    }
    pub fn current_monthly_saving(&self) -> f32 {
        let curr_month = Local::now().month();
        let curr_month_expenses = self.get_month_expense(curr_month);
        let curr_month_revenues = self.get_month_revenue(curr_month);

        println!("curr_month_expense = {:?}", curr_month_expenses);
        println!("curr_month_revenues = {:?}", curr_month_revenues);
        // curr_month_expenses.iter();
        let expense_amount: f32 = curr_month_expenses
            .map(|expenses| {
                expenses.iter().fold(0.0, |acc, expense| {
                    println!("expense = {}, adding {}", acc, expense.amount());
                    acc + expense.amount()
                })
            })
            .unwrap_or(0.0);
        let revenue_amount: f32 = curr_month_revenues
            .map(|revenue| {
                revenue.iter().fold(0.0, |acc, revenue| {
                    println!("revenue = {}, adding {}", acc, revenue.amount());
                    acc + revenue.amount()
                })
            })
            .unwrap_or(0.0);
        println!(
            "revenue - expense = {} - {}",
            revenue_amount, expense_amount
        );
        revenue_amount - expense_amount
    }

    // getters
    pub fn expenses(&self) -> &Vec<Expense> {
        &self.expenses
    }
    pub fn revenues(&self) -> &Vec<Revenue> {
        &self.revenues
    }
    /// Returns an `Option` containing `None` if no expenses for that month, or a `Vec<&Expense>` if there are expenses for that month.
    /// the `month` variable takes in values from 1 to 12, with 1 being January, and 12 being December.
    pub fn get_month_expense(&self, month: u32) -> Option<Vec<&Expense>> {
        if (1..=12).contains(&month) {
            let expenses: Vec<&Expense> = self
                .expenses
                .iter()
                .filter(|e| e.time().month() == month)
                .collect();
            if expenses.is_empty() {
                None
            } else {
                Some(expenses)
            }
        } else {
            eprintln!("Wrong usage of get_month_expense(), 1 <= month <= 12");
            None
        }
    }
    pub fn get_month_revenue(&self, month: u32) -> Option<Vec<&Revenue>> {
        if (1..=12).contains(&month) {
            let revenues: Vec<&Revenue> = self
                .revenues
                .iter()
                .filter(|r| r.time().month() == month)
                .collect();
            if revenues.is_empty() {
                None
            } else {
                Some(revenues)
            }
        } else {
            eprintln!("Wrong usage of get_month_revenge(), 1 <= month <= 12");
            None
        }
    }
}

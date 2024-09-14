pub mod person;
use person::Budget;

struct Person {
    budget: Budget,
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;
    use person::{expense, Expense, Money, Revenue};

    use super::*;

    #[test]
    fn create_new_budget() {
        let budget = Budget::new();
        assert_eq!(Budget::new(), budget);
    }

    #[test]
    fn build_budget() {
        let mut temp = Budget::new();
        temp.new_expense("test1", 1.0);
        temp.new_revenue("test2", 2.0);
        let budget = Budget::build(temp.expenses().to_owned(), temp.revenues().to_owned());
        assert_eq!(1.0, budget.expenses().first().unwrap().amount());
    }

    #[test]
    fn add_new_expense() {
        let mut budget = Budget::new();
        budget.new_expense("Dinner", 25.0);
        assert_eq!(25.0, budget.expenses().first().unwrap().amount());
    }

    #[test]
    fn add_new_revenue() {
        let mut budget = Budget::new();
        budget.new_revenue("work", 120.0);
        assert_eq!(120.0, budget.revenues().first().unwrap().amount());
    }

    #[test]
    fn remove_revenue() {
        let mut budget = Budget::new();
        budget.new_revenue("to_remove", 1.0);
        assert_eq!(1.0, budget.revenues().first().unwrap().amount());
        budget.remove_revenue(0);
        assert!(budget.revenues().first().is_none());
    }

    #[test]
    fn get_current_month_revenues() {
        let mut budget = Budget::new();
        for i in 0..10 {
            let x = i as f32;
            budget.new_revenue(x.to_string().as_str(), x);
        }
        assert_eq!(0.0, budget.revenues().first().unwrap().amount());
    }

    #[test]
    fn calculate_monthly_saving() {
        let mut budget = Budget::new();
        for _ in 0..10 {
            budget.new_expense("test", 5.0);
            budget.new_revenue("test2", 10.0);
        }
        assert_eq!(50.0, budget.current_monthly_saving());
    }

    #[test]
    fn calculate_diff_between_last_and_current_month_saving() {
        let mut budget = Budget::new();
        for i in 0..20 {
            let x = i as f32;
            budget.new_expense("expense", x);
            budget.new_revenue("revenue", x);
        }
        let mut day = 255; // september 12th

        for expense in budget.expenses().iter_mut().take(12) {
            if let Some(date) = NaiveDate::from_yo_opt(2024, day) {
                expense.edit_time(date);
                day -= 1;
            }
        }
    }

    #[test]
    fn get_current_month_expenses() {
        let mut budget = Budget::new();
        for i in 0..10 {
            let x = i as f32;
            budget.new_expense(x.to_string().as_str(), x);
        }
        assert_eq!(5.0, budget.expenses().get(5).unwrap().amount());
    }

    #[test]
    fn test_money_operations() {
        let m = Money::new(12.0);
        let m2 = Money::new(12.0);

        assert_eq!(Money::new(144.0), m * m2);
        assert_eq!(Money::new(24.0), m + m2);
        assert_eq!(Money::new(0.0), m - m2);
        assert_eq!(Money::new(1.0), m / m2);
    }

    #[test]
    fn test_money_from_str() {
        assert_eq!(Ok(Money::new(10.25)), "$10.25".parse());
        assert_eq!(Ok(Money::new(10.25)), "10.25".parse());
    }
}

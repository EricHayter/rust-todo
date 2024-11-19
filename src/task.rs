use chrono::{self, Days, Local, NaiveDate };

struct Task {
    title: String,
    due_date: NaiveDate,
    is_complete: bool,
}

impl Default for Task {
    fn default() -> Self {
        let current_date = Local::now().naive_local().date();
        let due_date = current_date.checked_add_days(Days::new(1))
            .unwrap_or(current_date);

        Task { 
            title: "task".to_string(), 
            due_date: due_date, 
            is_complete: false 
        }
    }
}

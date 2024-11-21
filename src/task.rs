use std::fmt;
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

// [Due Date] Title [X] or [ ]
impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.is_complete {
            true => write!(f, "[{}] {}, [X]", self.due_date, self.title),
            false => write!(f, "[{}] {}, [ ]", self.due_date, self.title)
        }
    }
}

use std::fmt;

use chrono::{Local, NaiveDate};

pub struct Task {
    title: String,
    creation_date: NaiveDate,
    is_complete: bool,
}

impl Task {
    pub fn new(title: &str) -> Self {
        let current_date = Local::now().naive_local().date();
        Task {
            title: title.to_string(),
            creation_date: current_date,
            is_complete: false,
        }
    }
}

impl Default for Task {
    fn default() -> Self {
        let current_date = Local::now().naive_local().date();
        Task {
            title: "task".to_string(),
            creation_date: current_date,
            is_complete: false,
        }
    }
}

// [Due Date] Title [X] or [ ]
impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.is_complete {
            true => write!(f, "[{}] {} [X]", self.creation_date, self.title),
            false => write!(f, "[{}] {} [ ]", self.creation_date, self.title),
        }
    }
}

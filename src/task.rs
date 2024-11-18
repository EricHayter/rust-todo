use chrono::{self, Local, NaiveDate};

struct Task {
    title: &str,
    due_date: chrono::NaiveDate,
    is_complete: bool,
}

impl Default for Task {
    fn default() -> Self {
        let now = Local::now();
        let date: NaiveDate::from_ymd(now::year)
        Task { title: "task", due_date: chrono::NaiveDate::fr, is_complete: false }
    }
}

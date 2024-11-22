mod task;

use crate::task::Task;

fn main() {
    println!("Hello, world!");
    let t = Task::default();
    let t2 = Task::new("Read some more of my book");
    println!("{t}");
    println!("{}", t2);
}

mod task;

fn main() {
    println!("Hello, world!");
    let t = Task::default();
    println!(t.to_string());
}

mod task;

use crate::task::Task;

fn main() {
    todo_app();
}

fn todo_app() {
    println!(
        "{}",
        Task::of(1, "test".to_string(), "あいうえお".to_string()).show()
    );
}

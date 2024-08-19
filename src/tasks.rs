use crate::task::Task;

struct Tasks {
    // フィールドの定義
    tasks: Vec<Task>,
}

impl Tasks {
    pub fn show(&self) -> String {
        self.tasks
            .iter()
            .map(|task| task.show())
            .collect::<Vec<String>>()
            .join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn タスクの一覧を表示する() {
        let task1 = Task::new(String::from("test"), String::from("あいうえお"));
        let mut task2 = Task::new(String::from("test2"), String::from("かきくけこ"));
        task2.done();
        let tasks = Tasks { tasks: vec![task1, task2] };
        assert_eq!(
            "id: 0, status: 未完了, name: test, description: あいうえお\nid: 0, status: 完了, name: test2, description: かきくけこ",
            tasks.show()
        );
    }
}
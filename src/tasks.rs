use crate::task::Task;

pub struct Tasks {
    // フィールドの定義
    tasks: Vec<Task>,
}

impl Tasks {
    /// コンストラクタ
    pub fn new() -> Self {
        Self { tasks: Vec::new() }
    }

    /// タスク一覧を表示する
    pub fn show(&self) -> String {
        // タスクがない場合はテキストを返す
        if self.tasks.is_empty() {
            return "there is no task.".to_string();
        }
        self.tasks
            .iter()
            .map(|task| task.show())
            .collect::<Vec<String>>()
            .join("\n")
    }

    /// タスクを追加する
    pub fn add(&mut self, task: Task) {
        self.tasks.push(task);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn タスクがない場合はテキストを返す() {
        let tasks = Tasks::new();
        assert_eq!("there is no task.", tasks.show());
    }

    #[test]
    fn タスクの一覧を表示する() {
        let mut tasks = Tasks::new();
        let task1 = Task::new(
            0,
            "未完了".to_string(),
            String::from("test"),
            String::from("あいうえお"),
        );
        let task2 = Task::new(
            0,
            "完了".to_string(),
            String::from("test2"),
            String::from("かきくけこ"),
        );
        tasks.add(task1);
        tasks.add(task2);
        assert_eq!(
            "id: 0, status: 未完了, name: test, description: あいうえお\nid: 0, status: 完了, name: test2, description: かきくけこ",
            tasks.show()
        );
    }
}

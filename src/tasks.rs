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
    fn add(&mut self, task: Task) {
        self.tasks.push(task);
    }

    /// タスクを新規作成する
    pub fn create(&mut self, name: String, description: String) {
        let id = self.tasks.len() as u32 + 1;
        let task = Task::new(id, "未完了".to_string(), name, description);
        self.add(task);
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
        tasks.create("test".to_string(), "あいうえお".to_string());
        tasks.create("test2".to_string(), "かきくけこ".to_string());
        assert_eq!(
            "id: 1, status: 未完了, name: test, description: あいうえお\nid: 2, status: 未完了, name: test2, description: かきくけこ",
            tasks.show()
        );
    }
}

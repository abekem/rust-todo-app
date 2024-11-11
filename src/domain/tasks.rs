use crate::domain::task::Task;

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

    /// タスクを新規作成する
    pub fn create(&mut self, name: String, description: String) {
        let id = self.tasks.len() as u32 + 1;
        let task = Task::new(id, name, description);
        self.tasks.push(task);
    }

    /// タスクを取得する
    pub fn find(&mut self, id: u32) -> Option<&mut Task> {
        self.tasks.iter_mut().find(|task| task.is_same_id(id))
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

    #[test]
    fn タスクを完了させる() {
        let mut tasks = Tasks::new();
        tasks.create("test".to_string(), "あいうえお".to_string());
        let task = tasks.find(1).unwrap();
        task.done();
        assert_eq!(
            "id: 1, status: 完了, name: test, description: あいうえお",
            tasks.show()
        );
    }
}

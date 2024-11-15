use crate::domain::task::Task;

#[derive(Clone)]
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

    /// タスク配列にタスクを追加する
    pub fn push(&mut self, task: Task) -> Result<(), String> {
        // IDが重複している場合はエラー
        if self.tasks.iter().any(|t| t.is_same_id(task.get_id())) {
            return Err("ID is duplicated.".to_string());
        }
        self.tasks.push(task);
        Ok(())
    }

    /// タスクのイテレータを取得する
    pub fn iter_mut(&mut self) -> std::slice::IterMut<Task> {
        self.tasks.iter_mut()
    }

    /// タスク一覧から削除する
    pub fn delete(&mut self, id: u32) {
        self.tasks.retain(|t| !t.is_same_id(id));
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
        let tasks = Tasks {
            tasks: vec![
                Task::create(1, "test".to_string(), "あいうえお".to_string()),
                Task::create(2, "test2".to_string(), "かきくけこ".to_string()),
            ],
        };
        assert_eq!(
            "id: 1, status: 未完了, name: test, description: あいうえお\nid: 2, status: 未完了, name: test2, description: かきくけこ",
            tasks.show()
        );
    }

    #[test]
    fn タスク一覧から削除する() {
        let mut tasks = Tasks {
            tasks: vec![
                Task::create(1, "test".to_string(), "あいうえお".to_string()),
                Task::create(2, "test2".to_string(), "かきくけこ".to_string()),
            ],
        };
        tasks.delete(1);
        assert_eq!(
            "id: 2, status: 未完了, name: test2, description: かきくけこ",
            tasks.show()
        );
    }

    #[test]
    fn タスクを追加する() {
        let mut tasks = Tasks::new();
        tasks
            .push(Task::create(
                1,
                "test".to_string(),
                "あいうえお".to_string(),
            ))
            .unwrap();
        assert_eq!(
            "id: 1, status: 未完了, name: test, description: あいうえお",
            tasks.show()
        );
    }

    #[test]
    fn idが重複している場合はエラーを返す() {
        let mut tasks = Tasks {
            tasks: vec![Task::create(
                1,
                "test".to_string(),
                "あいうえお".to_string(),
            )],
        };
        assert_eq!(
            Err("ID is duplicated.".to_string()),
            tasks.push(Task::create(
                1,
                "test2".to_string(),
                "かきくけこ".to_string()
            ))
        );
    }
}

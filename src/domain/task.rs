use strum::Display;

/// タスクを表す構造体
#[derive(Debug, Eq, Clone)]
pub struct Task {
    /// ID
    id: u32,
    /// ステータス
    status: Status,
    /// 名前
    name: String,
    /// 説明
    description: String,
}

/// タスクのステータス
#[derive(Debug, Eq, PartialEq, Display, Clone)]
enum Status {
    #[strum(to_string = "未完了")]
    Todo,
    #[strum(to_string = "完了")]
    Done,
}

impl Task {
    /// タスクを表示する
    pub fn show(&self) -> String {
        format!(
            "id: {}, status: {}, name: {}, description: {}",
            self.id, self.status, self.name, self.description
        )
    }

    /// タスクを新規作成する
    fn new(id: u32, status: Status, name: String, description: String) -> Self {
        Self {
            id,
            status,
            name,
            description,
        }
    }

    /// 初期状態のタスクを新規作成する
    pub fn create(id: u32, name: String, description: String) -> Self {
        Task::new(id, Status::Todo, name, description)
    }

    /// タスクを完了する
    pub fn done(&mut self) {
        self.status = Status::Done;
    }

    /// タスクのIDが一致するか判定する
    pub fn is_same_id(&self, id: u32) -> bool {
        self.id == id
    }

    /// タスクの名前を更新する
    pub fn update_name(&mut self, name: String) {
        self.name = name;
    }

    /// タスクの説明を更新する
    pub fn update_description(&mut self, description: String) {
        self.description = description;
    }

    /// タスクのIDを取得する
    pub fn get_id(&self) -> u32 {
        self.id
    }
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn タスクを新規作成する() {
        assert_eq!(
            "id: 0, status: 未完了, name: test, description: あいうえお",
            Task::create(0, String::from("test"), String::from("あいうえお")).show()
        );
    }

    #[test]
    fn タスクを完了する() {
        let mut task = Task::create(0, String::from("test"), String::from("あいうえお"));
        task.done();
        assert_eq!(
            "id: 0, status: 完了, name: test, description: あいうえお",
            task.show()
        );
    }

    #[test]
    fn idが一致するタスクは等しい() {
        let task1 = Task::create(1, String::from("abc"), String::from("def"));
        let task2 = Task::create(1, String::from("ghi"), String::from("jkl"));
        assert_eq!(task1, task2);
    }

    #[test]
    fn idが一致しないタスクは等しくない() {
        let task1 = Task::create(1, String::from("abc"), String::from("def"));
        let task2 = Task::create(2, String::from("abc"), String::from("def"));
        assert_ne!(task1, task2);
    }

    #[test]
    fn タスクの名前を更新する() {
        let mut task = Task::create(0, String::from("test"), String::from("あいうえお"));
        task.update_name(String::from("test2"));
        assert_eq!(
            "id: 0, status: 未完了, name: test2, description: あいうえお",
            task.show()
        );
    }

    #[test]
    fn タスクの説明を更新する() {
        let mut task = Task::create(0, String::from("test"), String::from("あいうえお"));
        task.update_description(String::from("かきくけこ"));
        assert_eq!(
            "id: 0, status: 未完了, name: test, description: かきくけこ",
            task.show()
        );
    }
}

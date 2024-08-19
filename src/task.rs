/// タスクを表す構造体
#[derive(Debug, Eq)]
pub struct Task {
    /// ID
    id: i32,
    /// ステータス
    status: String,
    /// 名前
    name: String,
    /// 説明
    description: String,
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
    pub fn new(name: String, description: String) -> Self {
        Self {
            id: 0,
            status: String::from("未完了"),
            name,
            description,
        }
    }
    
    /// タスクを完了する
    pub fn done(&mut self) {
        self.status = String::from("完了");
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
    fn タスクを表示する() {
        assert_eq!(
            "id: 1, status: 未完了, name: test, description: あいうえお",
            Task {
                id: 1,
                status: String::from("未完了"),
                name: String::from("test"),
                description: String::from("あいうえお")
            }
            .show()
        );
    }
    
    #[test]
    fn タスクを新規作成する() {
        assert_eq!(
            "id: 0, status: 未完了, name: test, description: あいうえお",
            Task::new(String::from("test"), String::from("あいうえお")).show()
        );
    }
    
    #[test]
    fn タスクを完了する() {
        let mut task = Task::new(String::from("test"), String::from("あいうえお"));
        task.done();
        assert_eq!(
            "id: 0, status: 完了, name: test, description: あいうえお",
            task.show()
        );
    }
    
    #[test]
    fn idが一致するタスクは等しい() {
        let task1 = Task {
            id: 1,
            status: String::from("未完了"),
            name: String::from("abc"),
            description: String::from("def"),
        };
        let task2 = Task {
            id: 1,
            status: String::from("完了"),
            name: String::from("ghi"),
            description: String::from("jkl"),
        };
        assert_eq!(
            task1, task2
        );
    }

    #[test]
    fn idが一致しないタスクは等しくない() {
        let task1 = Task {
            id: 1,
            status: String::from("完了"),
            name: String::from("abc"),
            description: String::from("def"),
        };
        let task2 = Task {
            id: 2,
            status: String::from("完了"),
            name: String::from("abc"),
            description: String::from("def"),
        };
        assert_ne!(
            task1, task2
        );
    }
}

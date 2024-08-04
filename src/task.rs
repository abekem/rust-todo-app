/// タスクを表す構造体
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
}

pub struct Task {
    id: i32,
    status: String,
    name: String,
    description: String,
}

impl Task {
    pub fn show(&self) -> String {
        format!(
            "id: {}, status: {}, name: {}, description: {}",
            self.id, self.status, self.name, self.description
        )
    }

    pub fn of(id: i32, status: String, name: String, description: String) -> Self {
        Self {
            id,
            status,
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
                status: "未完了".to_string(),
                name: "test".to_string(),
                description: "あいうえお".to_string()
            }
            .show()
        );
    }
}

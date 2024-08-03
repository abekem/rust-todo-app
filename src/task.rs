pub struct Task {
    id: i32,
    name: String,
    description: String,
}

impl Task {
    pub fn show(&self) -> String {
        format!(
            "id: {}, name: {}, description: {}",
            self.id, self.name, self.description
        )
    }

    pub fn of(id: i32, name: String, description: String) -> Self {
        Self {
            id,
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
            "id: 1, name: test, description: あいうえお",
            Task {
                id: 1,
                name: "test".to_string(),
                description: "あいうえお".to_string()
            }
            .show()
        );
    }
}

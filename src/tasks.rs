use crate::task::Task;

pub struct Tasks {
    // フィールドの定義
    tasks: Vec<Task>,
}

impl Tasks {
    // コンストラクタ
    fn new(tasks: Vec<Task>) -> Self {
        Self { tasks }
    }

    // タスク一覧を表示する
    pub fn show(&self) -> String {
        self.tasks
            .iter()
            .map(|task| task.show())
            .collect::<Vec<String>>()
            .join("\n")
    }

    // タスク一覧を返す
    pub fn list() -> Tasks {
        // 初期化したTasks構造体を返す
        let task_a = Task::new(
            1,
            "未完了".to_string(),
            "A".to_string(),
            "description of task A".to_string(),
        );
        let task_b = Task::new(
            2,
            "実施中".to_string(),
            "B".to_string(),
            "description of task B".to_string(),
        );

        Tasks::new(vec![task_a, task_b])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn タスクの一覧を表示する() {
        let task1 = Task::new(
            0,
            "未完了".to_string(),
            String::from("test"),
            String::from("あいうえお"),
        );
        let mut task2 = Task::new(
            0,
            "未完了".to_string(),
            String::from("test2"),
            String::from("かきくけこ"),
        );
        task2.done();
        let tasks = Tasks {
            tasks: vec![task1, task2],
        };
        assert_eq!(
            "id: 0, status: 未完了, name: test, description: あいうえお\nid: 0, status: 完了, name: test2, description: かきくけこ",
            tasks.show()
        );
    }
}

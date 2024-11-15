use crate::domain::task::Task;
use crate::domain::task_repository_trait::TaskRepositoryTrait;
use crate::domain::tasks::Tasks;

/// インメモリのタスクリポジトリ
pub struct InMemoryTaskRepository {
    tasks: Tasks,
    id_sequence: u32,
}

impl TaskRepositoryTrait for InMemoryTaskRepository {
    fn all(&self) -> Tasks {
        self.tasks.clone()
    }

    fn find(&mut self, id: u32) -> Option<&mut Task> {
        self.tasks.iter_mut().find(|task| task.is_same_id(id))
    }

    fn create(&mut self, name: String, description: String) -> Result<(), String> {
        self.id_sequence += 1;
        let task = Task::create(self.id_sequence, name, description);
        self.tasks.push(task)
    }

    fn update(&mut self, id: u32, name: String, description: String) {
        let task = self.find(id).unwrap();
        task.update_name(name);
        task.update_description(description);
    }

    fn delete(&mut self, id: u32) {
        self.tasks.delete(id);
    }
}

impl InMemoryTaskRepository {
    pub fn new() -> Self {
        Self {
            tasks: Tasks::new(),
            id_sequence: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn タスクがない場合はテキストを返す() {
        let repository = InMemoryTaskRepository::new();
        assert_eq!("there is no task.", repository.all().show());
    }

    #[test]
    fn タスクの一覧を表示する() {
        let mut repository = InMemoryTaskRepository::new();
        repository
            .create("test".to_string(), "あいうえお".to_string())
            .unwrap();
        repository
            .create("test2".to_string(), "かきくけこ".to_string())
            .unwrap();
        assert_eq!(
            "id: 1, status: 未完了, name: test, description: あいうえお\nid: 2, status: 未完了, name: test2, description: かきくけこ",
            repository.all().show()
        );
    }

    #[test]
    fn タスクを完了させる() {
        let mut repository = InMemoryTaskRepository::new();
        repository
            .create("test".to_string(), "あいうえお".to_string())
            .unwrap();
        let task = repository.find(1).unwrap();
        task.done();
        assert_eq!(
            "id: 1, status: 完了, name: test, description: あいうえお",
            repository.all().show()
        );
    }

    #[test]
    fn タスクを更新する() {
        let mut repository = InMemoryTaskRepository::new();
        repository
            .create("test".to_string(), "あいうえお".to_string())
            .unwrap();
        repository.update(1, "test2".to_string(), "かきくけこ".to_string());
        assert_eq!(
            "id: 1, status: 未完了, name: test2, description: かきくけこ",
            repository.all().show()
        );
    }

    #[test]
    fn タスクを削除する() {
        let mut repository = InMemoryTaskRepository::new();
        repository
            .create("test".to_string(), "あいうえお".to_string())
            .unwrap();
        repository
            .create("test2".to_string(), "かきくけこ".to_string())
            .unwrap();
        repository.delete(1);
        repository
            .create("test2".to_string(), "かきくけこ".to_string())
            .unwrap();
        assert_eq!(
            "id: 2, status: 未完了, name: test2, description: かきくけこ\nid: 3, status: 未完了, name: test2, description: かきくけこ",
            repository.all().show()
        );
    }
}

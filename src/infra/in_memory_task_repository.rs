use crate::domain::task::Task;
use crate::domain::task_repository_trait::TaskRepositoryTrait;
use crate::domain::tasks::Tasks;

pub struct InMemoryTaskRepository {
    tasks: Tasks,
}

impl TaskRepositoryTrait for InMemoryTaskRepository {
    fn create(&mut self, name: String, description: String) {
        self.tasks.create(name, description);
    }

    fn find(&mut self, id: u32) -> Option<&mut Task> {
        self.tasks.find(id)
    }

    fn all(&self) -> Tasks {
        self.tasks.clone()
    }
}

impl InMemoryTaskRepository {
    pub fn new() -> Self {
        Self {
            tasks: Tasks::new(),
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
        repository.create("test".to_string(), "あいうえお".to_string());
        repository.create("test2".to_string(), "かきくけこ".to_string());
        assert_eq!(
            "id: 1, status: 未完了, name: test, description: あいうえお\nid: 2, status: 未完了, name: test2, description: かきくけこ",
            repository.all().show()
        );
    }

    #[test]
    fn タスクを完了させる() {
        let mut repository = InMemoryTaskRepository::new();
        repository.create("test".to_string(), "あいうえお".to_string());
        let task = repository.find(1).unwrap();
        task.done();
        assert_eq!(
            "id: 1, status: 完了, name: test, description: あいうえお",
            repository.all().show()
        );
    }
}

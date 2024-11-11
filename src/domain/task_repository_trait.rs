use crate::domain::task::Task;
use crate::domain::tasks::Tasks;

pub trait TaskRepositoryTrait {
    /// タスクを作成する
    fn create(&mut self, name: String, description: String);
    /// タスクを取得する
    fn find(&mut self, id: u32) -> Option<&mut Task>;
    /// タスクの一覧を返す
    fn all(&self) -> Tasks;
}
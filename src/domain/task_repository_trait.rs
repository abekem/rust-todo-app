use crate::domain::task::Task;
use crate::domain::tasks::Tasks;

pub trait TaskRepositoryTrait {
    /// タスクの一覧を返す
    fn all(&self) -> Tasks;
    /// タスクを取得する
    fn find(&mut self, id: u32) -> Option<&mut Task>;
    /// タスクを作成する
    fn create(&mut self, name: String, description: String);
    /// タスクを更新する
    fn update(&mut self, id: u32, name: String, description: String);
    /// タスクを削除する
    fn delete(&mut self, id: u32);
}

use core::str;
use strum::{Display, EnumIter, EnumString, IntoEnumIterator, IntoStaticStr};

mod domain;
mod infra;

use domain::task_repository_trait::TaskRepositoryTrait;

/// コマンド
#[derive(Debug, PartialEq, Clone, Copy, EnumIter, Display, EnumString, IntoStaticStr)]
pub enum Command {
    #[strum(serialize = "list")]
    List,
    #[strum(serialize = "add")]
    Add,
    #[strum(serialize = "update")]
    Update,
    #[strum(serialize = "delete")]
    Delete,
    #[strum(serialize = "done")]
    Done,
    #[strum(serialize = "quit")]
    Quit,
}

impl Command {
    /// コマンドの一覧を表示する
    pub fn get_all() -> Vec<String> {
        Command::iter()
            .map(|c| c.to_string().to_lowercase())
            .collect()
    }

    /// コマンドを実行する
    pub fn execute(&self, task_repository: &mut impl TaskRepositoryTrait) {
        match self {
            Command::List => {
                // タスクの一覧を表示
                println!("{}", task_repository.all().show());
            }
            Command::Add => {
                // タスクを追加
                println!("put task name.");
                let input = read();
                let name = input.trim();
                println!("put task description.");
                let input = read();
                let description = input.trim();
                task_repository.create(name.to_string(), description.to_string());
            }
            Command::Update => {
                // タスクを更新
                println!("put task id.");
                let input = read();
                let id = input.trim().parse::<u32>().unwrap();
                println!("put task name.");
                let input = read();
                let name = input.trim();
                println!("put task description.");
                let input = read();
                let description = input.trim();
                task_repository.update(id, name.to_string(), description.to_string());
            }
            Command::Delete => {
                // タスクを削除
                println!("put task id.");
                let input = read();
                let id = input.trim().parse::<u32>().unwrap();
                task_repository.delete(id);
            }
            Command::Done => {
                // タスクを完了
                println!("put task id.");
                let input = read();
                let id = input.trim().parse::<u32>().unwrap();
                if let Some(task) = task_repository.find(id) {
                    task.done();
                } else {
                    println!("task not found.");
                }
            }
            Command::Quit => {
                // プログラムを終了
                println!("bye.");
                return;
            }
        }
    }
}

fn main() {
    // 利用可能なコマンドの一覧を取得
    let commands = Command::get_all();

    // タスクリポジトリ
    let mut task_repository = infra::in_memory_task_repository::InMemoryTaskRepository::new();

    // 無限ループ
    loop {
        // コマンドの入力を促す
        println!("put command.");
        // 標準入力からコマンドを受け取る
        let input = read();
        let command = input.trim();
        // 文字列をコマンドに変換
        let command = match command.parse::<Command>() {
            Ok(command) => command,
            Err(_) => {
                // // エラーメッセージを表示
                eprintln!("invalid command: {}", command);
                // 利用可能なコマンドの一覧を表示
                println!("available commands: {:?}", commands);
                continue;
            }
        };
        // コマンドを実行
        command.execute(&mut task_repository);
    }
}

/// 入力を受け付ける
fn read() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
}

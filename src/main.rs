use core::str;
use strum::{Display, EnumIter, EnumString, IntoEnumIterator, IntoStaticStr};

mod domain;

/// コマンド
#[derive(Debug, PartialEq, Clone, Copy, EnumIter, Display, EnumString, IntoStaticStr)]
pub enum Command {
    #[strum(serialize = "list")]
    List,
    #[strum(serialize = "add")]
    Add,
    #[strum(serialize = "done")]
    Done,
    #[strum(serialize = "quit")]
    Quit,
}

fn main() {
    // 利用可能なコマンドの一覧をlowercaseで取得
    let commands = Command::iter()
        .map(|c| c.to_string().to_lowercase())
        .collect::<Vec<String>>();
    // タスクの一覧
    let mut tasks = domain::tasks::Tasks::new();

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

        // コマンドによって処理を分岐
        match command {
            Command::List => {
                // タスクの一覧を表示
                println!("{}", tasks.show());
            }
            Command::Add => {
                // タスクを追加
                println!("put task name.");
                let input = read();
                let name = input.trim();
                println!("put task description.");
                let input = read();
                let description = input.trim();
                tasks.create(name.to_string(), description.to_string());
            }
            Command::Done => {
                // タスクを完了
                println!("put task id.");
                let input = read();
                let id = input.trim().parse::<u32>().unwrap();
                if let Some(task) = tasks.find(id) {
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

/// 入力を受け付ける
fn read() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
}

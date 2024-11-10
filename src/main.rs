use core::str;
use strum::{Display, EnumIter, EnumString, IntoEnumIterator, IntoStaticStr};

mod task;
mod tasks;

/// コマンド
#[derive(Debug, PartialEq, Clone, Copy, EnumIter, Display, EnumString, IntoStaticStr)]
pub enum Command {
    #[strum(serialize = "list")]
    List,
    #[strum(serialize = "quit")]
    Quit,
}

fn main() {
    // タスクの一覧
    let tasks = tasks::Tasks::new();

    // 無限ループ
    loop {
        // コマンドの入力を促す
        println!("put command.");
        // 標準入力からコマンドを受け取る
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let command = input.trim();
        // 文字列をコマンドに変換
        let command = match command.parse::<Command>() {
            Ok(command) => command,
            Err(_) => {
                // // エラーメッセージを表示
                eprintln!("invalid command: {}", command);
                // 利用可能なコマンドの一覧を表示
                println!(
                    "available commands: {:?}",
                    Command::iter().collect::<Vec<_>>()
                );
                continue;
            }
        };

        // コマンドによって処理を分岐
        match command {
            // "list"の場合
            Command::List => {
                // タスクの一覧を表示
                println!("{}", tasks.show());
            }
            // "quit"の場合
            Command::Quit => {
                // プログラムを終了
                println!("bye.");
                return;
            }
        }
    }
}

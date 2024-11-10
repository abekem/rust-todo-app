mod task;
mod tasks;

const LIST: &str = "list";
const QUIT: &str = "quit";

fn main() {
    // 利用可能なコマンドの一覧
    let available_commands = vec![LIST, QUIT];

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
        // コマンドによって処理を分岐
        match command {
            // "list"の場合
            LIST => {
                // タスクの一覧を表示
                println!("{}", tasks.show());
            }
            // "quit"の場合
            QUIT => {
                // プログラムを終了
                println!("bye.");
                return;
            }
            // それ以外の場合
            _ => {
                // エラーメッセージを表示
                println!("invalid command: {}", command);
                // 利用可能なコマンドの一覧を表示
                println!("available commands: {:?}", available_commands);
            }
        }
    }
}

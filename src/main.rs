mod task;
mod tasks;

const LIST: &str = "list";

fn main() {
    // 利用可能なコマンドの一覧
    let available_commands = vec![LIST];

    // コマンドライン引数を取得
    let args: Vec<String> = std::env::args().collect();

    // コマンドライン引数の数によって処理を分岐
    match args.len() {
        // 引数が2つの場合
        2 => {
            // 2つ目の引数を取得
            let command = &args[1];
            // コマンドによって処理を分岐
            match command.as_str() {
                // "list"の場合
                LIST => {
                    // タスクの一覧を表示
                    let tasks = tasks::Tasks::list();
                    println!("{}", tasks.show());
                }
                // それ以外の場合
                _ => {
                    // エラーメッセージを表示
                    eprintln!("サポートされていないコマンドです: {}", command);
                }
            }
        }
        // それ以外の場合
        _ => {
            // 利用可能なコマンドの一覧を表示
            println!("利用可能なコマンド: {:?}", available_commands);
        }
    }
}

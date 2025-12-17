// ============================================================================
// Rustエラーハンドリングサンプル
// 公式ドキュメント: https://doc.rust-lang.org/book/ch09-00-error-handling.html
// ============================================================================
//
// Rustのエラー処理は2種類:
// - 回復不能なエラー: panic! マクロ（プログラムを停止）
// - 回復可能なエラー: Result<T, E> 型

use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};

/// panic!による回復不能なエラー
pub fn panic_demo() {
    println!("\n=== panic! マクロ ===");

    // panic!は通常、回復不能なバグに使用
    // panic!("crash and burn"); // これを実行するとプログラムが停止

    // 配列の境界外アクセスもpanicを引き起こす
    let v = vec![1, 2, 3];
    // v[99]; // これはpanicを引き起こす

    println!("panic!はコメントアウトしています（実行するとプログラムが停止）");
    println!("環境変数 RUST_BACKTRACE=1 でバックトレースを表示可能");

    // 配列アクセスの安全な方法
    match v.get(99) {
        Some(value) => println!("値: {}", value),
        None => println!("インデックス99は範囲外です"),
    }
}

/// Result型の基本
pub fn result_basics() {
    println!("\n=== Result型の基本 ===");

    // Result<T, E>の定義:
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }

    // ファイルを開く（存在しない場合エラー）
    let greeting_file_result = File::open("hello.txt");

    let _greeting_file = match greeting_file_result {
        Ok(file) => {
            println!("ファイルを開けました");
            file
        }
        Err(error) => {
            println!("ファイルを開けませんでした: {:?}", error);
            return; // 早期リターン
        }
    };
}

/// エラーの種類によるマッチング
pub fn matching_on_different_errors() {
    println!("\n=== エラーの種類によるマッチング ===");

    let file_result = File::open("hello.txt");

    let _file = match file_result {
        Ok(file) => {
            println!("既存のファイルを開きました");
            file
        }
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                println!("ファイルが見つからないので作成します");
                match File::create("hello.txt") {
                    Ok(fc) => {
                        println!("ファイルを作成しました");
                        fc
                    }
                    Err(e) => {
                        println!("ファイル作成に失敗: {:?}", e);
                        return;
                    }
                }
            }
            other_error => {
                println!("ファイルを開く際にエラー: {:?}", other_error);
                return;
            }
        },
    };

    // クロージャを使ったより簡潔な書き方
    let _file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("ファイル作成に失敗: {:?}", error);
            })
        } else {
            panic!("ファイルを開くのに失敗: {:?}", error);
        }
    });
}

/// unwrapとexpect
pub fn unwrap_and_expect() {
    println!("\n=== unwrapとexpect ===");

    // unwrap: Okなら値を返し、Errならpanic
    // let f = File::open("hello.txt").unwrap();

    // expect: unwrapと同じだがエラーメッセージを指定できる
    // let f = File::open("hello.txt")
    //     .expect("hello.txtを開けるはずです");

    println!("unwrap/expectはエラー時にpanic!するので注意が必要");
    println!("プロトタイプやテストコードでは便利");

    // 安全な代替手段
    let result = File::open("nonexistent.txt");
    if result.is_ok() {
        println!("ファイルが存在します");
    } else {
        println!("ファイルは存在しません");
    }
}

/// ?演算子によるエラー伝播
pub fn error_propagation() {
    println!("\n=== エラー伝播 ===");

    // 長い方法
    fn read_username_from_file_verbose() -> Result<String, io::Error> {
        let username_file_result = File::open("username.txt");

        let mut username_file = match username_file_result {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut username = String::new();

        match username_file.read_to_string(&mut username) {
            Ok(_) => Ok(username),
            Err(e) => Err(e),
        }
    }

    // ?演算子を使った簡潔な方法
    fn read_username_from_file() -> Result<String, io::Error> {
        let mut username_file = File::open("username.txt")?;
        let mut username = String::new();
        username_file.read_to_string(&mut username)?;
        Ok(username)
    }

    // さらに簡潔に（メソッドチェーン）
    fn read_username_from_file_chained() -> Result<String, io::Error> {
        let mut username = String::new();
        File::open("username.txt")?.read_to_string(&mut username)?;
        Ok(username)
    }

    // 最も簡潔（標準ライブラリ関数を使用）
    fn read_username_from_file_shortest() -> Result<String, io::Error> {
        fs::read_to_string("username.txt")
    }

    // デモ実行
    match read_username_from_file_verbose() {
        Ok(name) => println!("verbose: ユーザー名 = {}", name),
        Err(e) => println!("verbose: エラー = {:?}", e),
    }

    match read_username_from_file() {
        Ok(name) => println!("?演算子: ユーザー名 = {}", name),
        Err(e) => println!("?演算子: エラー = {:?}", e),
    }

    match read_username_from_file_chained() {
        Ok(name) => println!("チェーン: ユーザー名 = {}", name),
        Err(e) => println!("チェーン: エラー = {:?}", e),
    }

    match read_username_from_file_shortest() {
        Ok(name) => println!("最短: ユーザー名 = {}", name),
        Err(e) => println!("最短: エラー = {:?}", e),
    }
}

/// Option<T>での?演算子
pub fn question_mark_with_option() {
    println!("\n=== Option<T>での?演算子 ===");

    fn last_char_of_first_line(text: &str) -> Option<char> {
        // ?はNoneの場合に早期リターン
        text.lines().next()?.chars().last()
    }

    let text1 = "Hello\nWorld";
    let text2 = "";

    println!(
        "'{}'の最初の行の最後の文字: {:?}",
        text1,
        last_char_of_first_line(text1)
    );
    println!(
        "'{}'の最初の行の最後の文字: {:?}",
        text2,
        last_char_of_first_line(text2)
    );
}

/// カスタムエラー型
pub fn custom_error_types() {
    println!("\n=== カスタムエラー型 ===");

    // シンプルなカスタムエラー
    #[derive(Debug)]
    enum MathError {
        DivisionByZero,
        NegativeSquareRoot,
    }

    fn divide(a: f64, b: f64) -> Result<f64, MathError> {
        if b == 0.0 {
            Err(MathError::DivisionByZero)
        } else {
            Ok(a / b)
        }
    }

    fn square_root(x: f64) -> Result<f64, MathError> {
        if x < 0.0 {
            Err(MathError::NegativeSquareRoot)
        } else {
            Ok(x.sqrt())
        }
    }

    // 使用例
    match divide(10.0, 2.0) {
        Ok(result) => println!("10 / 2 = {}", result),
        Err(e) => println!("エラー: {:?}", e),
    }

    match divide(10.0, 0.0) {
        Ok(result) => println!("10 / 0 = {}", result),
        Err(e) => println!("エラー: {:?}", e),
    }

    match square_root(-1.0) {
        Ok(result) => println!("sqrt(-1) = {}", result),
        Err(e) => println!("エラー: {:?}", e),
    }
}

/// Result のコンビネータメソッド
pub fn result_combinators() {
    println!("\n=== Resultのコンビネータ ===");

    // map: Okの中身を変換
    let result: Result<i32, &str> = Ok(2);
    let mapped = result.map(|x| x * 2);
    println!("map: {:?} -> {:?}", Ok::<i32, &str>(2), mapped);

    // map_err: Errの中身を変換
    let result: Result<i32, &str> = Err("error");
    let mapped_err = result.map_err(|e| format!("変換されたエラー: {}", e));
    println!("map_err: {:?}", mapped_err);

    // and_then: 成功時に別のResultを返す操作をチェーン
    fn double(x: i32) -> Result<i32, &'static str> {
        Ok(x * 2)
    }

    let result = Ok(2).and_then(double).and_then(double);
    println!("and_then: {:?}", result);

    // or_else: エラー時に別の操作を試す
    let result: Result<i32, &str> = Err("エラー1");
    let recovered = result.or_else(|_| Ok::<i32, &str>(0));
    println!("or_else: {:?}", recovered);

    // unwrap_or: エラー時にデフォルト値
    let result: Result<i32, &str> = Err("error");
    let value = result.unwrap_or(42);
    println!("unwrap_or: {}", value);

    // unwrap_or_else: エラー時にクロージャでデフォルト値を計算
    let result: Result<i32, &str> = Err("error");
    let value = result.unwrap_or_else(|e| {
        println!("  (エラーから回復: {})", e);
        0
    });
    println!("unwrap_or_else: {}", value);
}

/// エラー処理のベストプラクティス
pub fn best_practices() {
    println!("\n=== ベストプラクティス ===");

    println!(
        r#"
panic!を使うべき場面:
- プログラムのバグを示す不整合な状態
- テストコード
- プロトタイピング
- 回復が不可能な致命的エラー

Result<T, E>を使うべき場面:
- ファイルI/O操作
- ネットワーク操作
- ユーザー入力の検証
- 失敗する可能性があるが回復可能な操作

実践的なガイドライン:
1. ライブラリはpanicを避け、Resultを返す
2. unwrap/expectはプロトタイプかテストで使用
3. ?演算子でエラー伝播を簡潔に
4. カスタムエラー型で詳細な情報を提供
5. anyhowやthiserrorクレートの活用を検討
"#
    );
}

/// Validationパターン
pub fn validation_pattern() {
    println!("\n=== 検証パターン ===");

    // 型システムを使った検証
    pub struct Guess {
        value: i32,
    }

    impl Guess {
        pub fn new(value: i32) -> Result<Guess, String> {
            if value < 1 || value > 100 {
                return Err(format!(
                    "予想は1から100の間でなければなりません。入力値: {}",
                    value
                ));
            }
            Ok(Guess { value })
        }

        pub fn value(&self) -> i32 {
            self.value
        }
    }

    // 使用例
    match Guess::new(50) {
        Ok(guess) => println!("有効な予想: {}", guess.value()),
        Err(e) => println!("無効: {}", e),
    }

    match Guess::new(200) {
        Ok(guess) => println!("有効な予想: {}", guess.value()),
        Err(e) => println!("無効: {}", e),
    }
}

/// すべてのデモを実行
pub fn run_all() {
    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║          Rustエラーハンドリングサンプル                         ║");
    println!("╚════════════════════════════════════════════════════════════════╝");

    panic_demo();
    result_basics();
    matching_on_different_errors();
    unwrap_and_expect();
    error_propagation();
    question_mark_with_option();
    custom_error_types();
    result_combinators();
    best_practices();
    validation_pattern();
}

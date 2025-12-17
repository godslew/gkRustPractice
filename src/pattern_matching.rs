// ============================================================================
// Rustパターンマッチングサンプル
// 公式ドキュメント: https://doc.rust-lang.org/book/ch06-02-match.html
//                 https://doc.rust-lang.org/book/ch18-00-patterns.html
// ============================================================================

/// match式の基本
pub fn basic_match() {
    println!("\n=== match式の基本 ===");

    #[derive(Debug)]
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    fn value_in_cents(coin: &Coin) -> u32 {
        match coin {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }

    let coins = [Coin::Penny, Coin::Nickel, Coin::Dime, Coin::Quarter];
    for coin in &coins {
        println!("{:?} = {} cents", coin, value_in_cents(coin));
    }
}

/// パターンと値の束縛
pub fn patterns_that_bind() {
    println!("\n=== パターンと値の束縛 ===");

    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
        California,
        // ... 他の州
    }

    #[derive(Debug)]
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState), // 州のデータを持つ
    }

    fn value_in_cents(coin: &Coin) -> u32 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                // stateに値が束縛される
                println!("State quarter from {:?}!", state);
                25
            }
        }
    }

    let coin = Coin::Quarter(UsState::California);
    println!("値: {} cents", value_in_cents(&coin));
}

/// Option<T>とのマッチング
pub fn matching_with_option() {
    println!("\n=== Option<T>とのマッチング ===");

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("Some(5) + 1 = {:?}", six);
    println!("None + 1 = {:?}", none);
}

/// 網羅性とキャッチオール
pub fn exhaustiveness_and_catchall() {
    println!("\n=== 網羅性とキャッチオール ===");

    // matchは全てのケースを網羅する必要がある
    let dice_roll = 9;

    // _はキャッチオールパターン
    match dice_roll {
        3 => println!("帽子をゲット!"),
        7 => println!("帽子を失う!"),
        _ => println!("移動する"), // それ以外すべて
    }

    // 値を使いたいがバインドしたくない場合
    match dice_roll {
        3 => println!("帽子をゲット!"),
        7 => println!("帽子を失う!"),
        other => println!("{}マス進む", other), // 値をバインド
    }

    // 何もしない場合はユニット型を返す
    match dice_roll {
        3 => println!("帽子をゲット!"),
        7 => println!("帽子を失う!"),
        _ => (), // 何もしない
    }
}

/// if let - 簡潔なパターンマッチ
pub fn if_let_demo() {
    println!("\n=== if let ===");

    let config_max: Option<u8> = Some(3);

    // matchで書くと冗長
    match config_max {
        Some(max) => println!("match: 最大値は {}", max),
        _ => (),
    }

    // if letで簡潔に
    if let Some(max) = config_max {
        println!("if let: 最大値は {}", max);
    }

    // elseブロックも使える
    let coin_state: Option<&str> = None;
    if let Some(state) = coin_state {
        println!("州: {}", state);
    } else {
        println!("州の情報なし");
    }
}

/// while let - ループでのパターンマッチ
pub fn while_let_demo() {
    println!("\n=== while let ===");

    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    // popがSomeを返す限りループ
    while let Some(top) = stack.pop() {
        println!("ポップした値: {}", top);
    }
}

/// let文でのパターン
pub fn let_patterns() {
    println!("\n=== let文でのパターン ===");

    // タプルの分解
    let (x, y, z) = (1, 2, 3);
    println!("x = {}, y = {}, z = {}", x, y, z);

    // 一部を無視
    let (a, _, c) = (1, 2, 3);
    println!("a = {}, c = {} (bは無視)", a, c);

    // ネストした構造の分解
    let ((feet, inches), point) = ((5, 10), (3, 4));
    println!("身長: {}フィート{}インチ", feet, inches);
    println!("座標: ({}, {})", point.0, point.1);
}

/// 関数パラメータでのパターン
pub fn function_parameter_patterns() {
    println!("\n=== 関数パラメータでのパターン ===");

    fn print_coordinates(&(x, y): &(i32, i32)) {
        println!("現在位置: ({}, {})", x, y);
    }

    let point = (3, 5);
    print_coordinates(&point);
}

/// 複雑なパターン
pub fn complex_patterns() {
    println!("\n=== 複雑なパターン ===");

    // リテラルのマッチ
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // 複数パターン（|）
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // 範囲パターン（..=）
    let x = 5;
    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    // 文字の範囲
    let c = 'c';
    match c {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}

/// 構造体のパターン分解
pub fn destructuring_structs() {
    println!("\n=== 構造体の分解 ===");

    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 7 };

    // フィールド名を使った分解
    let Point { x: a, y: b } = p;
    println!("a = {}, b = {}", a, b);

    // 省略形（変数名がフィールド名と同じ場合）
    let Point { x, y } = p;
    println!("x = {}, y = {}", x, y);

    // matchでの構造体分解
    match p {
        Point { x, y: 0 } => println!("x軸上の点: x = {}", x),
        Point { x: 0, y } => println!("y軸上の点: y = {}", y),
        Point { x, y } => println!("その他の点: ({}, {})", x, y),
    }
}

/// 列挙型のパターン分解
pub fn destructuring_enums() {
    println!("\n=== 列挙型の分解 ===");

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("Quit: 分解する値なし");
        }
        Message::Move { x, y } => {
            println!("Move: x = {}, y = {}", x, y);
        }
        Message::Write(text) => {
            println!("Write: {}", text);
        }
        Message::ChangeColor(r, g, b) => {
            println!("ChangeColor: RGB({}, {}, {})", r, g, b);
        }
    }
}

/// 参照の分解
pub fn destructuring_references() {
    println!("\n=== 参照の分解 ===");

    let points = vec![
        (1, 2),
        (3, 4),
        (5, 6),
    ];

    // &でパターンマッチすることで参照を外せる
    let sum: i32 = points
        .iter()
        .map(|&(x, y)| x + y)
        .sum();

    println!("各点の座標の合計: {}", sum);
}

/// パターンでの値の無視
pub fn ignoring_values() {
    println!("\n=== 値の無視 ===");

    // _で全体を無視
    fn foo(_: i32, y: i32) {
        println!("この関数は最初の引数を使わない: y = {}", y);
    }
    foo(3, 4);

    // ネストした_
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("既存の設定値を上書きできません");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }
    println!("設定: {:?}", setting_value);

    // _で始まる変数名は警告を抑制
    let _x = 5; // 未使用でも警告なし

    // ..で残りを無視
    struct Point3D {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point3D { x: 0, y: 0, z: 0 };
    match origin {
        Point3D { x, .. } => println!("x = {} (y, zは無視)", x),
    }

    // タプルで..を使う
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => {
            println!("最初: {}, 最後: {}", first, last);
        }
    }
}

/// マッチガード
pub fn match_guards() {
    println!("\n=== マッチガード ===");

    let num = Some(4);

    // ifでさらに条件を追加
    match num {
        Some(x) if x % 2 == 0 => println!("{} は偶数", x),
        Some(x) => println!("{} は奇数", x),
        None => (),
    }

    // 外部変数との比較
    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
}

/// @バインディング
pub fn at_bindings() {
    println!("\n=== @バインディング ===");

    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    // 範囲をテストしながら値を変数にバインド
    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("範囲内のid: {}", id_variable),
        Message::Hello { id: 10..=12 } => {
            println!("別の範囲内のid");
        }
        Message::Hello { id } => println!("その他のid: {}", id),
    }
}

/// すべてのデモを実行
pub fn run_all() {
    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║          Rustパターンマッチングサンプル                         ║");
    println!("╚════════════════════════════════════════════════════════════════╝");

    basic_match();
    patterns_that_bind();
    matching_with_option();
    exhaustiveness_and_catchall();
    if_let_demo();
    while_let_demo();
    let_patterns();
    function_parameter_patterns();
    complex_patterns();
    destructuring_structs();
    destructuring_enums();
    destructuring_references();
    ignoring_values();
    match_guards();
    at_bindings();
}

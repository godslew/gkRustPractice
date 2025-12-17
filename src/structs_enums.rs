// ============================================================================
// Rust構造体と列挙型サンプル
// 公式ドキュメント: https://doc.rust-lang.org/book/ch05-00-structs.html
//                 https://doc.rust-lang.org/book/ch06-00-enums.html
// ============================================================================

/// 基本的な構造体の定義と使用
pub fn basic_structs() {
    println!("\n=== 基本的な構造体 ===");

    // 構造体の定義（通常はモジュールレベルで行う）
    #[derive(Debug)] // デバッグ出力を可能にするderiveマクロ
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    // 構造体のインスタンス作成
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    println!("User: {:?}", user1);
    println!("Username: {}", user1.username);

    // 可変インスタンス（構造体全体が可変になる）
    let mut user2 = User {
        active: true,
        username: String::from("anotheruser"),
        email: String::from("another@example.com"),
        sign_in_count: 1,
    };
    user2.email = String::from("newemail@example.com");
    println!("更新後のemail: {}", user2.email);

    // フィールド初期化省略記法
    fn build_user(email: String, username: String) -> User {
        User {
            active: true,
            username, // 変数名とフィールド名が同じなら省略可能
            email,    // 変数名とフィールド名が同じなら省略可能
            sign_in_count: 1,
        }
    }

    let user3 = build_user(
        String::from("test@example.com"),
        String::from("testuser"),
    );
    println!("build_userで作成: {:?}", user3);

    // 構造体更新記法
    let user4 = User {
        email: String::from("different@example.com"),
        ..user3 // 残りのフィールドをuser3からコピー
    };
    println!("更新記法で作成: {:?}", user4);
    // 注意: user3のStringフィールドはムーブされたので、user3は部分的に無効
}

/// タプル構造体
pub fn tuple_structs() {
    println!("\n=== タプル構造体 ===");

    // タプル構造体 - 名前付きタプル
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // フィールドへのアクセスはインデックス
    println!("Color: ({}, {}, {})", black.0, black.1, black.2);
    println!("Point: ({}, {}, {})", origin.0, origin.1, origin.2);

    // 分解も可能
    let Color(r, g, b) = black;
    println!("RGB: r={}, g={}, b={}", r, g, b);

    // 注意: ColorとPointは同じ構造でも異なる型
    // let c: Color = origin; // エラー！
}

/// ユニット様構造体
pub fn unit_like_structs() {
    println!("\n=== ユニット様構造体 ===");

    // フィールドを持たない構造体
    // トレイトの実装時に便利
    struct AlwaysEqual;

    let _subject = AlwaysEqual;
    println!("AlwaysEqualは中身がないので何も表示しません");
}

/// メソッドの定義
pub fn methods() {
    println!("\n=== メソッド ===");

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    // implブロックでメソッドを定義
    impl Rectangle {
        // メソッド（&selfを取る）
        fn area(&self) -> u32 {
            self.width * self.height
        }

        // 他のRectangleを引数に取るメソッド
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }

        // 可変参照を取るメソッド
        fn double_size(&mut self) {
            self.width *= 2;
            self.height *= 2;
        }
    }

    // 複数のimplブロックを持つことも可能
    impl Rectangle {
        // 関連関数（selfを取らない）- コンストラクタによく使う
        fn square(size: u32) -> Self {
            Self {
                width: size,
                height: size,
            }
        }

        fn new(width: u32, height: u32) -> Self {
            Self { width, height }
        }
    }

    let rect1 = Rectangle::new(30, 50);
    println!("rect1: {:?}", rect1);
    println!("面積: {} 平方ピクセル", rect1.area());

    let rect2 = Rectangle::new(10, 40);
    let rect3 = Rectangle::new(60, 45);

    println!("rect1はrect2を含められる? {}", rect1.can_hold(&rect2));
    println!("rect1はrect3を含められる? {}", rect1.can_hold(&rect3));

    // 関連関数の呼び出し（::を使う）
    let square = Rectangle::square(25);
    println!("正方形: {:?}, 面積: {}", square, square.area());

    // 可変メソッドの呼び出し
    let mut rect = Rectangle::new(10, 20);
    println!("元のサイズ: {:?}", rect);
    rect.double_size();
    println!("2倍後: {:?}", rect);
}

/// 列挙型の基本
pub fn basic_enums() {
    println!("\n=== 列挙型の基本 ===");

    // シンプルな列挙型
    #[derive(Debug)]
    enum IpAddrKind {
        V4,
        V6,
    }

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    println!("IPv4: {:?}", four);
    println!("IPv6: {:?}", six);

    // 列挙型をパラメータとして使う
    fn route(ip_kind: IpAddrKind) {
        println!("Routing: {:?}", ip_kind);
    }

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
}

/// データを持つ列挙型
pub fn enums_with_data() {
    println!("\n=== データを持つ列挙型 ===");

    // 各バリアントが異なるデータを持てる
    #[derive(Debug)]
    enum IpAddr {
        V4(u8, u8, u8, u8),  // タプル形式
        V6(String),          // 単一の値
    }

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    println!("home: {:?}", home);
    println!("loopback: {:?}", loopback);

    // より複雑な例
    #[derive(Debug)]
    enum Message {
        Quit,                       // データなし
        Move { x: i32, y: i32 },    // 名前付きフィールド（構造体様）
        Write(String),              // 単一のString
        ChangeColor(i32, i32, i32), // 3つのi32
    }

    let messages = [
        Message::Quit,
        Message::Move { x: 10, y: 20 },
        Message::Write(String::from("hello")),
        Message::ChangeColor(255, 128, 0),
    ];

    for msg in &messages {
        println!("Message: {:?}", msg);
    }

    // 列挙型にもメソッドを定義できる
    impl Message {
        fn call(&self) {
            match self {
                Message::Quit => println!("  -> Quit!"),
                Message::Move { x, y } => println!("  -> Move to ({}, {})", x, y),
                Message::Write(text) => println!("  -> Write: {}", text),
                Message::ChangeColor(r, g, b) => {
                    println!("  -> Change color to RGB({}, {}, {})", r, g, b)
                }
            }
        }
    }

    println!("\nメソッド呼び出し:");
    for msg in &messages {
        msg.call();
    }
}

/// Option列挙型 - nullの代わり
pub fn option_enum() {
    println!("\n=== Option列挙型 ===");

    // Option<T>は標準ライブラリで定義されている
    // enum Option<T> {
    //     None,
    //     Some(T),
    // }

    let some_number: Option<i32> = Some(5);
    let some_string: Option<&str> = Some("a string");
    let absent_number: Option<i32> = None;

    println!("some_number: {:?}", some_number);
    println!("some_string: {:?}", some_string);
    println!("absent_number: {:?}", absent_number);

    // Option<T>とTは異なる型なので、直接演算できない
    let x: i32 = 5;
    let y: Option<i32> = Some(5);

    // let sum = x + y; // エラー！

    // 値を取り出すには明示的な処理が必要
    let sum = x + y.unwrap_or(0); // Noneなら0を使う
    println!("x + y.unwrap_or(0) = {}", sum);

    // match や if let を使うのが一般的（後述）
}

/// Result列挙型 - エラーハンドリング
pub fn result_enum() {
    println!("\n=== Result列挙型 ===");

    // Result<T, E>も標準ライブラリで定義されている
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }

    fn divide(a: f64, b: f64) -> Result<f64, String> {
        if b == 0.0 {
            Err(String::from("0で割ることはできません"))
        } else {
            Ok(a / b)
        }
    }

    let result1 = divide(10.0, 2.0);
    let result2 = divide(10.0, 0.0);

    println!("10 / 2 = {:?}", result1);
    println!("10 / 0 = {:?}", result2);

    // matchで処理
    match result1 {
        Ok(value) => println!("成功: {}", value),
        Err(e) => println!("エラー: {}", e),
    }

    match result2 {
        Ok(value) => println!("成功: {}", value),
        Err(e) => println!("エラー: {}", e),
    }
}

/// Deriveマクロ
pub fn derive_macros() {
    println!("\n=== Deriveマクロ ===");

    // よく使うderiveマクロ
    #[derive(Debug, Clone, PartialEq, Eq, Default)]
    struct Point {
        x: i32,
        y: i32,
    }

    let p1 = Point { x: 1, y: 2 };
    let p2 = p1.clone(); // Clone
    let p3 = Point::default(); // Default

    println!("Debug表示: {:?}", p1);
    println!("p1 == p2: {}", p1 == p2); // PartialEq
    println!("デフォルト値: {:?}", p3);

    // 列挙型にもderiveできる
    #[derive(Debug, Clone, Copy, PartialEq)]
    enum Direction {
        North,
        South,
        East,
        West,
    }

    let d1 = Direction::North;
    let d2 = d1; // Copy
    println!("d1 = {:?}, d2 = {:?}", d1, d2);
}

/// すべてのデモを実行
pub fn run_all() {
    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║          Rust構造体と列挙型サンプル                             ║");
    println!("╚════════════════════════════════════════════════════════════════╝");

    basic_structs();
    tuple_structs();
    unit_like_structs();
    methods();
    basic_enums();
    enums_with_data();
    option_enum();
    result_enum();
    derive_macros();
}

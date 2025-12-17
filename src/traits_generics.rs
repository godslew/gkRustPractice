// ============================================================================
// Rustトレイトとジェネリクスサンプル
// 公式ドキュメント: https://doc.rust-lang.org/book/ch10-00-generics.html
// ============================================================================

use std::fmt::{Debug, Display};

/// ジェネリクスの基本
pub fn generics_basics() {
    println!("\n=== ジェネリクスの基本 ===");

    // ジェネリクスなしの場合 - 型ごとに関数が必要
    fn largest_i32(list: &[i32]) -> &i32 {
        let mut largest = &list[0];
        for item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    fn largest_char(list: &[char]) -> &char {
        let mut largest = &list[0];
        for item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    let number_list = vec![34, 50, 25, 100, 65];
    println!("最大の数: {}", largest_i32(&number_list));

    let char_list = vec!['y', 'm', 'a', 'q'];
    println!("最大の文字: {}", largest_char(&char_list));

    // ジェネリクスを使った関数（後述のトレイト境界が必要）
    fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
        let mut largest = &list[0];
        for item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    println!("ジェネリック版 - 最大の数: {}", largest(&number_list));
    println!("ジェネリック版 - 最大の文字: {}", largest(&char_list));
}

/// ジェネリック構造体
pub fn generic_structs() {
    println!("\n=== ジェネリック構造体 ===");

    // 1つの型パラメータ
    #[derive(Debug)]
    struct Point<T> {
        x: T,
        y: T,
    }

    let integer_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.0, y: 4.0 };

    println!("整数Point: {:?}", integer_point);
    println!("浮動小数点Point: {:?}", float_point);

    // 複数の型パラメータ
    #[derive(Debug)]
    struct Point2<T, U> {
        x: T,
        y: U,
    }

    let mixed_point = Point2 { x: 5, y: 4.0 };
    println!("混合Point: {:?}", mixed_point);

    // メソッドの定義
    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }

    // 特定の型に対するメソッド
    impl Point<f32> {
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    let p = Point { x: 3.0_f32, y: 4.0_f32 };
    println!("原点からの距離: {}", p.distance_from_origin());

    // 異なる型パラメータを持つメソッド
    impl<T, U> Point2<T, U> {
        fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
            Point2 {
                x: self.x,
                y: other.y,
            }
        }
    }

    let p1 = Point2 { x: 5, y: 10.4 };
    let p2 = Point2 { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("mixup結果: x = {}, y = {}", p3.x, p3.y);
}

/// ジェネリック列挙型
pub fn generic_enums() {
    println!("\n=== ジェネリック列挙型 ===");

    // 標準ライブラリのOption<T>とResult<T, E>
    // enum Option<T> {
    //     Some(T),
    //     None,
    // }
    //
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }

    let some_number: Option<i32> = Some(5);
    let some_string: Option<String> = Some(String::from("Hello"));
    let absent_number: Option<i32> = None;

    println!("Option<i32>: {:?}", some_number);
    println!("Option<String>: {:?}", some_string);
    println!("None: {:?}", absent_number);
}

/// トレイトの定義と実装
pub fn traits_basics() {
    println!("\n=== トレイトの基本 ===");

    // トレイト定義
    trait Summary {
        fn summarize(&self) -> String;

        // デフォルト実装
        fn summarize_author(&self) -> String {
            String::from("(著者不明)")
        }
    }

    // 構造体定義
    struct NewsArticle {
        headline: String,
        location: String,
        author: String,
        content: String,
    }

    struct Tweet {
        username: String,
        content: String,
        reply: bool,
        retweet: bool,
    }

    // トレイト実装
    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, {} ({})", self.headline, self.author, self.location)
        }

        fn summarize_author(&self) -> String {
            format!("@{}", self.author)
        }
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }

    let article = NewsArticle {
        headline: String::from("Rustが最も愛されている言語に選ばれる"),
        location: String::from("東京"),
        author: String::from("技術太郎"),
        content: String::from("Rustは..."),
    };

    let tweet = Tweet {
        username: String::from("rust_lover"),
        content: String::from("Rustを学び始めました!"),
        reply: false,
        retweet: false,
    };

    println!("記事の要約: {}", article.summarize());
    println!("記事の著者: {}", article.summarize_author());
    println!("ツイートの要約: {}", tweet.summarize());
    println!("ツイートの著者: {}", tweet.summarize_author()); // デフォルト実装
}

/// トレイト境界
pub fn trait_bounds() {
    println!("\n=== トレイト境界 ===");

    trait Summary {
        fn summarize(&self) -> String;
    }

    #[derive(Debug)]
    struct Article {
        title: String,
    }

    impl Summary for Article {
        fn summarize(&self) -> String {
            self.title.clone()
        }
    }

    // impl Trait 構文（引数として）
    fn notify(item: &impl Summary) {
        println!("速報! {}", item.summarize());
    }

    // トレイト境界構文（より明示的）
    fn notify_verbose<T: Summary>(item: &T) {
        println!("速報（verbose）! {}", item.summarize());
    }

    // 複数のトレイト境界
    fn notify_with_display<T: Summary + Display>(item: &T) {
        println!("表示: {}, 要約: {}", item, item.summarize());
    }

    // where句を使った読みやすい構文
    fn some_function<T, U>(_t: &T, _u: &U) -> i32
    where
        T: Display + Clone,
        U: Clone + Debug,
    {
        42
    }

    let article = Article {
        title: String::from("Rustの新機能"),
    };

    notify(&article);
    notify_verbose(&article);

    // impl Displayを実装した例
    impl Display for Article {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Article: {}", self.title)
        }
    }

    notify_with_display(&article);
}

/// トレイトを戻り値として
pub fn returning_traits() {
    println!("\n=== 戻り値としてのトレイト ===");

    trait Summary {
        fn summarize(&self) -> String;
    }

    struct Tweet {
        username: String,
        content: String,
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }

    // impl Traitを戻り値として使用
    // 注意: 1つの具体型のみ返せる
    fn returns_summarizable() -> impl Summary {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("もちろん、ご存知かもしれませんが"),
        }
    }

    let item = returns_summarizable();
    println!("戻り値: {}", item.summarize());
}

/// 条件付きメソッド実装
pub fn conditional_implementations() {
    println!("\n=== 条件付きメソッド実装 ===");

    struct Pair<T> {
        x: T,
        y: T,
    }

    // すべてのPair<T>に実装
    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }

    // PartialOrdとDisplayを実装している型のPairにのみ実装
    impl<T: Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("最大値は x = {}", self.x);
            } else {
                println!("最大値は y = {}", self.y);
            }
        }
    }

    let pair = Pair::new(10, 5);
    pair.cmp_display();

    // ブランケット実装の例（標準ライブラリ）
    // impl<T: Display> ToString for T {
    //     fn to_string(&self) -> String { ... }
    // }
    // Displayを実装していれば自動的にto_string()が使える

    let s = 3.to_string();
    println!("to_string(): {}", s);
}

/// 関連型を持つトレイト
pub fn associated_types() {
    println!("\n=== 関連型 ===");

    // 関連型を持つトレイト
    trait Iterator {
        type Item; // 関連型

        fn next(&mut self) -> Option<Self::Item>;
    }

    struct Counter {
        count: u32,
        max: u32,
    }

    impl Counter {
        fn new(max: u32) -> Counter {
            Counter { count: 0, max }
        }
    }

    impl Iterator for Counter {
        type Item = u32; // 具体的な型を指定

        fn next(&mut self) -> Option<Self::Item> {
            if self.count < self.max {
                self.count += 1;
                Some(self.count)
            } else {
                None
            }
        }
    }

    let mut counter = Counter::new(3);
    println!("カウンター:");
    while let Some(n) = counter.next() {
        println!("  {}", n);
    }
}

/// デフォルト型パラメータ
pub fn default_generic_type_parameters() {
    println!("\n=== デフォルト型パラメータ ===");

    use std::ops::Add;

    #[derive(Debug, Clone, Copy, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    // Addトレイトの定義（参考）:
    // trait Add<Rhs = Self> {
    //     type Output;
    //     fn add(self, rhs: Rhs) -> Self::Output;
    // }

    impl Add for Point {
        type Output = Point;

        fn add(self, other: Point) -> Point {
            Point {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    let p1 = Point { x: 1, y: 0 };
    let p2 = Point { x: 2, y: 3 };
    let p3 = p1 + p2;

    println!("{:?} + {:?} = {:?}", p1, p2, p3);
}

/// スーパートレイト
pub fn supertraits() {
    println!("\n=== スーパートレイト ===");

    // OutlinePrintはDisplayを要求する（スーパートレイト）
    trait OutlinePrint: Display {
        fn outline_print(&self) {
            let output = self.to_string();
            let len = output.len();
            println!("{}", "*".repeat(len + 4));
            println!("*{}*", " ".repeat(len + 2));
            println!("* {} *", output);
            println!("*{}*", " ".repeat(len + 2));
            println!("{}", "*".repeat(len + 4));
        }
    }

    struct Point {
        x: i32,
        y: i32,
    }

    impl Display for Point {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }

    impl OutlinePrint for Point {}

    let p = Point { x: 1, y: 3 };
    p.outline_print();
}

/// すべてのデモを実行
pub fn run_all() {
    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║          Rustトレイトとジェネリクスサンプル                      ║");
    println!("╚════════════════════════════════════════════════════════════════╝");

    generics_basics();
    generic_structs();
    generic_enums();
    traits_basics();
    trait_bounds();
    returning_traits();
    conditional_implementations();
    associated_types();
    default_generic_type_parameters();
    supertraits();
}

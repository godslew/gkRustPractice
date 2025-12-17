// ============================================================================
// Rust所有権システムサンプル
// 公式ドキュメント: https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html
// ============================================================================
//
// 所有権のルール:
// 1. Rustの各値には「所有者」と呼ばれる変数がある
// 2. 値の所有者は同時に1つだけ
// 3. 所有者がスコープを抜けると、値は破棄される（drop）

/// 所有権の基本デモ
pub fn ownership_basics() {
    println!("\n=== 所有権の基本 ===");

    // スコープと所有権
    {
        let s = String::from("hello"); // sがスコープに入り、有効になる
        println!("スコープ内: s = {}", s);
        // sを使って何かする
    } // スコープを抜けると、sはdropされてメモリが解放される
      // ここではsは無効

    // ムーブ（Move）
    println!("\n-- ムーブ --");
    let s1 = String::from("hello");
    let s2 = s1; // s1の値はs2にムーブされる
                 // println!("{}", s1); // エラー！s1はもう有効ではない
    println!("s2 = {} (s1からムーブされた)", s2);

    // 整数などのスカラー型はCopyトレイトを持つのでムーブされない
    let x = 5;
    let y = x; // xはコピーされる（ムーブではない）
    println!("x = {}, y = {} (整数はコピーされる)", x, y);

    // クローン（Clone）
    println!("\n-- クローン --");
    let s1 = String::from("hello");
    let s2 = s1.clone(); // ヒープデータを含めて深いコピー
    println!("s1 = {}, s2 = {} (クローンされた)", s1, s2);
}

/// 関数と所有権
pub fn ownership_and_functions() {
    println!("\n=== 関数と所有権 ===");

    let s = String::from("hello"); // sがスコープに入る
    takes_ownership(s); // sの値が関数にムーブする
                        // println!("{}", s); // エラー！sはもう有効ではない

    let x = 5; // xがスコープに入る
    makes_copy(x); // xは関数にコピーされる
    println!("makes_copy後も x = {} (コピーされたので使える)", x);

    // 所有権を返す
    println!("\n-- 所有権を返す --");
    let s1 = gives_ownership(); // 関数が所有権を返す
    println!("gives_ownership から: {}", s1);

    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2); // s2はムーブし、戻り値はs3にムーブ
                                       // println!("{}", s2); // エラー！s2は無効
    println!("takes_and_gives_back から: {}", s3);
}

fn takes_ownership(some_string: String) {
    println!("takes_ownership: {}", some_string);
} // some_stringがスコープを抜けdropされる

fn makes_copy(some_integer: i32) {
    println!("makes_copy: {}", some_integer);
} // some_integerがスコープを抜けるが、特に何も起きない

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string // 戻り値として所有権を移動
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string // そのまま返す
}

/// 参照と借用
pub fn references_and_borrowing() {
    println!("\n=== 参照と借用 ===");

    // 不変参照
    println!("\n-- 不変参照 --");
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // &s1でs1への参照を作成
    println!("'{}' の長さは {} です", s1, len); // s1はまだ使える！

    // 複数の不変参照は許可される
    let r1 = &s1;
    let r2 = &s1;
    println!("r1 = {}, r2 = {}", r1, r2);

    // 可変参照
    println!("\n-- 可変参照 --");
    let mut s = String::from("hello");
    change(&mut s);
    println!("変更後: {}", s);

    // 可変参照の制限: 同時に1つの可変参照しか持てない
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
        println!("スコープ内の可変参照: {}", r1);
    } // r1はここでスコープを抜ける
    let r2 = &mut s; // 新しい可変参照を作れる
    println!("新しい可変参照: {}", r2);

    // 不変参照と可変参照は同時に存在できない（データ競合を防ぐ）
    let mut s = String::from("hello");
    let r1 = &s; // OK
    let r2 = &s; // OK
    println!("r1 = {}, r2 = {}", r1, r2);
    // r1とr2はここ以降使われないので、このスコープは終了
    let r3 = &mut s; // OK - r1とr2はもう使われない
    println!("r3 = {}", r3);
}

fn calculate_length(s: &String) -> usize {
    s.len()
} // sはスコープを抜けるが、参照なので何もdropされない

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

/// スライス型
pub fn slices() {
    println!("\n=== スライス ===");

    // 文字列スライス
    println!("\n-- 文字列スライス --");
    let s = String::from("hello world");

    let hello = &s[0..5]; // または &s[..5]
    let world = &s[6..11]; // または &s[6..]
    let whole = &s[..]; // 全体

    println!("s = '{}'", s);
    println!("hello = '{}', world = '{}'", hello, world);
    println!("whole = '{}'", whole);

    // 文字列リテラルはスライス
    let s: &str = "Hello, world!"; // &str型
    println!("文字列リテラル: {}", s);

    // first_wordの例
    let my_string = String::from("hello world");
    let word = first_word(&my_string[..]); // Stringのスライスを渡す
    println!("最初の単語: {}", word);

    let my_string_literal = "hello world";
    let word = first_word(my_string_literal); // 文字列リテラルはそのまま渡せる
    println!("最初の単語: {}", word);

    // 配列スライス
    println!("\n-- 配列スライス --");
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3]; // [2, 3]
    println!("配列: {:?}", a);
    println!("スライス [1..3]: {:?}", slice);
    assert_eq!(slice, &[2, 3]);
}

// &str を受け取ることで String と &str の両方を受け入れられる
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

/// ダングリング参照の防止
pub fn no_dangling() {
    println!("\n=== ダングリング参照の防止 ===");

    // Rustコンパイラはダングリング参照を防ぐ
    // 以下のコードはコンパイルエラーになる:
    //
    // fn dangle() -> &String {
    //     let s = String::from("hello");
    //     &s // sへの参照を返そうとする
    // } // sはここでdropされるので、参照は無効になる
    //
    // 解決策: 所有権を返す
    fn no_dangle() -> String {
        let s = String::from("hello");
        s // 所有権をムーブする
    }

    let result = no_dangle();
    println!("ダングリングしない: {}", result);
}

/// 所有権のまとめ
pub fn ownership_summary() {
    println!("\n=== 所有権のまとめ ===");
    println!(
        r#"
所有権のルール:
1. 各値には1つの所有者がある
2. 所有者は同時に1つだけ
3. 所有者がスコープを抜けると値はdropされる

参照のルール:
1. 任意の時点で、1つの可変参照 OR 任意の数の不変参照を持てる
2. 参照は常に有効でなければならない

これらのルールにより:
- メモリ安全性がコンパイル時に保証される
- データ競合がコンパイル時に防がれる
- ガベージコレクタが不要になる
"#
    );
}

/// すべてのデモを実行
pub fn run_all() {
    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║          Rust所有権システムサンプル                             ║");
    println!("╚════════════════════════════════════════════════════════════════╝");

    ownership_basics();
    ownership_and_functions();
    references_and_borrowing();
    slices();
    no_dangling();
    ownership_summary();
}

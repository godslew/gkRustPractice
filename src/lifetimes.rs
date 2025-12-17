// ============================================================================
// Rustライフタイムサンプル
// 公式ドキュメント: https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html
// ============================================================================
//
// ライフタイムはRustの借用チェッカーが参照の有効期間を追跡するための仕組み
// 主な目的: ダングリング参照（無効なメモリを指す参照）を防ぐ

/// ライフタイムが必要な理由
pub fn why_lifetimes() {
    println!("\n=== ライフタイムが必要な理由 ===");

    // 以下のコードはコンパイルエラーになる:
    // let r;
    // {
    //     let x = 5;
    //     r = &x; // xへの参照
    // } // xがスコープを抜ける
    // println!("{}", r); // ダングリング参照！

    // 正しい例:
    let x = 5;
    let r = &x;
    println!("r = {} (xはまだ有効)", r);
}

/// 関数シグネチャのライフタイム
pub fn function_lifetimes() {
    println!("\n=== 関数シグネチャのライフタイム ===");

    // 2つの文字列スライスを受け取り、長い方を返す
    // 戻り値の参照がどちらの引数と同じライフタイムを持つか不明なため、
    // 明示的なライフタイム注釈が必要

    // ライフタイム注釈: 'a（アポストロフィ + 名前）
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let string1 = String::from("long string is long");
    let string2 = String::from("xyz");

    let result = longest(&string1, &string2);
    println!("最も長い文字列: {}", result);

    // 異なるスコープの例
    let string1 = String::from("abcd");
    {
        let string2 = String::from("xyz");
        let result = longest(&string1, &string2);
        // resultは内側のスコープ内でのみ有効
        println!("内側スコープでの最長: {}", result);
    }
    // ここではresultは使えない（string2のライフタイムが終了）
}

/// ライフタイム注釈の構文
pub fn lifetime_syntax() {
    println!("\n=== ライフタイム注釈の構文 ===");

    // 参照のライフタイム注釈
    // &i32        - 参照
    // &'a i32     - 明示的なライフタイムを持つ参照
    // &'a mut i32 - 明示的なライフタイムを持つ可変参照

    // 複数のライフタイムパラメータ
    fn first_word<'a>(s: &'a str) -> &'a str {
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
        &s[..]
    }

    let sentence = String::from("hello world");
    let word = first_word(&sentence);
    println!("最初の単語: {}", word);

    // 異なるライフタイムを持つ複数の参照
    fn compare<'a, 'b>(x: &'a str, y: &'b str) -> bool {
        x.len() > y.len()
    }

    let a = "hello";
    let b = "world!";
    println!("'{}' > '{}' (長さ): {}", a, b, compare(a, b));
}

/// 構造体のライフタイム
pub fn struct_lifetimes() {
    println!("\n=== 構造体のライフタイム ===");

    // 参照を持つ構造体にはライフタイム注釈が必要
    #[derive(Debug)]
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    impl<'a> ImportantExcerpt<'a> {
        fn level(&self) -> i32 {
            3
        }

        fn announce_and_return_part(&self, announcement: &str) -> &str {
            println!("お知らせ: {}", announcement);
            self.part
        }
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("文が見つかりません");

    let excerpt = ImportantExcerpt {
        part: first_sentence,
    };

    println!("抜粋: {:?}", excerpt);
    println!("レベル: {}", excerpt.level());
    println!(
        "発表: {}",
        excerpt.announce_and_return_part("重要なお知らせです")
    );
}

/// ライフタイムの省略規則
pub fn lifetime_elision() {
    println!("\n=== ライフタイムの省略規則 ===");

    // コンパイラは以下の規則でライフタイムを推論:
    // 1. 各参照パラメータに個別のライフタイムが割り当てられる
    // 2. 入力ライフタイムが1つだけなら、出力ライフタイムも同じ
    // 3. &selfか&mut selfがあれば、selfのライフタイムが出力に適用される

    // 省略規則が適用される例（明示的な注釈不要）
    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
        &s[..]
    }

    // 上記は以下と同等:
    // fn first_word<'a>(s: &'a str) -> &'a str { ... }

    let s = "hello world";
    println!("最初の単語: {}", first_word(s));

    // メソッドでの省略（規則3）
    struct StringHolder {
        content: String,
    }

    impl StringHolder {
        // &selfがあるので、戻り値のライフタイムはselfと同じと推論される
        fn get_content(&self) -> &str {
            &self.content
        }
    }

    let holder = StringHolder {
        content: String::from("Hello"),
    };
    println!("内容: {}", holder.get_content());
}

/// 'static ライフタイム
pub fn static_lifetime() {
    println!("\n=== 'static ライフタイム ===");

    // 'static はプログラム全体の期間有効な参照
    let s: &'static str = "I have a static lifetime.";
    println!("静的ライフタイム: {}", s);

    // 文字列リテラルは全て 'static
    // バイナリに直接埋め込まれるため

    // 'staticを乱用しないこと！
    // 本当に必要な場合:
    // - 文字列リテラル
    // - グローバル定数
    // - static変数
    // - Box::leak()で意図的にリークさせた値

    // エラーメッセージで'staticを提案されても、
    // 実際には別のライフタイム問題がある場合が多い
}

/// ジェネリクス、トレイト境界、ライフタイムの組み合わせ
pub fn complex_lifetimes() {
    println!("\n=== 複合的なライフタイム ===");

    use std::fmt::Display;

    // 全ての要素を組み合わせた関数
    fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where
        T: Display,
    {
        println!("お知らせ: {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest_with_an_announcement(string1.as_str(), string2, "比較を開始します");
    println!("最長の文字列: {}", result);
}

/// ライフタイムの制約
pub fn lifetime_bounds() {
    println!("\n=== ライフタイムの制約 ===");

    // 'b: 'a は「'bは少なくとも'aと同じ長さ」を意味
    struct Context<'s>(&'s str);

    struct Parser<'c, 's: 'c> {
        context: &'c Context<'s>,
    }

    impl<'c, 's: 'c> Parser<'c, 's> {
        fn parse(&self) -> Result<(), &'s str> {
            let input = self.context.0;
            if input.is_empty() {
                Err("入力が空です")
            } else {
                println!("パース中: {}", input);
                Ok(())
            }
        }
    }

    let text = String::from("hello");
    let context = Context(&text);
    let parser = Parser { context: &context };
    match parser.parse() {
        Ok(()) => println!("パース成功"),
        Err(e) => println!("パースエラー: {}", e),
    }
}

/// 実践的な例
pub fn practical_examples() {
    println!("\n=== 実践的な例 ===");

    // キャッシュ構造体
    struct Cache<'a> {
        data: &'a str,
        processed: Option<String>,
    }

    impl<'a> Cache<'a> {
        fn new(data: &'a str) -> Self {
            Cache {
                data,
                processed: None,
            }
        }

        fn get_processed(&mut self) -> &str {
            if self.processed.is_none() {
                // 初回のみ処理
                self.processed = Some(self.data.to_uppercase());
            }
            self.processed.as_ref().unwrap()
        }

        fn get_original(&self) -> &str {
            self.data
        }
    }

    let data = String::from("hello world");
    let mut cache = Cache::new(&data);

    println!("オリジナル: {}", cache.get_original());
    println!("処理済み: {}", cache.get_processed());
    println!("再度（キャッシュから）: {}", cache.get_processed());

    // イテレータを返す例
    struct Words<'a> {
        text: &'a str,
    }

    impl<'a> Words<'a> {
        fn new(text: &'a str) -> Self {
            Words { text }
        }

        fn iter(&self) -> impl Iterator<Item = &str> {
            self.text.split_whitespace()
        }
    }

    let text = "Rust is a systems programming language";
    let words = Words::new(text);
    println!("単語:");
    for word in words.iter() {
        println!("  - {}", word);
    }
}

/// ライフタイムのベストプラクティス
pub fn best_practices() {
    println!("\n=== ライフタイムのベストプラクティス ===");

    println!(
        r#"
1. 可能な限り省略規則に任せる
   - コンパイラが推論できる場合は注釈不要

2. 'staticの乱用を避ける
   - 本当に必要な場合のみ使用
   - エラーで'staticを提案されても、別の解決策を探る

3. 最小限の制約を付ける
   - 必要以上に制約を厳しくしない
   - ライフタイムを複雑にしすぎない

4. 所有権を移す選択肢も考える
   - 参照の代わりに所有権を渡す方が簡単な場合も
   - Cloneのコストが許容できるなら

5. 構造体での参照を避ける選択
   - String vs &str
   - Vec<T> vs &[T]
   - 所有する方が扱いやすい場合が多い

6. エラーメッセージをよく読む
   - Rustのエラーメッセージはヒントが豊富
   - 提案される修正を理解してから適用する
"#
    );
}

/// すべてのデモを実行
pub fn run_all() {
    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║          Rustライフタイムサンプル                               ║");
    println!("╚════════════════════════════════════════════════════════════════╝");

    why_lifetimes();
    function_lifetimes();
    lifetime_syntax();
    struct_lifetimes();
    lifetime_elision();
    static_lifetime();
    complex_lifetimes();
    lifetime_bounds();
    practical_examples();
    best_practices();
}

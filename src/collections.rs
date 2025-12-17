// ============================================================================
// Rustコレクションサンプル
// 公式ドキュメント: https://doc.rust-lang.org/book/ch08-00-common-collections.html
// ============================================================================
//
// 主なコレクション:
// - Vec<T>: 可変長の配列
// - String: UTF-8エンコードされた可変長文字列
// - HashMap<K, V>: キーと値のマッピング

use std::collections::HashMap;

/// ベクター（Vec<T>）の基本
pub fn vector_basics() {
    println!("\n=== ベクターの基本 ===");

    // ベクターの作成
    let v1: Vec<i32> = Vec::new(); // 空のベクター（型注釈が必要）
    let v2 = vec![1, 2, 3]; // vec!マクロで初期化

    println!("空のベクター: {:?}", v1);
    println!("vec!マクロ: {:?}", v2);

    // 要素の追加（mutが必要）
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("pushで追加: {:?}", v);

    // 要素へのアクセス
    let third: &i32 = &v[2]; // インデックスアクセス（パニックの可能性）
    println!("3番目の要素（インデックス）: {}", third);

    let third: Option<&i32> = v.get(2); // getメソッド（安全）
    match third {
        Some(value) => println!("3番目の要素（get）: {}", value),
        None => println!("3番目の要素はありません"),
    }

    // 範囲外アクセス
    // let does_not_exist = &v[100]; // これはパニック!
    let does_not_exist = v.get(100); // これはNoneを返す
    println!("範囲外アクセス（get）: {:?}", does_not_exist);

    // 要素の変更
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50; // デリファレンスして値を変更
    }
    println!("各要素に50を加算: {:?}", v);
}

/// ベクターの操作
pub fn vector_operations() {
    println!("\n=== ベクターの操作 ===");

    let mut v = vec![1, 2, 3, 4, 5];

    // 最後の要素を取り出す
    let last = v.pop();
    println!("pop: {:?}, ベクター: {:?}", last, v);

    // 特定位置に挿入
    v.insert(0, 100);
    println!("insert(0, 100): {:?}", v);

    // 特定位置から削除
    let removed = v.remove(0);
    println!("remove(0): {}, ベクター: {:?}", removed, v);

    // 長さと容量
    println!("長さ: {}, 容量: {}", v.len(), v.capacity());

    // ベクターのクリア
    v.clear();
    println!("clear後: {:?}, 空?: {}", v, v.is_empty());

    // スライスとして使用
    let v = vec![1, 2, 3, 4, 5];
    let slice = &v[1..4];
    println!("スライス [1..4]: {:?}", slice);

    // ソート
    let mut v = vec![5, 3, 1, 4, 2];
    v.sort();
    println!("ソート後: {:?}", v);

    // 逆順
    v.reverse();
    println!("逆順: {:?}", v);

    // 重複除去（ソート済みの場合）
    let mut v = vec![1, 1, 2, 2, 3, 3];
    v.dedup();
    println!("重複除去後: {:?}", v);
}

/// ベクターでの反復処理
pub fn vector_iteration() {
    println!("\n=== ベクターでの反復処理 ===");

    let v = vec![100, 32, 57];

    // 不変参照での反復
    println!("不変参照での反復:");
    for i in &v {
        println!("  {}", i);
    }
    println!("反復後もvは使用可能: {:?}", v);

    // 可変参照での反復
    let mut v = vec![100, 32, 57];
    println!("可変参照での反復（2倍）:");
    for i in &mut v {
        *i *= 2;
    }
    println!("  結果: {:?}", v);

    // インデックス付きの反復
    println!("インデックス付き:");
    for (index, value) in v.iter().enumerate() {
        println!("  v[{}] = {}", index, value);
    }
}

/// 異なる型を格納するベクター
pub fn vector_with_enums() {
    println!("\n=== 列挙型で異なる型を格納 ===");

    // 列挙型を使えば異なる「型」の値を格納できる
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("スプレッドシートの行: {:?}", row);

    for cell in &row {
        match cell {
            SpreadsheetCell::Int(i) => println!("  整数: {}", i),
            SpreadsheetCell::Float(f) => println!("  浮動小数点: {}", f),
            SpreadsheetCell::Text(s) => println!("  テキスト: {}", s),
        }
    }
}

/// 文字列（String）の基本
pub fn string_basics() {
    println!("\n=== 文字列の基本 ===");

    // 文字列の作成
    let mut s = String::new(); // 空のString
    println!("空のString: '{}'", s);

    let s1 = "初期内容".to_string(); // &strからString
    let s2 = String::from("初期内容"); // fromで作成
    println!("to_string: '{}', from: '{}'", s1, s2);

    // 文字列の追加
    s.push_str("hello"); // 文字列スライスを追加
    s.push(' '); // 単一文字を追加
    s.push_str("world");
    println!("push後: '{}'", s);

    // +演算子での結合
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1はムーブされる、s2は借用
    // println!("{}", s1); // エラー！s1はムーブ済み
    println!("s2: '{}', s3: '{}'", s2, s3);

    // format!マクロ（所有権を奪わない）
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("format!: '{}'", s);
    println!("s1, s2, s3はまだ使える: '{}', '{}', '{}'", s1, s2, s3);
}

/// 文字列のインデックスアクセス
pub fn string_indexing() {
    println!("\n=== 文字列のインデックスアクセス ===");

    // Rustの文字列はUTF-8エンコード
    // 直接インデックスアクセスはできない

    let hello = "Здравствуйте"; // ロシア語
    println!("ロシア語: {}", hello);
    println!("バイト長: {} bytes", hello.len());

    // let s = &hello[0]; // エラー！直接インデックスは不可

    // スライスは可能だが注意が必要
    let s = &hello[0..4]; // 最初の2文字（各2バイト）
    println!("最初の2文字: {}", s);
    // let s = &hello[0..1]; // パニック！文字の途中でスライス

    // 安全な方法: chars()やbytes()を使う
    println!("文字単位での反復:");
    for c in hello.chars() {
        print!("{} ", c);
    }
    println!();

    println!("バイト単位での反復:");
    for b in hello.bytes() {
        print!("{} ", b);
    }
    println!();

    // 日本語の例
    let japanese = "こんにちは";
    println!("\n日本語: {}", japanese);
    println!("バイト長: {} bytes", japanese.len());
    println!("文字数: {} 文字", japanese.chars().count());
}

/// 文字列の操作
pub fn string_operations() {
    println!("\n=== 文字列の操作 ===");

    let s = String::from("  hello world  ");

    // トリム
    println!("トリム: '{}'", s.trim());

    // 置換
    let s = String::from("hello");
    println!("置換: '{}'", s.replace("l", "L"));

    // 分割
    let s = "one,two,three";
    println!("分割:");
    for part in s.split(',') {
        println!("  '{}'", part);
    }

    // 含むかどうか
    let s = "Hello, World!";
    println!("'World'を含む: {}", s.contains("World"));
    println!("'Hello'で始まる: {}", s.starts_with("Hello"));
    println!("'!'で終わる: {}", s.ends_with("!"));

    // 大文字・小文字変換
    println!("小文字: '{}'", s.to_lowercase());
    println!("大文字: '{}'", s.to_uppercase());

    // 行単位での反復
    let multiline = "line1\nline2\nline3";
    println!("行単位:");
    for line in multiline.lines() {
        println!("  '{}'", line);
    }
}

/// HashMap（ハッシュマップ）の基本
pub fn hashmap_basics() {
    println!("\n=== HashMapの基本 ===");

    // HashMapの作成
    let mut scores: HashMap<String, i32> = HashMap::new();

    // 要素の挿入
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("scores: {:?}", scores);

    // ベクターからHashMapを作成
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams
        .into_iter()
        .zip(initial_scores.into_iter())
        .collect();

    println!("collectで作成: {:?}", scores);

    // 値へのアクセス
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("Blueのスコア: {:?}", score);

    // getは Option<&V> を返す
    match scores.get("Blue") {
        Some(score) => println!("Blueのスコア: {}", score),
        None => println!("Blueのスコアなし"),
    }

    // copiedとunwrap_orでデフォルト値
    let score = scores.get("Blue").copied().unwrap_or(0);
    println!("Blueのスコア（デフォルト付き）: {}", score);

    // キーが存在しない場合
    let score = scores.get("Red").copied().unwrap_or(0);
    println!("Redのスコア（デフォルト付き）: {}", score);
}

/// HashMapの反復処理
pub fn hashmap_iteration() {
    println!("\n=== HashMapの反復処理 ===");

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    scores.insert(String::from("Red"), 30);

    // キーと値のペアで反復
    println!("全エントリー:");
    for (key, value) in &scores {
        println!("  {}: {}", key, value);
    }

    // キーのみ
    println!("キーのみ: {:?}", scores.keys().collect::<Vec<_>>());

    // 値のみ
    println!("値のみ: {:?}", scores.values().collect::<Vec<_>>());
}

/// HashMapの更新
pub fn hashmap_updating() {
    println!("\n=== HashMapの更新 ===");

    let mut scores = HashMap::new();

    // 値の上書き
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25); // 上書き
    println!("上書き後: {:?}", scores);

    // キーが存在しない場合のみ挿入
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50); // Blueは既存なので挿入されない
    println!("entry().or_insert()後: {:?}", scores);

    // 古い値に基づいて更新
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("単語カウント: {:?}", map);
}

/// HashMapと所有権
pub fn hashmap_ownership() {
    println!("\n=== HashMapと所有権 ===");

    // Copy トレイトを実装している型（i32など）はコピーされる
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // println!("{}", field_name); // エラー！field_nameはムーブ済み

    // 参照を挿入すれば所有権は移動しない
    let key = String::from("key");
    let value = String::from("value");

    let mut map: HashMap<&String, &String> = HashMap::new();
    map.insert(&key, &value);

    println!("参照を使用: key = '{}', value = '{}'", key, value);
    println!("map: {:?}", map);
}

/// その他のコレクション
pub fn other_collections() {
    println!("\n=== その他のコレクション ===");

    // VecDeque - 両端キュー
    use std::collections::VecDeque;
    let mut deque = VecDeque::new();
    deque.push_back(1);
    deque.push_back(2);
    deque.push_front(0);
    println!("VecDeque: {:?}", deque);
    println!("  pop_front: {:?}", deque.pop_front());
    println!("  pop_back: {:?}", deque.pop_back());

    // HashSet - 重複なしの集合
    use std::collections::HashSet;
    let mut set: HashSet<i32> = HashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(2); // 重複は無視される
    println!("HashSet: {:?}", set);
    println!("  2を含む: {}", set.contains(&2));

    // 集合演算
    let set_a: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
    let set_b: HashSet<i32> = [2, 3, 4].iter().cloned().collect();
    println!("set_a: {:?}", set_a);
    println!("set_b: {:?}", set_b);
    println!("  和集合: {:?}", set_a.union(&set_b).collect::<Vec<_>>());
    println!(
        "  積集合: {:?}",
        set_a.intersection(&set_b).collect::<Vec<_>>()
    );
    println!(
        "  差集合(a-b): {:?}",
        set_a.difference(&set_b).collect::<Vec<_>>()
    );

    // BTreeMap - 順序付きマップ
    use std::collections::BTreeMap;
    let mut btree = BTreeMap::new();
    btree.insert("c", 3);
    btree.insert("a", 1);
    btree.insert("b", 2);
    println!("BTreeMap（キー順）: {:?}", btree);
}

/// すべてのデモを実行
pub fn run_all() {
    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║          Rustコレクションサンプル                               ║");
    println!("╚════════════════════════════════════════════════════════════════╝");

    vector_basics();
    vector_operations();
    vector_iteration();
    vector_with_enums();
    string_basics();
    string_indexing();
    string_operations();
    hashmap_basics();
    hashmap_iteration();
    hashmap_updating();
    hashmap_ownership();
    other_collections();
}

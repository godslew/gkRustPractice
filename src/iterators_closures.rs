// ============================================================================
// Rustイテレータとクロージャサンプル
// 公式ドキュメント: https://doc.rust-lang.org/book/ch13-00-functional-features.html
// ============================================================================

/// クロージャの基本
pub fn closure_basics() {
    println!("\n=== クロージャの基本 ===");

    // クロージャ = 匿名関数（環境をキャプチャできる）
    let add_one = |x: i32| -> i32 { x + 1 };
    println!("add_one(5) = {}", add_one(5));

    // 型推論により注釈を省略可能
    let add_one = |x| x + 1;
    println!("型推論版 add_one(5) = {}", add_one(5));

    // 複数の引数
    let add = |a, b| a + b;
    println!("add(3, 4) = {}", add(3, 4));

    // 引数なし
    let hello = || println!("Hello from closure!");
    hello();

    // 複数行のクロージャ
    let complex = |x: i32| {
        let a = x + 1;
        let b = a * 2;
        b + 10
    };
    println!("complex(5) = {}", complex(5));
}

/// 環境のキャプチャ
pub fn closure_capture() {
    println!("\n=== 環境のキャプチャ ===");

    // 不変借用でキャプチャ（Fn）
    let x = 4;
    let equal_to_x = |z| z == x;
    println!("equal_to_x(4) = {}", equal_to_x(4));
    println!("xはまだ使える: {}", x);

    // 可変借用でキャプチャ（FnMut）
    let mut count = 0;
    let mut increment = || {
        count += 1;
        println!("  カウント: {}", count);
    };
    println!("FnMut（可変借用）:");
    increment();
    increment();
    increment();
    println!("最終カウント: {}", count);

    // 所有権を取得（FnOnce）
    let s = String::from("hello");
    let consume_string = move || {
        println!("  文字列を消費: {}", s);
        // sはこのクロージャに移動された
    };
    consume_string();
    // println!("{}", s); // エラー！sはムーブ済み

    // moveキーワード
    println!("\nmoveキーワード:");
    let x = vec![1, 2, 3];
    let contains = move |n| x.contains(n);
    println!("contains(&2) = {}", contains(&2));
    // println!("{:?}", x); // エラー！xはムーブ済み
}

/// クロージャを引数に取る関数
pub fn closures_as_parameters() {
    println!("\n=== クロージャを引数に取る関数 ===");

    // Fn - 不変参照でキャプチャ
    fn apply_fn<F>(f: F)
    where
        F: Fn(),
    {
        f();
    }

    // FnMut - 可変参照でキャプチャ
    fn apply_fn_mut<F>(mut f: F)
    where
        F: FnMut(),
    {
        f();
        f();
    }

    // FnOnce - 所有権を取る（1回だけ呼び出し可能）
    fn apply_fn_once<F>(f: F)
    where
        F: FnOnce(),
    {
        f();
    }

    println!("Fn:");
    let x = 5;
    apply_fn(|| println!("  x = {}", x));

    println!("FnMut:");
    let mut count = 0;
    apply_fn_mut(|| {
        count += 1;
        println!("  count = {}", count);
    });

    println!("FnOnce:");
    let s = String::from("hello");
    apply_fn_once(|| println!("  s = {}", s));

    // 戻り値を持つクロージャ
    fn apply_with_result<F>(f: F) -> i32
    where
        F: Fn(i32) -> i32,
    {
        f(10)
    }

    let result = apply_with_result(|x| x * 2);
    println!("apply_with_result: {}", result);
}

/// イテレータの基本
pub fn iterator_basics() {
    println!("\n=== イテレータの基本 ===");

    let v = vec![1, 2, 3];

    // イテレータの作成
    let v_iter = v.iter();

    // forループでの使用
    println!("forループ:");
    for val in v_iter {
        print!("{} ", val);
    }
    println!();

    // イテレータを手動で進める
    let mut v_iter = v.iter();
    println!("next()を手動で呼ぶ:");
    println!("  {:?}", v_iter.next()); // Some(&1)
    println!("  {:?}", v_iter.next()); // Some(&2)
    println!("  {:?}", v_iter.next()); // Some(&3)
    println!("  {:?}", v_iter.next()); // None

    // 異なるイテレータメソッド
    let v = vec![1, 2, 3];

    // iter() - 不変参照のイテレータ
    println!("iter() - &T:");
    for val in v.iter() {
        print!("{} ", val);
    }
    println!();

    // iter_mut() - 可変参照のイテレータ
    let mut v = vec![1, 2, 3];
    println!("iter_mut() - &mut T:");
    for val in v.iter_mut() {
        *val *= 2;
    }
    println!("  結果: {:?}", v);

    // into_iter() - 所有権を取るイテレータ
    let v = vec![1, 2, 3];
    println!("into_iter() - T:");
    for val in v.into_iter() {
        print!("{} ", val);
    }
    println!();
    // println!("{:?}", v); // エラー！vはムーブ済み
}

/// イテレータのアダプタ（遅延評価）
pub fn iterator_adapters() {
    println!("\n=== イテレータアダプタ ===");

    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // map - 各要素を変換
    let squared: Vec<i32> = v.iter().map(|x| x * x).collect();
    println!("map (二乗): {:?}", squared);

    // filter - 条件を満たす要素のみ
    let even: Vec<&i32> = v.iter().filter(|x| *x % 2 == 0).collect();
    println!("filter (偶数): {:?}", even);

    // take - 最初のn個
    let first_three: Vec<&i32> = v.iter().take(3).collect();
    println!("take(3): {:?}", first_three);

    // skip - 最初のn個をスキップ
    let after_five: Vec<&i32> = v.iter().skip(5).collect();
    println!("skip(5): {:?}", after_five);

    // チェーンさせる
    let result: Vec<i32> = v
        .iter()
        .filter(|x| *x % 2 == 0) // 偶数のみ
        .map(|x| x * x) // 二乗
        .take(3) // 最初の3つ
        .collect();
    println!("filter->map->take: {:?}", result);

    // enumerate - インデックス付き
    println!("enumerate:");
    for (index, value) in v.iter().enumerate().take(3) {
        println!("  v[{}] = {}", index, value);
    }

    // zip - 2つのイテレータを結合
    let a = vec![1, 2, 3];
    let b = vec!["one", "two", "three"];
    let zipped: Vec<_> = a.iter().zip(b.iter()).collect();
    println!("zip: {:?}", zipped);

    // flatten - ネストを平坦化
    let nested = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
    let flat: Vec<i32> = nested.into_iter().flatten().collect();
    println!("flatten: {:?}", flat);

    // rev - 逆順
    let reversed: Vec<&i32> = v.iter().rev().take(3).collect();
    println!("rev (最後の3つを逆順で): {:?}", reversed);
}

/// イテレータの消費アダプタ
pub fn iterator_consumers() {
    println!("\n=== イテレータ消費アダプタ ===");

    let v = vec![1, 2, 3, 4, 5];

    // collect - コレクションに収集
    let collected: Vec<i32> = v.iter().map(|x| x * 2).collect();
    println!("collect: {:?}", collected);

    // sum - 合計
    let total: i32 = v.iter().sum();
    println!("sum: {}", total);

    // product - 積
    let product: i32 = v.iter().product();
    println!("product: {}", product);

    // count - 要素数
    let count = v.iter().count();
    println!("count: {}", count);

    // min, max
    println!("min: {:?}", v.iter().min());
    println!("max: {:?}", v.iter().max());

    // fold - 畳み込み
    let sum = v.iter().fold(0, |acc, x| acc + x);
    println!("fold (sum): {}", sum);

    let product = v.iter().fold(1, |acc, x| acc * x);
    println!("fold (product): {}", product);

    // reduce - 最初の要素を初期値として使用
    let sum = v.iter().copied().reduce(|acc, x| acc + x);
    println!("reduce (sum): {:?}", sum);

    // any, all - 条件チェック
    let has_even = v.iter().any(|x| x % 2 == 0);
    let all_positive = v.iter().all(|x| *x > 0);
    println!("any (偶数あり): {}", has_even);
    println!("all (全て正): {}", all_positive);

    // find - 最初にマッチした要素
    let first_even = v.iter().find(|x| *x % 2 == 0);
    println!("find (最初の偶数): {:?}", first_even);

    // position - 最初にマッチした位置
    let position = v.iter().position(|x| *x == 3);
    println!("position (3の位置): {:?}", position);

    // for_each - 各要素に対して処理（戻り値なし）
    print!("for_each: ");
    v.iter().for_each(|x| print!("{} ", x));
    println!();
}

/// カスタムイテレータの作成
pub fn custom_iterator() {
    println!("\n=== カスタムイテレータ ===");

    // カウンターを作成
    struct Counter {
        count: u32,
        max: u32,
    }

    impl Counter {
        fn new(max: u32) -> Counter {
            Counter { count: 0, max }
        }
    }

    // Iteratorトレイトを実装
    impl Iterator for Counter {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count < self.max {
                self.count += 1;
                Some(self.count)
            } else {
                None
            }
        }
    }

    // 使用例
    let counter = Counter::new(5);
    println!("カスタムイテレータ:");
    for num in counter {
        print!("{} ", num);
    }
    println!();

    // イテレータアダプタも使える
    let sum: u32 = Counter::new(5).filter(|x| x % 2 == 0).sum();
    println!("偶数の合計: {}", sum);

    // 複雑な例: フィボナッチ数列
    struct Fibonacci {
        current: u64,
        next: u64,
    }

    impl Fibonacci {
        fn new() -> Self {
            Fibonacci {
                current: 0,
                next: 1,
            }
        }
    }

    impl Iterator for Fibonacci {
        type Item = u64;

        fn next(&mut self) -> Option<Self::Item> {
            let new_next = self.current.checked_add(self.next)?;
            self.current = self.next;
            self.next = new_next;
            Some(self.current)
        }
    }

    let fibs: Vec<u64> = Fibonacci::new().take(10).collect();
    println!("フィボナッチ数列 (最初の10個): {:?}", fibs);
}

/// イテレータとクロージャの実践例
pub fn practical_examples() {
    println!("\n=== 実践例 ===");

    // 単語カウント
    let text = "hello world hello rust world world";
    let mut word_count = std::collections::HashMap::new();
    text.split_whitespace()
        .for_each(|word| *word_count.entry(word).or_insert(0) += 1);
    println!("単語カウント: {:?}", word_count);

    // 最大値を持つ要素を見つける
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u32,
    }

    let people = vec![
        Person {
            name: "Alice".to_string(),
            age: 30,
        },
        Person {
            name: "Bob".to_string(),
            age: 25,
        },
        Person {
            name: "Charlie".to_string(),
            age: 35,
        },
    ];

    let oldest = people.iter().max_by_key(|p| p.age);
    println!("最年長: {:?}", oldest);

    // グループ化（年代別）
    let ages: Vec<_> = people.iter().map(|p| p.age / 10 * 10).collect();
    println!("年代: {:?}", ages);

    // パイプライン処理
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let result: i32 = numbers
        .iter()
        .filter(|&x| x % 2 == 0) // 偶数
        .map(|x| x * x) // 二乗
        .filter(|&x| x > 10) // 10より大きい
        .sum();
    println!("パイプライン処理結果: {}", result);

    // Option/Resultのイテレータ変換
    let options = vec![Some(1), None, Some(2), None, Some(3)];
    let values: Vec<i32> = options.into_iter().flatten().collect();
    println!("Option::flatten: {:?}", values);
}

/// すべてのデモを実行
pub fn run_all() {
    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║          Rustイテレータとクロージャサンプル                      ║");
    println!("╚════════════════════════════════════════════════════════════════╝");

    closure_basics();
    closure_capture();
    closures_as_parameters();
    iterator_basics();
    iterator_adapters();
    iterator_consumers();
    custom_iterator();
    practical_examples();
}

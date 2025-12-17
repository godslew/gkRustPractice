# Rust学習サンプル集

公式ドキュメント「[The Rust Programming Language](https://doc.rust-lang.org/book/)」に対応したRust学習用サンプルコード集です。

## 実行方法

```bash
cargo run
```

インタラクティブメニューから学習したいトピックを選択できます。

## モジュール構成

| # | モジュール | 対応章 | 主なトピック |
|---|-----------|--------|-------------|
| 1 | `basics` | Ch.3 | 変数、可変性、データ型、関数、制御フロー |
| 2 | `ownership` | Ch.4 | 所有権、ムーブ、借用、参照、スライス |
| 3 | `structs_enums` | Ch.5-6 | 構造体、メソッド、列挙型、Option、Result |
| 4 | `pattern_matching` | Ch.6, 18 | match式、if let、パターン構文 |
| 5 | `error_handling` | Ch.9 | panic!、Result、?演算子、カスタムエラー |
| 6 | `traits_generics` | Ch.10 | ジェネリクス、トレイト、トレイト境界 |
| 7 | `collections` | Ch.8 | Vec、String、HashMap、その他コレクション |
| 8 | `iterators_closures` | Ch.13 | クロージャ、イテレータ、アダプタ |
| 9 | `lifetimes` | Ch.10 | ライフタイム注釈、省略規則、'static |

## ファイル構成

```
src/
├── main.rs               # エントリーポイント（インタラクティブメニュー）
├── basics.rs             # 基本構文
├── ownership.rs          # 所有権システム
├── structs_enums.rs      # 構造体と列挙型
├── pattern_matching.rs   # パターンマッチング
├── error_handling.rs     # エラーハンドリング
├── traits_generics.rs    # トレイトとジェネリクス
├── collections.rs        # コレクション
├── iterators_closures.rs # イテレータとクロージャ
└── lifetimes.rs          # ライフタイム
```

## 学習の進め方

1. 番号順に進めることを推奨（基礎から応用へ）
2. 各ファイルのコメントで概念を理解
3. コードを実行して出力を確認
4. コードを変更して動作を実験

## 参考リンク

- [The Rust Programming Language](https://doc.rust-lang.org/book/) - 公式ドキュメント
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - 実例で学ぶRust
- [Rust Reference](https://doc.rust-lang.org/reference/) - 言語リファレンス
- [Standard Library](https://doc.rust-lang.org/std/) - 標準ライブラリドキュメント

## License

MIT

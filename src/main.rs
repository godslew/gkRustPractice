// ============================================================================
// Rust学習サンプル集
// ============================================================================
//
// このプロジェクトはRustの主要な概念を学ぶためのサンプルコード集です。
// 各モジュールは公式ドキュメント "The Rust Programming Language" に対応しています。
// https://doc.rust-lang.org/book/
//
// 実行方法:
//   cargo run
//
// 特定のモジュールのみ実行したい場合は、main関数内で該当する
// run_all() 以外をコメントアウトしてください。

// モジュール宣言
mod basics;            // 基本構文（変数、データ型、関数、制御フロー）
mod collections;       // コレクション（Vec、String、HashMap）
mod error_handling;    // エラーハンドリング（Result、panic!）
mod iterators_closures; // イテレータとクロージャ
mod lifetimes;         // ライフタイム
mod ownership;         // 所有権システム
mod pattern_matching;  // パターンマッチング
mod structs_enums;     // 構造体と列挙型
mod traits_generics;   // トレイトとジェネリクス

use std::io::{self, Write};

fn main() {
    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║                                                                ║");
    println!("║               Rust学習サンプル集                               ║");
    println!("║         The Rust Programming Language 準拠                     ║");
    println!("║                                                                ║");
    println!("╚════════════════════════════════════════════════════════════════╝");
    println!();
    println!("学習したいトピックを選択してください:");
    println!();
    println!("  1. 基本構文（変数、データ型、関数、制御フロー）");
    println!("  2. 所有権システム");
    println!("  3. 構造体と列挙型");
    println!("  4. パターンマッチング");
    println!("  5. エラーハンドリング");
    println!("  6. トレイトとジェネリクス");
    println!("  7. コレクション");
    println!("  8. イテレータとクロージャ");
    println!("  9. ライフタイム");
    println!("  0. すべて実行");
    println!("  q. 終了");
    println!();

    loop {
        print!("選択 (0-9, q): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => basics::run_all(),
            "2" => ownership::run_all(),
            "3" => structs_enums::run_all(),
            "4" => pattern_matching::run_all(),
            "5" => error_handling::run_all(),
            "6" => traits_generics::run_all(),
            "7" => collections::run_all(),
            "8" => iterators_closures::run_all(),
            "9" => lifetimes::run_all(),
            "0" => {
                basics::run_all();
                ownership::run_all();
                structs_enums::run_all();
                pattern_matching::run_all();
                error_handling::run_all();
                traits_generics::run_all();
                collections::run_all();
                iterators_closures::run_all();
                lifetimes::run_all();
            }
            "q" | "Q" => {
                println!("終了します。Happy Rusting!");
                break;
            }
            _ => {
                println!("無効な選択です。0-9 または q を入力してください。");
                continue;
            }
        }

        println!();
        println!("---");
        println!();
    }
}

// ============================================================================
// モジュール構成
// ============================================================================
//
// src/
// ├── main.rs              - エントリーポイント
// ├── basics.rs            - Ch.3: 基本的なプログラミング概念
// ├── ownership.rs         - Ch.4: 所有権
// ├── structs_enums.rs     - Ch.5-6: 構造体と列挙型
// ├── pattern_matching.rs  - Ch.6, 18: パターンマッチング
// ├── error_handling.rs    - Ch.9: エラー処理
// ├── traits_generics.rs   - Ch.10: ジェネリクスとトレイト
// ├── collections.rs       - Ch.8: コレクション
// ├── iterators_closures.rs - Ch.13: イテレータとクロージャ
// └── lifetimes.rs         - Ch.10: ライフタイム
//
// ============================================================================
// 参考リンク
// ============================================================================
//
// - The Rust Programming Language (The Book):
//   https://doc.rust-lang.org/book/
//
// - Rust by Example:
//   https://doc.rust-lang.org/rust-by-example/
//
// - Rust Reference:
//   https://doc.rust-lang.org/reference/
//
// - 標準ライブラリドキュメント:
//   https://doc.rust-lang.org/std/
//
// ============================================================================

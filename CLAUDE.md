# CLAUDE.md

## このプロジェクトについて

nozomi-koborinai の dotfiles 環境をヘルスチェックする CLI ツール。
Rust キャッチアップを兼ねた OSS プロジェクト。

- リポジトリ: https://github.com/nozomi-koborinai/dotctor
- 現時点では nozomi-koborinai/dotfiles に特化、将来的に汎用化予定

## アーキテクチャ

### Library + Binary 分離

```
src/
├── main.rs          ← CLI エントリポイント（clap で引数パース）
├── lib.rs           ← 公開 API（モジュールの re-export）
├── checks/
│   ├── mod.rs       ← Check trait + Diagnostic/Level 型の定義
│   ├── symlink.rs   ← SymlinkCheck: symlink の存在・整合性チェック
│   ├── tools.rs     ← ToolsCheck: 必須コマンドの存在チェック
│   └── version.rs   ← VersionCheck: ツールバージョンの最低要件チェック
├── config.rs        ← 設定ファイル読み込み（TOML 対応予定）
└── report.rs        ← チェック結果の表示・フォーマット
```

### Check trait（共通インターフェース）

すべてのチェッカーは `Check` trait を実装する:

```rust
pub trait Check {
    fn name(&self) -> &str;
    fn run(&self) -> Vec<Diagnostic>;
}
```

新しいチェッカーを追加するときは:
1. `src/checks/` に新しいファイルを作成
2. `Check` trait を実装した構造体を定義
3. `src/checks/mod.rs` に `pub mod` を追加
4. `src/main.rs` の checkers 一覧に追加

### データ型

```rust
pub struct Diagnostic {
    pub level: Level,    // Ok, Warn, Error
    pub message: String, // 表示メッセージ
}
```

## コマンド

```bash
cargo check              # コンパイルチェック
cargo clippy -- -D warnings  # Lint
cargo fmt --check         # フォーマット確認
cargo test                # テスト実行
cargo run                 # 全チェック実行
cargo run -- --check symlink  # 個別チェック実行
```

## 規約

- エラーハンドリングは `Result` + `?` 演算子を使う（`unwrap()` はテスト内のみ許容）
- チェック項目は将来 TOML 設定ファイルに外出しする予定
- clippy の警告はすべて対応する

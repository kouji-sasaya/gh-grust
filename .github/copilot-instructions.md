# Copilot 用指示（推奨テンプレート）

## 基本
- 言語: Rust
- コメントはすべて日本語で記述すること。
- clap の型は常に完全修飾（例: clap::Command）で書くこと。
- コマンドは動的に生成する（static macro や clap! マクロを使わない）。

## コーディング規約（必須）
- エラー型: 公開 API には Result<(), Box<dyn std::error::Error>> を使用する。
- ファイル読み込み: 実行時は実行ファイル隣接の dist/<target>/data を優先。無ければ src/command/... の組み込みデフォルトを使用する。
- キャッシュ: once_cell を使用して初期化は一度だけ行う。
- シリアライズ: serde / serde_json / serde_yaml を使用する。

## CI / ローカルチェック
- フォーマット: cargo fmt -- --check
- 静的解析: cargo clippy -- -D warnings
- テスト: cargo test を必須とする

## プロンプトの書き方（Copilot に依頼する際）
- 明確にファイルパスと関数シグネチャを指定する。
- 期待する振る舞い（例: "matches.subcommand_name() が None の場合に print_help() を呼ぶ"）を短く列挙する。
- 例: 「src/command/dog/cmd.rs に CommandNode を実装する DogCommand を追加。name()/about()/command()/add()/execute() を実装し、execute は bark/howl サブコマンドを処理するように」

## 参考（短いテンプレ）
- 「このリポジトリの規約に従って Rust で実装してください。コメントは日本語、clap::Command は完全修飾、戻り値は Result<(), Box<dyn std::error::Error> です。」
# SPECSHEAT — Specification sheet for gh-animal (for AI / implementer)

目的
- 生成AI や開発者が「現在のコード」を正確に再現・理解できるように、実装上の細かい仕様とファイルレイアウト、関数シグネチャを明記します。
- Rust 初心者や GitHub Actions 初心者にも分かるよう、重要箇所には説明を付けることを前提とします。

読者
- Rust 初心者
- clap / GitHub Actions 初心者
- 生成AI がコードを自動生成するときの参照

前提（依存）
- Rust stable（最新推奨）
- clap: 常に最新のメジャーバージョンを使用する（コード中は省略せず clap::Command を明記する）
- serde, serde_json, serde_yaml：データファイル読み込み用
- once_cell：ランタイム一回初期化用キャッシュ
- anyhow（任意）：簡易エラー管理（ただし trait の公開シグネチャは Box<dyn std::error::Error> を使う）

設計上のルール（重要）
- コマンド定義は「動的に」生成すること（静的 macro やハードコードされた clap! マクロは使わない）。
- clap の型は常に完全修飾で記述する（例: clap::Command）。
- usage/help は clap の標準機能（print_help 等）で表示すること。
- コマンドパターン（Command + Invoker）を採用する。

CommandNode トレイト（必須 I/F）
- ファイル: src/common/commandnode.rs
- 明確なシグネチャ（実装と参照の齟齬をなくす）:

```rust
// filepath: src/common/commandnode.rs
use clap::ArgMatches;
use std::error::Error;

pub trait CommandNode {
    /// コマンド名（clap::Command::new に渡す文字列）
    fn name(&self) -> &'static str;

    /// コマンドの説明（clap::Command::about に渡す文字列）
    fn about(&self) -> &'static str;

    /// このコマンドの clap::Command を返す（サブコマンド定義用）
    fn command(&self) -> clap::Command;

    /// 親コマンドに自身を追加して返すユーティリティ。
    /// 引数 parent を受け取り、parent.subcommand(self.command()) を行って返す。
    fn add(&self, parent: clap::Command) -> clap::Command;

    /// コマンド実行。ArgMatches（サブコマンドの matches）を受け取りエラーを返す
    fn execute(&self, matches: &ArgMatches) -> Result<(), Box<dyn Error>>;
}
```

コマンド実装（Concrete Commands）
- それぞれ CommandNode を実装する。
- 実装ファイル:
  - src/command/dog/cmd.rs (DogCommand の CommandNode 実装)
  - src/command/dog/dog.rs (犬の鳴き声ロジック・非公開)
  - src/command/cat/cmd.rs (CatCommand の CommandNode 実装)
  - src/command/cat/cat.rs (猫の鳴き声ロジック・非公開)
  - src/command/fox/cmd.rs (FoxCommand の CommandNode 実装)
  - src/command/fox/fox.rs (狐の鳴き声ロジック・非公開)
  - src/command/rat/cmd.rs (RatCommand の CommandNode 実装)
  - src/command/rat/rat.rs (rat ロジック)

コマンド仕様（CLI）
- gh animal dog bark
- gh animal dog howl
- gh animal cat meow
- gh animal cat hiss
- gh animal fox yip
- gh animal fox cry
- gh animal rat squeak
- gh animal rat scurry
- gh animal（トップ）でサブコマンド指定なし → ヘルプ表示
- gh animal <unknown> → top-level help（またはエラーメッセージ＋ヘルプ）

Invoker（Composite）
- ファイル: src/command/animal.rs
- 型名: AnimalCommand（Invoker であり、トップコマンド）
- AnimalCommand も CommandNode を実装する（Composite）。内部に Vec<Box<dyn CommandNode>> handlers を持ち、command() は各 handler.command() を subcommand として追加した clap::Command を返す。
- AnimalCommand の execute は top-level の matches を受け、matches.subcommand_name() で該当 handler を探して handler.execute(sub_matches) を呼ぶ。

データファイルの位置と配布ルール
- 犬の鳴き声（開発時の定義）: src/command/dog/dog.yml
- 猫の鳴き声: src/command/cat/cat.json
- 狐とネズミのデータは release 用に data/ 配下に置く:
  - data/fox.yml
  - data/rat.json
- ランタイムの読み込み方（実装ルール）:
  1. 実行時は「実行ファイルの隣にある data/ ディレクトリ（dist/<target>/data）」を優先して読み込む。
  2. 見つからなければ、ソースツリー内の src/command/... の組み込みデフォルトを利用する（開発中のフォールバック）。
- リリース手順: build.sh は dist/<target>/ を作成し、バイナリと data/ をコピーして同梱する（ワークフローはこの dist を zip 化してリリースに添付する）。

ファイルレイアウト（最終）
- src/main.rs
- src/common/commandnode.rs
- src/command/mod.rs
- src/command/animal.rs
- src/command/dog/cmd.rs
- src/command/dog/dog.rs
- src/command/dog/dog.yml
- src/command/cat/cmd.rs
- src/command/cat/cat.rs
- src/command/cat/cat.json
- src/command/fox/cmd.rs
- src/command/fox/fox.rs
- src/command/rat/cmd.rs
- src/command/rat/rat.rs
- data/fox.yml
- data/rat.json
- build.sh

実装の注意点（clap）
- 動的生成: top-level clap::Command は AnimalCommand::command() が生成する。サブコマンドは各 handler.command() を追加する。
- clap::Command は必ず完全修飾で記述する（例: clap::Command::new(self.name()).about(self.about()).subcommand(...)）。
- サブコマンド未指定時の help 表示: 各 CommandNode::execute は matches.subcommand_name() を確認し、None の場合は自身の clap::Command に対して print_help() を呼ぶ。
- ArgMatches をサブコマンド単位で渡す：AnimalCommand は matches.subcommand_matches(sub) を handler.execute に渡す。

エラー型と戻り値
- 公開トレイトの実行メソッドは互換性確保のため下記のシグネチャを推奨:
  - fn execute(&self, matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>>

VSCode デバッグ
- .vscode/launch.json を用意して、cargo run の引数（例: ["animal", "dog", "bark"]）でデバッグできるように設定する。コメントで各フィールド（program, args, cwd）を説明すること。

CI / ワークフロー（概要）
- build.yml (on: push)
  - matrix: ubuntu-latest, windows-latest
  - steps:
    - checkout
    - set up rust (rustup)
    - cargo fmt -- --check
    - cargo clippy -- -D warnings
    - run build.sh for respective target
    - upload artifact (dist/<target>)
- release.yml (on: push tags: v*)
  - matrix: same targets
  - build artifact zip(s)
  - create GitHub Release and upload zip artifacts

補足（実装例の短いサンプル）
- AnimalCommand の app/command 構築は次のパターン：

```rust
// snippet (説明用)
let mut app = clap::Command::new("animal").about("animal commands");
for h in &self.handlers {
    app = h.add(app);
}
```

- add の実装は単純に parent.subcommand(self.command())

文体とコメント方針
- Rust 初心者のため、実装ファイル内の重要行（trait定義, invokeループ, clap の subcommand 追加, ArgMatches の扱い, ファイル読み込みロジック）には丁寧なコメントを付けること。
- GitHub Actions の workflow ファイルにも、matrix や artifact の説明コメントを入れること。



// filepath: src/common/commandnode.rs
// CommandNode トレイト: すべてのコマンドが実装すべき共通インターフェース
// Command パターンの基盤となる trait です

use clap::ArgMatches;
use std::error::Error;

/// CommandNode トレイト
/// 各コマンド（Dog, Cat, Fox, Rat）およびトップレベルの AnimalCommand が実装します。
/// これにより、Composite パターンで階層的なコマンド構造を実現できます。
pub trait CommandNode {
    /// コマンド名（clap::Command::new に渡す文字列）
    /// 例: "dog", "cat", "animal" など
    fn name(&self) -> &'static str;

    /// コマンドの説明（clap::Command::about に渡す文字列）
    /// ヘルプメッセージに表示されます
    fn about(&self) -> &'static str;

    /// このコマンドの clap::Command を返す（サブコマンド定義用）
    /// 各コマンドは自身のサブコマンド構造を定義して返します
    fn command(&self) -> clap::Command;

    /// 親コマンドに自身を追加して返すユーティリティ
    /// 引数 parent を受け取り、parent.subcommand(self.command()) を行って返します
    /// これにより、コマンドツリーの構築が簡潔になります
    fn add(&self, parent: clap::Command) -> clap::Command {
        // デフォルト実装: 親コマンドに自身のコマンド定義を subcommand として追加
        parent.subcommand(self.command())
    }

    /// コマンド実行
    /// ArgMatches（サブコマンドの matches）を受け取り、実際の処理を行います
    /// エラーが発生した場合は Box<dyn Error> を返します
    fn execute(&self, matches: &ArgMatches) -> Result<(), Box<dyn Error>>;
}

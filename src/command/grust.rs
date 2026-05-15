// src/command/grust.rs
// GrustCommand: トップレベルの Invoker（Composite パターン）

use crate::common::commandnode::CommandNode;
use clap::ArgMatches;
use std::error::Error;

/// GrustCommand: すべての GUI コマンドを統括する Invoker
/// Composite パターンを採用し、内部に複数の CommandNode を保持します
pub struct GrustCommand {
    /// 各 GUI コマンドへの参照（Box<dyn CommandNode> で多態性を実現）
    handlers: Vec<Box<dyn CommandNode>>,
}

impl GrustCommand {
    /// 新しい GrustCommand を作成
    /// handlers には各 GUI コマンド（gui-a 〜 gui-f）を登録します
    pub fn new(handlers: Vec<Box<dyn CommandNode>>) -> Self {
        GrustCommand { handlers }
    }
}

impl CommandNode for GrustCommand {
    fn name(&self) -> &'static str {
        "grust"
    }

    fn about(&self) -> &'static str {
        "Rust GUI サンプル集（各種アーキテクチャのウィンドウデモ）"
    }

    fn command(&self) -> clap::Command {
        // 完全修飾で clap::Command を記述
        // トップレベルのコマンドを作成し、すべての handlers を subcommand として追加
        let mut app = clap::Command::new(self.name())
            .about(self.about())
            .version("0.1.0")
            // ヘルプサブコマンドを無効化（-h/--help は残す）
            .disable_help_subcommand(true);

        // 各 handler の command() を subcommand として追加
        // これにより動的にコマンドツリーが構築されます
        for handler in &self.handlers {
            app = handler.add(app);
        }

        app
    }

    fn execute(&self, matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
        // サブコマンド名を取得
        match matches.subcommand() {
            Some((subcommand_name, sub_matches)) => {
                // 該当する handler を探して実行
                for handler in &self.handlers {
                    if handler.name() == subcommand_name {
                        return handler.execute(sub_matches);
                    }
                }
                // 該当する handler が見つからない場合
                eprintln!("Unknown subcommand: {}", subcommand_name);
                self.command().print_help()?;
                println!();
            }
            None => {
                // サブコマンドが指定されていない場合はヘルプを表示
                self.command().print_help()?;
                println!();
            }
        }
        Ok(())
    }
}

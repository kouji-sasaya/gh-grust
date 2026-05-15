// src/main.rs
// gh-grust のエントリポイント

mod command;
mod common;

use command::grust::GrustCommand;
use command::gui_a::cmd::GuiACommand;
use command::gui_b::cmd::GuiBCommand;
use command::gui_c::cmd::GuiCCommand;
use command::gui_d::cmd::GuiDCommand;
use command::gui_e::cmd::GuiECommand;
use command::gui_f::cmd::GuiFCommand;
use command::gui_g::cmd::GuiGCommand;
use command::gui_h::cmd::GuiHCommand;
use command::gui_i::cmd::GuiICommand;
use command::gui_j::cmd::GuiJCommand;
use command::gui_k::cmd::GuiKCommand;
use common::commandnode::CommandNode;
use std::process;

fn main() {
    // 各 GUI コマンドのインスタンスを作成
    // Box<dyn CommandNode> として格納することで、多態性を実現
    let handlers: Vec<Box<dyn CommandNode>> = vec![
        Box::new(GuiACommand::new()),
        Box::new(GuiBCommand::new()),
        Box::new(GuiCCommand::new()),
        Box::new(GuiDCommand::new()),
        Box::new(GuiECommand::new()),
        Box::new(GuiFCommand::new()),
        Box::new(GuiGCommand::new()),
        Box::new(GuiHCommand::new()),
        Box::new(GuiICommand::new()),
        Box::new(GuiJCommand::new()),
        Box::new(GuiKCommand::new()),
    ];

    // GrustCommand (Invoker) を作成
    // handlers を渡すことで、すべての GUI コマンドを統括
    let grust_cmd = GrustCommand::new(handlers);

    // clap::Command を動的に構築
    // grust_cmd.command() がすべてのサブコマンドを含むコマンドツリーを返す
    let app = grust_cmd.command();

    // コマンドライン引数をパース
    let matches = app.get_matches();

    // コマンドを実行
    // matches を grust_cmd.execute() に渡して、適切なハンドラに委譲
    if let Err(e) = grust_cmd.execute(&matches) {
        // エラーが発生した場合はエラーメッセージを表示して終了
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

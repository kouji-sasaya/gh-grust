fn main() {
    // gui-c 用の Slint UI 定義をビルド時に Rust コードへ変換する。
    slint_build::compile("src/command/gui_c/main.slint").expect("failed to compile gui-c slint ui");
}

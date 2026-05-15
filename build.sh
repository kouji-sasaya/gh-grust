#!/usr/bin/env bash
# build.sh - ビルドスクリプト
# このスクリプトは以下を行います：
# 1. Rust プロジェクトをビルド
# 2. dist/<target>/ ディレクトリを作成
# 3. バイナリと data/ ディレクトリをコピー
# 4. リリース用のパッケージを準備

set -e  # エラーが発生したら即座に終了

# プラットフォームの検出
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    PLATFORM="linux"
    EXE_EXT=""
elif [[ "$OSTYPE" == "darwin"* ]]; then
    PLATFORM="macos"
    EXE_EXT=""
elif [[ "$OSTYPE" == "msys" || "$OSTYPE" == "win32" ]]; then
    PLATFORM="windows"
    EXE_EXT=".exe"
else
    PLATFORM="unknown"
    EXE_EXT=""
fi

# アーキテクチャの検出
ARCH=$(uname -m)
if [[ "$ARCH" == "x86_64" ]]; then
    ARCH="x64"
elif [[ "$ARCH" == "aarch64" || "$ARCH" == "arm64" ]]; then
    ARCH="arm64"
fi

TARGET="${PLATFORM}-${ARCH}"
echo "Building for: $TARGET"

# スクリプトのあるディレクトリに移動（以前は script/ 以下だったが現在はルート）
# dirname "$0" は相対パスになるので、移動先を正規化するために cd する
cd "$(dirname "$0")"

# コード品質チェック
# - フォーマットが正しいか確認
# - clippy で警告をエラー扱い
echo "Running formatter and linter..."
cargo fmt -- --check
cargo clippy -- -D warnings

# ビルド実行（リリースモード）
echo "Building release binary..."
cargo build --release

# dist ディレクトリの作成
DIST_DIR="dist/${TARGET}"
echo "Creating distribution directory: ${DIST_DIR}"
mkdir -p "${DIST_DIR}/data"

# バイナリのコピー
echo "Copying binary..."
cp "target/release/gh-animal${EXE_EXT}" "${DIST_DIR}/"

# data/ ディレクトリのコピー
echo "Copying data files..."
cp data/fox.yml "${DIST_DIR}/data/"
cp data/rat.json "${DIST_DIR}/data/"

echo "Build complete! Output: ${DIST_DIR}"
echo "Binary: ${DIST_DIR}/gh-animal${EXE_EXT}"
echo "Data files:"
echo "  - ${DIST_DIR}/data/fox.yml"
echo "  - ${DIST_DIR}/data/rat.json"

# gh-animal

A GitHub CLI extension for animal-related functionality.

## How to this command


gh extension create を実行して、コマンド(gh-animal)を入力します。

Rust 言語でコマンド作成するので、Rust を選択します。

```bash
$ gh extension create
? Extension name: gh-animal
? What kind of extension? Other Precompiled (C++, Rust, etc)
```

GitHGub copilopt のエージェントに、以下を指示します。

<!--ここで、SPECSHEAT.mdを参照-->
-->
## Specification (参照)

詳細な設計・仕様はリポジトリ内の SPECSHEAT.md を参照してください。

- ローカルファイル: [SPECSHEAT.md](SPECSHEAT.md)
- 要点（抜粋）:
  - 提供コマンド: gh animal {dog,cat,fox,rat} とそれぞれのサブコマンド
  - 配布: release アーティファクトにバイナリ + data/ を同梱
  - ビルド: ./build.sh を使用して dist/<target> を作成
  - 実行時: 実行ファイル隣の data/ を優先して読み込む

（詳細は SPECSHEAT.md を参照してください）

## Installation

To install this GitHub CLI extension, run:

```bash
gh extension install kouji-sasaya/gh-animal
```

## Usage

```bash
gh animal [command] [options]
```

### Commands




<!-- Add your specific commands here -->

## Development

### Prerequisites

- [GitHub CLI](https://cli.github.com/) installed
- Bash shell environment

### Building

This extension uses a build script to create platform-specific binaries:

```bash
./build.sh
```

The build script will generate binaries in `dist/<platform>-<arch>[.exe]` format.

### Project Structure

```
.
├── .github/
│   └── workflows/
│       └── release.yml    # GitHub Actions workflow for releases
├── build.sh              # Build script for creating binaries
└── README.md             # This file
```

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Release Process

This extension uses GitHub Actions for automated releases. When you push a tag starting with `v` (e.g., `v1.0.0`), the release workflow will automatically:

1. Build the extension using the build script
2. Create platform-specific binaries
3. Publish a new release

To create a new release:

```bash
git tag v1.0.0
git push origin v1.0.0
```

## License

<!-- Add your license information here -->

## Support

If you encounter any issues or have questions, please [open an issue](../../issues) on GitHub.
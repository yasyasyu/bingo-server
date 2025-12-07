# Christmas Bingo System 🎄

クリスマスイベントを盛り上げるための、リッチな演出付きビンゴ抽選システムです。
Rust (Axum) による堅牢なバックエンドと、Vue.js によるインタラクティブなフロントエンドで構成されています。

## 📚 ドキュメント

プロジェクトの詳細な情報は `docs/` ディレクトリに集約されています。

| ドキュメント | 説明 | 対象読者 |
| :--- | :--- | :--- |
| [📄 仕様書 (Specification)](docs/SPECIFICATION.md) | システムの機能要件、画面構成、全体概要。 | 全員 |
| [📖 操作マニュアル (User Manual)](docs/USER_MANUAL.md) | 当日のオペレーター・司会者向けの操作手順書。 | 利用者 |
| [🏗️ アーキテクチャ設計書 (Architecture)](docs/ARCHITECTURE.md) | クラス図、シーケンス図を用いた内部設計の詳細。 | 開発者 |
| [🔌 API仕様書 (API Spec)](docs/API_SPEC.md) | バックエンドAPIのエンドポイント定義と使用例。 | 開発者 |
| [📂 ディレクトリ構造 (Directory Structure)](docs/DIRECTORY_STRUCTURE.md) | ファイル構成と各モジュールの責務解説。 | 開発者 |
| [🚀 デプロイメントガイド (Deployment)](docs/DEPLOYMENT.md) | 実行ファイルのビルドと、本番環境への配置手順。 | インフラ/運用 |

## 🚀 クイックスタート (開発者向け)

開発環境（Rust, Node.js）が整っている場合の起動手順です。
実行ファイルのみでの起動方法は [デプロイメントガイド](docs/DEPLOYMENT.md) を参照してください。

### 前提条件
- **Rust**: 最新の安定版 (cargo)
- **Node.js**: v18以上 (npm)

### 起動方法

本システムはバックエンドとフロントエンドを別々のターミナルで起動する必要があります。

#### 1. バックエンドの起動
```bash
cd backend
cargo run
```
サーバーが `http://0.0.0.0:3000` で起動します。

#### 2. フロントエンドの起動
別のターミナルを開いて実行してください。
```bash
cd frontend
npm install
npm run dev
```
ブラウザで表示されるURL（通常は `http://localhost:5173`）にアクセスしてください。

### 機能へのアクセス
- **ビンゴ**: `http://localhost:5173/`
- **あみだくじ**: `http://localhost:5173/amida`

## 🛠️ 技術スタック
- **Backend**: Rust, Axum, Tokio
- **Frontend**: Vue.js 3, TypeScript, Vite
- **Audio**: Web Audio API

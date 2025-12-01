# Christmas Bingo System 🎄

クリスマスイベントを盛り上げるための、リッチな演出付きビンゴ抽選システムです。
Rust (Axum) による堅牢なバックエンドと、Vue.js によるインタラクティブなフロントエンドで構成されています。

## 📚 ドキュメント

詳細な仕様や設計については `docs/` ディレクトリ内のドキュメントを参照してください。

- [仕様書 (Specification)](docs/SPECIFICATION.md): 機能要件、システム概要
- [API仕様書 (API Spec)](docs/API_SPEC.md): バックエンドAPIのエンドポイント定義
- [アーキテクチャ設計書 (Architecture)](docs/ARCHITECTURE.md): UML図を用いた設計図
- [ディレクトリ構造 (Directory Structure)](docs/DIRECTORY_STRUCTURE.md): ファイル構成と責務の詳細

## 🚀 クイックスタート

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

## 🛠️ 技術スタック
- **Backend**: Rust, Axum, Tokio
- **Frontend**: Vue.js 3, TypeScript, Vite
- **Audio**: Web Audio API

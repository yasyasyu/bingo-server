# ディレクトリ構造とファイル解説

プロジェクトの主要なディレクトリとファイルの役割について解説します。

## 📂 プロジェクトルート (`/`)

| パス | 説明 |
| :--- | :--- |
| `backend/` | RustによるAPIサーバーのソースコード |
| `frontend/` | Vue.jsによるフロントエンドアプリケーションのソースコード |
| `docs/` | 設計書、仕様書などのドキュメント群 |
| `README.md` | プロジェクトの概要とセットアップ手順 |

## 📂 バックエンド (`backend/`)

Rust (Axum) で実装されたREST APIサーバーです。

| パス | 説明 |
| :--- | :--- |
| `Cargo.toml` | 依存関係の定義ファイル (package.jsonのようなもの) |
| `src/main.rs` | **エントリーポイント**。サーバーの起動、ルーティング、CORS設定を行います。 |
| `src/domain.rs` | **ドメイン層**。`BingoGame`, `AmidaGame` 構造体など、純粋なビジネスロジック（抽選、リセット）を記述しています。Webフレームワークには依存しません。 |
| `src/state.rs` | **状態管理**。アプリケーション全体で共有する状態 (`AppState`) を定義し、スレッドセーフに管理します。 |
| `src/handlers.rs` | **インターフェース層**。HTTPリクエストを受け取り、ドメインロジックを実行してJSONレスポンスを返します。 |
| `src/rng.rs` | **乱数生成**。乱数生成トレイト (`IRng`) と XorShift アルゴリズムの実装。 |
| `seeds.txt` | 乱数シード設定ファイル（Git管理外）。 |
| `seeds.sample.txt` | 乱数シード設定のサンプルファイル。 |

## 📂 フロントエンド (`frontend/`)

Vue.js + TypeScript (Vite) で実装されたSPAです。

| パス | 説明 |
| :--- | :--- |
| `index.html` | アプリケーションのエントリーポイントとなるHTML |
| `src/main.ts` | Vueアプリケーションの初期化とマウント |
| `src/App.vue` | ルートコンポーネント。各コンポーネントのレイアウトを行います。 |
| `src/components/` | **UIコンポーネント**。見た目と単純なUI操作を担当します。 |
| ├── `BingoDisplay.vue` | 中央の数字表示パネル |
| ├── `BingoControls.vue` | SPIN/RESETボタン |
| ├── `BingoHistory.vue` | 抽選履歴のリスト表示 |
| ├── `SnowEffect.vue` | 背景の雪のアニメーション |
| ├── `AmidaSetup.vue` | あみだくじ設定（名前入力）フォーム |
| └── `AmidaBoard.vue` | あみだくじ描画・アニメーション・結果表示 |
| `src/views/` | **ページコンポーネント**。ルーティングに対応する画面全体。 |
| ├── `BingoView.vue` | ビンゴゲーム画面 |
| └── `AmidaView.vue` | あみだくじ画面（設定・ゲーム） |
| `src/composables/` | **ロジック層**。VueのComposition APIを使用した再利用可能なロジック。 |
| ├── `useBingoGame.ts` | ゲームの進行管理、状態保持、API呼び出しの制御 |
| ├── `useAmida.ts` | あみだくじのデータ取得・更新ロジック |
| └── `useAudio.ts` | Web Audio APIを使用した効果音の再生制御 |
| `src/services/` | **インフラ層**。外部システム（API）との通信を担当。 |
| └── `bingoApi.ts` | バックエンドAPIへのfetch処理をカプセル化 |

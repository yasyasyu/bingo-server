# アーキテクチャ設計書 (UML)

本ドキュメントでは、クリスマスビンゴシステムのアーキテクチャと設計をUML（Mermaid記法）を用いて可視化します。

## 1. システム全体構成 (コンポーネント図)

フロントエンドとバックエンドの疎結合な構成と、各層の依存関係を示します。

```mermaid
graph TD
    subgraph Frontend["Frontend (Vue.js)"]
        UI["UI Components"]
        Logic["Composables (Logic)"]
        Service["Services (Infrastructure)"]
    end

    subgraph Backend["Backend (Rust)"]
        Handler["Handlers (Presentation)"]
        State["AppState (State Management)"]
        Domain["Domain (Business Logic)"]
    end

    UI -->|Uses| Logic
    Logic -->|Uses| Service
    Service -->|HTTP/JSON REST API| Handler
    Handler -->|Access| State
    State -->|Holds| Domain
```

## 2. バックエンド詳細設計 (クラス図)

Rustバックエンドにおけるレイヤードアーキテクチャの実装詳細です。
`Handlers` は `AppState` を介して `BingoGame` ドメインオブジェクトにアクセスします。

```mermaid
classDiagram
    title Backend Class Structure

    class AppState {
        +Arc~Mutex~BingoGame~~ game
        +Arc~Mutex~AmidaGame~~ amida
        +u32 seed
        +new(seed: u32) Self
    }

    class IRng {
        <<Trait>>
        +next() u32
        +shuffle(slice: &mut [u8])
        +reset()
    }

    class XorShift {
        -u32 initial_state
        -u32 state
        +new(seed: u32) Self
        +next() u32
        +reset()
    }

    class MersenneTwister {
        -u32 initial_seed
        -u32[624] mt
        -usize index
        +new(seed: u32) Self
        +next() u32
        +reset()
    }

    class BingoGame {
        -usize count
        +Vec~u8~ remaining_numbers
        +Vec~u8~ history
        -Box~dyn IRng~ rng
        +new(count: usize, rng: Box~dyn IRng~) Self
        -shuffle()
        +get_next_number() Option~u8~
        +reset()
    }

    class AmidaGame {
        -usize count
        +Vec~String~ gests
        +Vec~u8~ prizes
        -Box~dyn IRng~ rng
        +new(count: usize, rng: Box~dyn IRng~) Self
        -shuffle()
        +update(gests: Vec~String~)
        +get_result() Option~Vec~tuple~~
    }

    class Handlers {
        <<Module>>
        +get_next_number(State) Json
        +reset_game(State) Json
        +get_amida(State) Json
        +set_amida(State) Json
        +get_amida_result(State) Json
    }
    
    class NumberResponse {
        <<Struct>>
        +Option~u8~ number
        +Vec~u8~ history
        +String message
        +u32 seed
    }

    class AmidaResponse {
        <<Struct>>
        +Vec~String~ items
        +String message
        +u32 seed
    }

    class AmidaResultResponse {
        <<Struct>>
        +Vec~tuple~ items
        +String message
        +u32 seed
    }

    Handlers ..> AppState : Uses via Axum State
    Handlers ..> NumberResponse : Returns
    Handlers ..> AmidaResponse : Returns
    Handlers ..> AmidaResultResponse : Returns
    AppState o-- BingoGame : Contains (Thread Safe)
    AppState o-- AmidaGame : Contains (Thread Safe)
    BingoGame o-- IRng : Depends on (DI)
    AmidaGame o-- IRng : Depends on (DI)
    XorShift ..|> IRng : Implements
    MersenneTwister ..|> IRng : Implements
```

## 3. バックエンド実装詳細

Rustバックエンドの主要な設計判断と実装パターンについて解説します。

### 3.1. 並行処理と状態管理 (Concurrency & State Management)

本システムは `Axum` (Webフレームワーク) と `Tokio` (非同期ランタイム) 上で動作します。
HTTPリクエストは非同期に並行処理されるため、アプリケーションの状態 (`AppState`) はスレッドセーフである必要があります。

*   **`Arc<Mutex<T>>` パターン**:
    *   `AppState` は `Arc` (Atomic Reference Counting) でラップされ、複数のスレッド（リクエストハンドラ）間で共有されます。
    *   内部の可変な状態 (`BingoGame`, `AmidaGame`) は `Mutex` で保護されています。
    *   ハンドラ内で `state.game.lock().unwrap()` を呼び出すことで、一時的に排他ロックを取得し、安全に状態を更新します。

### 3.2. 依存性の注入 (Dependency Injection)

テスト容易性と拡張性を高めるため、乱数生成器 (`IRng`) はトレイトとして定義され、ドメインロジック (`BingoGame`, `AmidaGame`) に注入されます。

*   **本番環境**: `XorShift` や `MersenneTwister` などの実装を使用。
*   **テスト環境**: 固定の値を返すモックや、特定のシードで初期化された乱数生成器を使用することで、決定論的なテストが可能になります。

### 3.3. エラーハンドリング方針

*   **パニック (Panic)**:
    *   `Mutex` のロック取得失敗時 (`.lock().unwrap()`) は、スレッドが汚染されている状態（Poisoned）を意味するため、パニックさせてリクエストを失敗させます（Axumが500エラーとして処理）。
    *   起動時の必須ファイル (`seeds.txt`) 読み込みエラーなどは、ログを出力してデフォルト値で続行するか、致命的な場合は停止します。
*   **Result型**:
    *   ドメインロジック内での期待されるエラー（例：あみだくじの参加者数が足りない）は `Option` や `Result` を返して呼び出し元に通知します。

## 4. フロントエンド詳細設計 (クラス図)

Vue.jsフロントエンドのコンポーネント構成とロジックの分離を示します。
`App.vue` が各コンポーネントを統合し、`useBingoGame` コンポーザブルがビジネスロジックを提供します。

```mermaid
classDiagram
    title Frontend Component and Logic Structure

    class App {
        <<View>>
    }
    
    class Router {
        <<Router>>
        /
        /amida
        /amida/result
    }

    class BingoView {
        <<View>>
    }

    class AmidaView {
        <<View>>
    }

    class BingoDisplay {
        <<Component>>
    }

    class BingoControls {
        <<Component>>
    }

    class BingoHistory {
        <<Component>>
    }

    class AmidaSetup {
        <<Component>>
    }

    class AmidaBoard {
        <<Component>>
    }

    class useBingoGame {
        <<Composable>>
        +Ref~number~ currentNumber
        +Ref~string~ displayText
        +Ref~array~ history
        +Ref~boolean~ isSpinning
        +spin()
        +resetGame()
    }

    class useAmida {
        <<Composable>>
        +Ref~array~ items
        +Ref~boolean~ isConfigured
        +fetchAmida()
        +setupAmida()
        +fetchResults()
    }

    class useAmidaGame {
        <<Composable>>
        +Ref~array~ horizontalLines
        +Ref~array~ bottomPrizes
        +generateAmida()
        +calculatePrizes()
    }

    class useAudio {
        <<Composable>>
        +playBeep()
        +playWin()
        +resumeAudioContext()
    }

    class useDrumRoll {
        <<Composable>>
        +play()
        +stop()
        +playCymbal()
    }

    class bingoApi {
        <<Service>>
        +fetchNextNumber()
        +resetGame()
    }

    class amidaApi {
        <<Service>>
        +fetchSettings()
        +updateSettings()
        +fetchResults()
    }

    App *-- Router
    Router --> BingoView
    Router --> AmidaView
    BingoView *-- BingoDisplay
    BingoView *-- BingoControls
    BingoView *-- BingoHistory
    BingoView ..> useBingoGame : Uses
    AmidaView *-- AmidaSetup
    AmidaView *-- AmidaBoard
    AmidaView ..> useAmida : Uses
    AmidaBoard ..> useAmidaGame : Uses
    AmidaBoard ..> useDrumRoll : Uses
    useBingoGame ..> bingoApi : Uses
    useBingoGame ..> useAudio : Uses
    useAmida ..> amidaApi : Uses
```

## 4. 処理フロー (シーケンス図)

ユーザーが「SPIN」ボタンを押してから、抽選が行われ、結果が表示されるまでの時系列フローです。

```mermaid
sequenceDiagram
    title Spin Action Flow
    actor User
    participant UI as Vue Component
    participant Logic as useBingoGame
    participant Audio as useAudio
    participant API as bingoApi
    participant Server as Backend Handler
    participant Domain as BingoGame

    User->>UI: Click "SPIN" Button
    UI->>Logic: spin()
    
    rect rgb(240, 248, 255)
        note right of Logic: Initialization
        Logic->>Audio: resumeAudioContext()
        Logic->>Logic: isSpinning = true
    end

    rect rgb(255, 240, 245)
        note right of Logic: Data Fetching
        Logic->>API: fetchNextNumber()
        API->>Server: GET /next_number
        Server->>Domain: draw_number()
        Domain-->>Server: number (e.g. 42)
        Server-->>API: JSON Response
        API-->>Logic: { number: 42, history: [...] }
    end
    
    rect rgb(240, 255, 240)
        note right of Logic: Animation Phase (3 sec)
        loop Every Frame
            Logic->>UI: Update Display (Random Number)
            Logic->>Audio: playBeep()
        end
    end

    rect rgb(255, 250, 205)
        note right of Logic: Finalization
        Logic->>UI: Update Display (42)
        Logic->>Audio: playWin()
        Logic->>Logic: isSpinning = false
    end
```

## 5. あみだくじ処理フロー (シーケンス図)

あみだくじの設定から結果取得までのフローです。
結果（誰がどの番号になるか）はサーバーサイドで決定されます。

```mermaid
sequenceDiagram
    title Amidakuji Setup & Play Flow
    actor User
    participant UI as AmidaBoard
    participant Logic as useAmida
    participant Audio as useDrumRoll
    participant API as amidaApi
    participant Server as Backend Handler
    participant Domain as AmidaGame

    note over User, Domain: Phase 1: Setup
    User->>UI: Input Name (Blur)
    UI->>Logic: setupAmida(names)
    Logic->>API: updateSettings(names)
    API->>Server: POST /amida
    Server->>Domain: update(names)
    Domain-->>Server: ok
    Server-->>API: { items: [...], message: "Updated" }
    API-->>Logic: Success

    note over User, Domain: Phase 2: Start Game
    User->>UI: Click "Start Game"
    UI->>Logic: fetchResults()
    Logic->>API: fetchResults()
    API->>Server: GET /amida/result
    Server->>Domain: get_result()
    Domain-->>Server: [(GuestA, 3), (GuestB, 1)...]
    Server-->>API: { items: [...], message: "Success" }
    API-->>Logic: Results
    Logic->>UI: Navigate if valid

    note over User, Domain: Phase 3: Play
    UI->>UI: generateAmida() (Random Lines)
    UI->>UI: calculatePrizes() (Map Results to Path)
    User->>UI: Click Prize Number Button
    UI->>Logic: startAnimation()
    Logic->>Audio: playDrum(isDual)
    UI->>UI: Animate Path (Canvas)
    Logic->>Audio: stopDrum()
    Logic->>Audio: playCymbal()
    UI->>UI: Show Guest at Goal

```

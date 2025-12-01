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
        +new() Self
    }

    class IRng {
        <<Trait>>
        +next_u32() u32
        +shuffle(slice: &mut [u8])
        +shift(shift: usize)
    }

    class XorShift {
        -u32 state
        +new(seed: u32) Self
        +next_u32() u32
        +shuffle(slice: &mut [u8])
        +shift(shift: usize)
    }

    class BingoGame {
        +Vec~u8~ remaining_numbers
        +Vec~u8~ history
        -Box~dyn IRng~ rng
        +new(count: usize, rng: Box~dyn IRng~) Self
        -shuffle()
        +get_next_number() Option~u8~
        +reset()
    }

    class AmidaGame {
        +Vec~String~ gests
        +Vec~u8~ items
        -Box~dyn IRng~ rng
        +new(count: usize, rng: Box~dyn IRng~) Self
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
    }

    class AmidaResultResponse {
        <<Struct>>
        +Vec~tuple~ items
        +String message
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
```

## 3. フロントエンド詳細設計 (クラス図)

Vue.jsフロントエンドのコンポーネント構成とロジックの分離を示します。
`App.vue` が各コンポーネントを統合し、`useBingoGame` コンポーザブルがビジネスロジックを提供します。

```mermaid
classDiagram
    title Frontend Component and Logic Structure

    class App {
        <<View>>
    }
    
    class Router {
        /
        /amida
        /amida/result
    }

    class BingoView {
        <<View>>
    }

    class AmidaView {
        <<View>>
        +setupMode
        +gameMode
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
    AmidaView ..> useAmida : Uses
    AmidaView ..> useAmidaGame : Uses
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
    participant UI as AmidaView
    participant Logic as useAmida
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
    User->>UI: Click Guest Button
    UI->>UI: Animate Path
    UI->>UI: Show Prize at Goal
```

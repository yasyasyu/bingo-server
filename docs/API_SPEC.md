# ビンゴシステム API仕様書

## 基本情報
*   **Base URL**: `http://localhost:3000`
*   **Content-Type**: `application/json`

## エンドポイント一覧

### 1. 次の数字を引く (Next Number)

ビンゴの数字を1つ抽選し、結果とこれまでの履歴を返します。

*   **URL**: `/next`
*   **Method**: `GET`

#### レスポンス

**成功時 (数字がまだある場合)**
```json
{
  "number": 42,
  "history": [5, 12, 42],
  "message": "Success"
}
```

**終了時 (全ての数字が出尽くした場合)**
```json
{
  "number": null,
  "history": [5, 12, 42, ...],
  "message": "Game Over"
}
```

| フィールド | 型 | 説明 |
| :--- | :--- | :--- |
| `number` | `integer` \| `null` | 抽選された数字 (1-75)。数字がない場合は `null`。 |
| `history` | `array<integer>` | これまでに抽選された全ての数字のリスト（順序保持）。 |
| `message` | `string` | 状態を表すメッセージ。 |
| `seed` | `integer` | 現在の乱数生成に使用されているシード値。 |

---

### 2. ゲームリセット (Reset Game)

現在のゲーム状態を破棄し、新しいゲームを開始します。数字は再シャッフルされます。

*   **URL**: `/reset`
*   **Method**: `POST`

#### レスポンス

```json
{
  "number": null,
  "history": [],
  "message": "Game Reset",
  "seed": 123456789
}
```

| フィールド | 型 | 説明 |
| :--- | :--- | :--- |
| `number` | `null` | 常に `null`。 |
| `history` | `array` | 常に空配列 `[]`。 |
| `message` | `string` | "Game Reset" |
| `seed` | `integer` | 現在の乱数生成に使用されているシード値。 |

## エラーハンドリング
サーバー内部エラーが発生した場合、標準的なHTTPステータスコード `500 Internal Server Error` が返される可能性があります。クライアント側では通信エラーとしてハンドリングすることを推奨します。

## CORS設定
開発環境向けに、全てのオリジン (`*`) からの `GET`, `POST` リクエストを許可しています。

---

### 3. あみだくじ設定 (Set Amida)

あみだくじの参加者（ゲスト）名を設定します。

*   **URL**: `/amida`
*   **Method**: `POST`
*   **Body**:
    ```json
    {
      "items": ["Guest A", "Guest B", ...]
    }
    ```

#### レスポンス

```json
{
  "items": ["Guest A", "Guest B", ...],
  "message": "Updated"
}
```

| フィールド | 型 | 説明 |
| :--- | :--- | :--- |
| `items` | `array<string>` | 更新後の参加者名リスト。 |
| `message` | `string` | "Updated" |

---

### 4. あみだくじ結果取得 (Get Amida Result)

あみだくじの結果（参加者と景品番号のペア）を取得します。

*   **URL**: `/amida/result`
*   **Method**: `GET`

#### レスポンス

```json
{
  "items": [
    ["Guest A", "3"],
    ["Guest B", "1"],
    ...
  ],
  "message": "Success"
}
```

| フィールド | 型 | 説明 |
| :--- | :--- | :--- |
| `items` | `array<array<string>>` | `[参加者名, 景品番号]` のペアのリスト。 |
| `message` | `string` | "Success" |

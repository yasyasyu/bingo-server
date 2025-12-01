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

### 3. あみだくじデータ取得 (Get Amida)

現在のあみだくじの設定（景品名）を取得します。

*   **URL**: `/amida`
*   **Method**: `GET`

#### レスポンス

```json
{
  "items": ["Prize A", "Prize B", ...],
  "message": "Success"
}
```

| フィールド | 型 | 説明 |
| :--- | :--- | :--- |
| `items` | `array<string>` | 設定された10個の景品名のリスト。未設定の場合は空文字が含まれる。 |
| `message` | `string` | "Success" |

---

### 4. あみだくじ設定 (Setup Amida)

あみだくじの景品名を設定します。

*   **URL**: `/amida/setup`
*   **Method**: `POST`
*   **Body**:
    ```json
    {
      "items": ["Prize A", "Prize B", ...]
    }
    ```

#### レスポンス

```json
{
  "items": ["Prize A", "Prize B", ...],
  "message": "Updated"
}
```

| フィールド | 型 | 説明 |
| :--- | :--- | :--- |
| `items` | `array<string>` | 更新後の景品名リスト。 |
| `message` | `string` | "Updated" |

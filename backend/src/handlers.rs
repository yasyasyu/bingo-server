use crate::state::AppState;
use axum::{Json, extract::State};
use serde::{Deserialize, Serialize};

/// ビンゴの抽選結果レスポンス
#[derive(Serialize)]
pub struct NumberResponse {
    /// 抽選された数字 (Noneの場合は終了)
    pub number: Option<u8>,
    /// これまでの抽選履歴
    pub history: Vec<u8>,
    /// ステータスメッセージ
    pub message: String,
    /// 使用されているシード値
    pub seed: u32,
}

/// あみだくじ設定リクエスト
#[derive(Serialize, Deserialize)]
pub struct AmidaRequest {
    /// 参加者名のリスト
    pub items: Vec<String>,
}

/// あみだくじ設定レスポンス
#[derive(Serialize)]
pub struct AmidaResponse {
    /// 現在の参加者名リスト
    pub items: Vec<String>,
    /// ステータスメッセージ
    pub message: String,
    /// 使用されているシード値
    pub seed: u32,
}

/// あみだくじ結果レスポンス
#[derive(Serialize)]
pub struct AmidaResultResponse {
    /// (参加者名, 景品番号) のペアリスト
    pub items: Vec<(String, String)>,
    /// ステータスメッセージ
    pub message: String,
    /// 使用されているシード値
    pub seed: u32,
}

/// 次のビンゴ番号を抽選する
///
/// # Arguments
/// * `state` - アプリケーション状態
///
/// # Returns
/// * `Json<NumberResponse>` - 抽選結果
pub async fn get_next_number(State(state): State<AppState>) -> Json<NumberResponse> {
    let mut game = state.game.lock().unwrap();

    if let Some(num) = game.get_next_number() {
        Json(NumberResponse {
            number: Some(num),
            history: game.history.clone(),
            message: "Success".to_string(),
            seed: state.seed,
        })
    } else {
        Json(NumberResponse {
            number: None,
            history: game.history.clone(),
            message: "Game Over".to_string(),
            seed: state.seed,
        })
    }
}

/// ゲームをリセットする
///
/// # Arguments
/// * `state` - アプリケーション状態
///
/// # Returns
/// * `Json<NumberResponse>` - リセット後の状態
pub async fn reset_game(State(state): State<AppState>) -> Json<NumberResponse> {
    let mut game = state.game.lock().unwrap();
    game.reset();

    Json(NumberResponse {
        number: None,
        history: Vec::new(),
        message: "Game Reset".to_string(),
        seed: state.seed,
    })
}

/// あみだくじの設定を取得する
///
/// 現在設定されている参加者名リストを返します。
///
/// # Arguments
/// * `state` - アプリケーション状態
///
/// # Returns
/// * `Json<AmidaResponse>` - 現在の設定
pub async fn get_amida(State(state): State<AppState>) -> Json<AmidaResponse> {
    let amida = state.amida.lock().unwrap();
    Json(AmidaResponse {
        items: amida.gests.clone(),
        message: "Success".to_string(),
        seed: state.seed,
    })
}

/// あみだくじの設定を更新する
///
/// クライアントから送信された参加者名リストでサーバーの状態を更新します。
///
/// # Arguments
/// * `state` - アプリケーション状態
/// * `payload` - 更新する参加者リスト
///
/// # Returns
/// * `Json<AmidaResponse>` - 更新後の設定
pub async fn set_amida(
    State(state): State<AppState>,
    Json(payload): Json<AmidaRequest>,
) -> Json<AmidaResponse> {
    let mut amida = state.amida.lock().unwrap();
    amida.update(payload.items);
    Json(AmidaResponse {
        items: amida.gests.clone(),
        message: "Updated".to_string(),
        seed: state.seed,
    })
}

/// あみだくじの結果を取得する
///
/// サーバー側で決定された「参加者」と「景品番号」のペアを返します。
/// あみだくじの線の形はクライアント側でランダムに生成されますが、
/// 最終的な結果はこのAPIのレスポンスに従います。
///
/// # Arguments
/// * `state` - アプリケーション状態
///
/// # Returns
/// * `Json<AmidaResultResponse>` - 抽選結果ペア
pub async fn get_amida_result(State(state): State<AppState>) -> Json<AmidaResultResponse> {
    let amida = state.amida.lock().unwrap();
    let result = amida.get_result();
    // ゲストと景品の組み合わせを返す
    Json(AmidaResultResponse {
        items: result.unwrap_or_default(),
        message: "Success".to_string(),
        seed: state.seed,
    })
}

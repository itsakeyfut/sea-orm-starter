use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use sea_orm::*;
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use std::sync::Arc;
use dotenvy::dotenv;

mod entities;
use entities::user;

// アプリケーションの状態
#[derive(Clone)]
struct AppState {
    db: DatabaseConnection,
}

// 新しいユーザーを追加するリクエストのための構造体
#[derive(Deserialize)]
struct CreateUserRequest {
    username: String,
    cognito_id: String
}

// JSONレスポンス用の構造体
#[derive(Serialize)]
struct UserResponse {
    user_id: i32,
    username: String,
    cognito_id: String
}

// ユーザー作成のハンドラー
async fn create_user(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<UserResponse>, StatusCode> {
    // 挿入するデータを ActiveModel に設定
    let new_user = user::ActiveModel {
        username: Set(payload.username.clone()),
        cognito_id: Set(payload.cognito_id.clone()),
        ..Default::default()
    };

    // データベースに挿入
    let result = user::Entity::insert(new_user)
        .exec(&state.db)
        .await
        .map_err(|err| {
            eprintln!("Database insertion failed: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(UserResponse {
        user_id: result.last_insert_id,
        username: payload.username,
        cognito_id: payload.cognito_id
    }))
}

// 特定ユーザー取得のハンドラー
async fn get_user(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<i32>
) -> Result<Json<UserResponse>, StatusCode> {
    let user = user::Entity::find_by_id(user_id)
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some(user) = user {
        Ok(Json(UserResponse {
            user_id: user.user_id,
            username: user.username,
            cognito_id: user.cognito_id
        }))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

#[tokio::main]
async fn main() -> Result<(), DbErr> {
    dotenv().ok();

    // データベース接続
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = Database::connect(&db_url).await?;

    // アプリケーション状態の設定
    let state = Arc::new(AppState { db });

    // ルーターの設定
    let app = Router::new()
        .route("/users", post(create_user))
        .route("/users/:id", get(get_user))
        .with_state(state);

    // サーバーのリスナーを作成
    let listener = match TcpListener::bind("0.0.0.0:8000").await {
        Ok(listener) => listener,
        Err(e) => {
            eprintln!("Server Connection Error: {:?}", e);
            return Err(DbErr::Custom("Server connection failed".to_string()));
        }
    };
    
    // サーバー起動
    axum::serve(listener, app).await.unwrap_or_else(|e| {
        eprintln!("Server failed to start: {:?}", e);
        std::process::exit(1);
    });

    Ok(())
}
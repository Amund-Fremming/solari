use axum::{routing::post, Json, Router};
use serde_json::Value;

use crate::models::AppState;

pub fn router() -> Router<AppState> {
	Router::new().route("/pay", post(pay))
}

async fn pay() -> Json<Value> {
	todo!("apple pay pay handler is not implemented yet")
}

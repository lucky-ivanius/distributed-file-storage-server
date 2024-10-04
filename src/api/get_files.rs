use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;
use sqlx::PgPool;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::error::Result;

#[derive(Serialize, ToSchema)]
pub struct FileInfo {
    file_id: Uuid,
    total_chunks: i64,
}

#[utoipa::path(
    get,
    path = "/files",
    responses(
        (status = 200, description = "List of uploaded files", body = Vec<FileInfo>),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_files(State(pool): State<PgPool>) -> Result<impl IntoResponse> {
    let files = sqlx::query!(
        r#"
    SELECT file_id, COUNT(*) as total_chunks
    FROM file_chunks
    GROUP BY file_id
    "#
    )
    .fetch_all(&pool)
    .await?;

    let file_infos: Vec<FileInfo> = files
        .into_iter()
        .map(|row| FileInfo {
            file_id: row.file_id,
            total_chunks: row.total_chunks.unwrap_or(0), // Use 0 if COUNT(*) returns NULL
        })
        .collect();

    Ok((StatusCode::OK, Json(file_infos)))
}

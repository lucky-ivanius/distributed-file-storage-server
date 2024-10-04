use axum::body::Bytes;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;
use sqlx::PgPool;
use tokio::task;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::error::Error;

const CHUNK_SIZE: usize = 1024 * 1024; // 1MB

#[derive(Serialize, ToSchema)]
pub struct UploadResponse {
    file_id: Uuid,
}

#[utoipa::path(
    post,
    path = "/upload",
    request_body(content = [u8], description = "The file to upload", content_type = "application/octet-stream"),
    responses(
        (status = 201, description = "File uploaded successfully", body = UploadResponse),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn upload_file(
    State(pool): State<PgPool>,
    body: Bytes,
) -> Result<impl IntoResponse, Error> {
    let file_id = Uuid::new_v4();

    let file_extension = infer::get(&body)
        .map(|t| t.extension().to_string())
        .unwrap_or_else(|| "file".to_string());

    let mut tasks = Vec::new();

    for (chunk_index, chunk) in body.chunks(CHUNK_SIZE).enumerate() {
        let pool = pool.clone();
        let file_id = file_id;
        let chunk = chunk.to_vec();
        let file_extension = file_extension.clone();

        let task = task::spawn(async move {
            sqlx::query!(
                "INSERT INTO file_chunks (file_id, chunk_index, chunk_data, file_extension) VALUES ($1, $2, $3, $4)",
                file_id,
                chunk_index as i32,
                chunk,
                file_extension
            )
            .execute(&pool)
            .await
        });

        tasks.push(task);
    }

    for task in tasks {
        task.await.map_err(|e| Error::TaskJoin(e.to_string()))??;
    }

    Ok((StatusCode::CREATED, Json(UploadResponse { file_id })))
}

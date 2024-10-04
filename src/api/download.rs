use axum::{
    body::{Body, Bytes},
    extract::{Path, State},
    http::{header, StatusCode},
    response::{IntoResponse, Response},
};
use sqlx::PgPool;
use tokio::task;
use uuid::Uuid;

use crate::error::Error;

#[utoipa::path(
    get,
    path = "/download/{file_id}",
    params(
        ("file_id" = String, Path, description = "The UUID of the file to download")
    ),
    responses(
        (status = 200, description = "File downloaded successfully", content_type = "application/octet-stream"),
        (status = 404, description = "File not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn download_file(
    State(pool): State<PgPool>,
    Path(file_id): Path<Uuid>,
) -> Result<impl IntoResponse, Error> {
    let file_info = sqlx::query!(
        r#"
        SELECT file_extension, COUNT(*) as chunk_count
        FROM file_chunks
        WHERE file_id = $1
        GROUP BY file_extension
        "#,
        file_id
    )
    .fetch_optional(&pool)
    .await?;

    let (file_extension, chunk_count) = match file_info {
        Some(info) => (info.file_extension, info.chunk_count.unwrap_or(0) as usize),
        None => return Err(Error::NotFound("File not found".to_string())),
    };

    let mut tasks = Vec::with_capacity(chunk_count);
    for chunk_index in 0..chunk_count {
        let pool = pool.clone();
        let file_id = file_id;
        let task = task::spawn(async move {
            sqlx::query!(
                "SELECT chunk_data FROM file_chunks WHERE file_id = $1 AND chunk_index = $2",
                file_id,
                chunk_index as i32
            )
            .fetch_one(&pool)
            .await
        });
        tasks.push(task);
    }

    let mut file_content = Vec::with_capacity(chunk_count);
    for task in tasks {
        let chunk = task.await.map_err(|e| Error::TaskJoin(e.to_string()))??;
        file_content.push(chunk.chunk_data);
    }

    // Combine all chunks into a single Bytes object
    let combined_content = file_content.into_iter().flatten().collect::<Vec<u8>>();
    let file_bytes = Bytes::from(combined_content);

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/octet-stream")
        .header(
            header::CONTENT_DISPOSITION,
            format!("attachment; filename=\"{}.{}\"", file_id, file_extension),
        )
        .body(Body::from(file_bytes))
        .unwrap())
}

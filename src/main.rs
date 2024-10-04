use axum::{
    extract::DefaultBodyLimit,
    routing::{get, post},
    Router,
};
use dotenv::dotenv;
use error::Result;
use sqlx::PgPool;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod api;
mod error;

const MAX_UPLOAD_SIZE: usize = 100 * 1024 * 1024; // 100MB

#[derive(OpenApi)]
#[openapi(
    paths(
        api::upload::upload_file,
        api::get_files::get_files,
        api::download::download_file
    ),
    components(
        schemas(
            api::upload::UploadResponse,
            api::get_files::FileInfo,
        )
    ),
    tags(
        (name = "file", description = "File management API")
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to create connection pool");

    let app = Router::new()
        .route("/upload", post(api::upload::upload_file))
        .route("/files", get(api::get_files::get_files))
        .route("/download/:file_id", get(api::download::download_file))
        .merge(SwaggerUi::new("/docs").url("/docs/openapi.json", ApiDoc::openapi()))
        .layer(DefaultBodyLimit::max(MAX_UPLOAD_SIZE))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

use std::{env, path::PathBuf, sync::Arc};

use axum::{extract::{Path, State}, http::{Request, header}, response::IntoResponse};
use tower::ServiceExt;
use tower_http::services::ServeFile;

use crate::domain::images::{ImageHandlerError, ImageService};



#[axum::debug_handler]
pub async fn get_image_by_id(
    State(image_service): State<Arc<dyn ImageService>>,
    Path(id): Path<String>
) -> Result<impl IntoResponse, ImageHandlerError>
{
    let image = image_service.get_image_by_id(&id).await?;

    //경로 조합
    let image_path = env::var("IMAGE_DIRECTORY").expect("IMAGE_DIRECTORY is not set in .env file");
    let image_path = PathBuf::from(image_path);
    let rel_path = PathBuf::from(&image.relative_path);
    let full_path = image_path.join(rel_path);

    //파일 서빙
    let file = ServeFile::new(full_path);
    let req = Request::builder()
        .uri("/")
        .body(axum::body::Body::empty())
        .unwrap();

    let mut res = file.oneshot(req).await.map_err(|_| ImageHandlerError::TowerError)?;

    res.headers_mut().insert(header::CACHE_CONTROL, header::HeaderValue::from_static("public, max-age=31536000, immutable"));

    Ok(res)
}
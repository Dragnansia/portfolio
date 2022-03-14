use crate::AppState;
use actix_web::{
    get,
    web::{Data, Json, Path},
    Responder,
};
use futures::TryStreamExt;
use mongodb::bson::doc;
use portfolio::project::Project;

#[get("/api/projects")]
pub async fn all_project(app: Data<AppState>) -> actix_web::Result<impl Responder> {
    let projects = match app
        .db
        .collection::<Project>("projects")
        .find(None, None)
        .await
    {
        Ok(p) => p.try_collect::<Vec<_>>().await.unwrap_or_default(),
        Err(err) => return Err(actix_web::error::ErrorNotFound(err.to_string())),
    };

    Ok(Json(projects))
}

#[get("/api/project/{id}")]
pub async fn project_info(
    app: Data<AppState>,
    path: Path<(i32,)>,
) -> actix_web::Result<impl Responder> {
    let id = path.into_inner().0;

    let project = match app
        .db
        .collection::<Project>("projects")
        .find_one(doc! { "id": id }, None)
        .await
    {
        Ok(p) => p,
        Err(err) => return Err(actix_web::error::ErrorNotFound(err.to_string())),
    };

    if let Some(p) = project {
        Ok(Json(p))
    } else {
        Err(actix_web::error::ErrorNotFound("No found project"))
    }
}

use crate::AppState;
use actix_web::{
    get,
    web::{Data, Json},
    Responder,
};
use futures::TryStreamExt;
use portfolio::project::Project;

#[get("/projects")]
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

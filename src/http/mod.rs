use crate::config::Config;
use anyhow::Context;
use axum::{Router, routing::get, extract::State, Json, response::Html};
use serde::{Serialize, Deserialize};
use sqlx::PgPool;
use uuid::Uuid;

mod error;
mod question;

pub use error::{Error, ResultExt};

pub type Result<T, E = Error> = std::result::Result<T, E>;

use tower_http::trace::TraceLayer;

#[derive(Clone, Debug)]
pub(crate) struct AppContext {
    db: PgPool,
}

pub async fn serve(config: Config, db: PgPool) -> anyhow::Result<()> {

    let ctx = AppContext { db };

    let app = router(ctx);

    axum::Server::bind(&config.bind_address)
        .serve(app.into_make_service())
        .await
        .context("error running HTTP server")
}

fn router(ctx: AppContext) -> Router {
    Router::new()
        .route("/", get(index))
        .route("/healthz", get(health_check))
        .route("/users", get(list_users))
        .route(
            "/question/:id",
            get(question::question_detail)
            .post(question::submit_answer)
        )
        .layer(TraceLayer::new_for_http())
        .with_state(ctx)
}

async fn health_check() -> &'static str {
    "OK"
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
struct User {
    id: Uuid,
    identifier: String,
}

async fn list_users(state: State<AppContext>) -> Result<Json<Vec<User>>> {
    let users: Vec<User> = sqlx::query_as!(User, "SELECT id, identifier FROM student;")
        .fetch_all(&state.db)
        .await
        .unwrap();

    Ok(Json(users))
}

async fn index() -> Html<&'static str> {
    Html(include_str!("../../static/index.html"))
}

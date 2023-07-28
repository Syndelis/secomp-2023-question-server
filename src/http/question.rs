use askama::Template;
use axum::{response::{IntoResponse, Response, Html}, http::StatusCode, extract::{self, State}, Form};

use super::AppContext;

pub use crate::http::error::{Error, ResultExt};
pub type Result<T, E = Error> = std::result::Result<T, E>;


pub(crate) async fn question_detail(
    extract::Path(id): extract::Path<String>,
    state: State<AppContext>
) -> Result<Response> {

    let question = sqlx::query_as!(
        Question,
        r#"
        SELECT
            q.id,
            q.title,
            q.body
        FROM question AS q
        WHERE q.id = $1
        "#,
        id
    )
    .fetch_one(&state.db)
    .await?;

    let options = sqlx::query_as!(
        QuestionOption,
        r#"
        SELECT
            o.id,
            o.body
        FROM option AS o
        WHERE o.question_id = $1
        "#,
        id
    )
    .fetch_all(&state.db)
    .await?;

    let template = QuestionTemplate { question, options };
    Ok(HtmlTemplate(template).into_response())

}

#[axum::debug_handler]
pub(crate) async fn submit_answer(
    extract::Path(id): extract::Path<String>,
    state: State<AppContext>,
    Form(payload): Form<AnswerPayload>,
) -> Result<Response> {

    // let student_id = sqlx::query_scalar!(
    //     r#"
    //     "#
    // );

    // sqlx::query!(
    //     r#"
    //     INSERT INTO answer (student_identifier, option_id)
    //     "#
    // );

    println!("{payload:#?}");

    Ok("OK".into_response())

}

#[derive(serde::Deserialize, Debug)]
pub(crate) struct AnswerPayload {
    student_identifier: String,
    option_id: String,
}

#[derive(Template)]
#[template(path = "question.html")]
struct QuestionTemplate {
    question: Question,
    options: Vec<QuestionOption>,
}

#[derive(sqlx::FromRow)]
struct Question {
    id: String,
    title: String,
    body: String,
}

#[derive(sqlx::FromRow, sqlx::Type)]
struct QuestionOption {
    id: String,
    body: String,
}

struct HtmlTemplate<T>(T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", err),
            )
                .into_response(),
        }
    }
}

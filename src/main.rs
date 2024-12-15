mod image;
mod judger;
mod template;
mod testcase;

use crate::image::extract_text_from_image;
use crate::judger::run_code;
use crate::template::Template;
use crate::testcase::get_input_output;
use axum::extract::Multipart;
use axum::http::StatusCode;
use axum::response::Html;
use axum::{routing, Router};
use bytes::Bytes;
use prost::Message;

pub async fn homepage() -> Html<&'static str> {
    Html(include_str!("../html/index.html"))
}

struct Submission {
    image: Bytes,
    problem: String,
}

impl Submission {
    async fn try_from(mut value: Multipart) -> Result<Self, StatusCode> {
        let mut image = None;
        let mut problem = None;
        while let Some(field) = value.next_field().await.unwrap() {
            match field.name() {
                Some("file") => {
                    image = Some(field.bytes().await.map_err(|_| StatusCode::BAD_REQUEST)?)
                }
                Some("problem") => {
                    problem = Some(field.text().await.map_err(|_| StatusCode::BAD_REQUEST)?)
                }
                _ => continue,
            };
        }

        Ok(Self {
            image: image.ok_or(StatusCode::BAD_REQUEST)?,
            problem: problem.ok_or(StatusCode::BAD_REQUEST)?,
        })
    }
}

pub async fn upload(multipart: Multipart) -> Result<Html<String>, StatusCode> {
    let submission = Submission::try_from(multipart).await?;
    let text = extract_text_from_image(submission.image)
        .await
        .map_err(|err| {
            println!("Failed to extract text from image: {:?}", err);
            StatusCode::BAD_REQUEST
        })?;
    let (input, output) = get_input_output(&*submission.problem);
    let io = (input.as_bytes().to_vec(), output.as_bytes().to_vec());
    let code = run_code(text.clone().encode_to_vec(), io).await;

    Ok(Html(Template::new(text, code).render()))
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new()
        .route("/", routing::get(homepage))
        .route("/upload", routing::post(upload));

    println!("Listening at http://0.0.0.0:3000");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    axum::serve(listener, app).await?;

    Ok(())
}

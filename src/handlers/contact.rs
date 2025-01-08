use axum::{
    response::Html,
    extract::State,
};
use std::sync::Arc;
use tera::Tera;
use crate::error::AppError;

pub async fn contact(State(tera): State<Arc<Tera>>) -> Result<Html<String>, AppError> {
    let context = tera::Context::new();
    let html = tera.render("contact.html", &context)?;
    Ok(Html(html))
}
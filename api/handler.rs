use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    // Handle your routes here
    match req.uri().path() {
        "/" => {
            Ok(Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "text/html")
                .body(include_str!("../templates/home.html").into())?)
        }
        _ => {
            Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body("Not Found".into())?)
        }
    }
}

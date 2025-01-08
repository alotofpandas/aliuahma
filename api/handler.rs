use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};
use tera::{Tera, Context};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    let tera = Tera::new("templates/**/*")?;
    let context = Context::new();

    match req.uri().path() {
        "/" => {
            let rendered = tera.render("home.html", &context)?;
            Ok(Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "text/html")
                .body(rendered.into())?)
        }
        _ => {
            Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body("Not Found".into())?)
        }
    }
}

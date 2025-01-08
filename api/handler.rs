use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};
use tera::{Tera, Context};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    let tera = match Tera::new("templates/**/*") {
        Ok(t) => t,
        Err(_) => return Ok(Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body("Template error".into())?)
    };
    
    let context = Context::new();
    
    match req.uri().path() {
        "/" => {
            match tera.render("home.html", &context) {
                Ok(rendered) => Ok(Response::builder()
                    .status(StatusCode::OK)
                    .header("Content-Type", "text/html")
                    .body(rendered.into())?),
                Err(_) => Ok(Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body("Rendering error".into())?)
            }
        },
        _ => Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body("Not Found".into())?)
    }
}

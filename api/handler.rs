use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};
use tera::{Tera, Context};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(_req: Request) -> Result<Response<Body>, Error> {
    let mut tera = Tera::new("templates/**/*").map_err(|e| Error::from(e.to_string()))?;
    let context = Context::new();
    
    let rendered = tera.render("home.html", &context)
        .map_err(|e| Error::from(e.to_string()))?;
    
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/html")
        .body(rendered.into())?)
}

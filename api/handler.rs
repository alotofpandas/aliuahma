use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};
use tera::Tera;
use lazy_static::lazy_static;

lazy_static! {
    static ref TEMPLATES: Tera = {
        let tera = match Tera::new("templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera
    };
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    let context = tera::Context::new();
    
    match req.uri().path() {
        "/" => {
            let rendered = TEMPLATES.render("home.html", &context)
                .map_err(|_| Error::from("Template rendering failed"))?;
            
            Ok(Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "text/html")
                .body(rendered.into())?)
        },
        _ => Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body("Not Found".into())?)
    }
}

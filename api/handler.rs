use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};
use tera::{Tera, Context};
use std::sync::OnceLock;

static TEMPLATES: OnceLock<Tera> = OnceLock::new();

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    let tera = TEMPLATES.get_or_init(|| {
        let mut tera = Tera::default();
        tera.add_raw_templates(vec![
            ("base.html", include_str!("../templates/base.html")),
            ("home.html", include_str!("../templates/home.html")),
            ("contact.html", include_str!("../templates/contact.html")),
        ]).unwrap();
        tera
    });
    
    let context = Context::new();
    
    // Get the template name based on the path
    let template_name = match req.uri().path() {
        "/" => "home.html",
        "/contact" => "contact.html",
        _ => return Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body("404 Not Found".into())?),
    };
    
    let rendered = tera.render(template_name, &context)
        .map_err(|e| Error::from(e.to_string()))?;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/html")
        .body(rendered.into())?)
}

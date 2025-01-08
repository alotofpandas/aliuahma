use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};
use tera::{Tera, Context};
use std::sync::OnceLock;
use std::collections::HashMap;
use tokio::sync::RwLock;

static TEMPLATES: OnceLock<Tera> = OnceLock::new();
static CACHE: OnceLock<RwLock<HashMap<String, String>>> = OnceLock::new();

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

    let cache = CACHE.get_or_init(|| RwLock::new(HashMap::new()));
    let path = req.uri().path().to_string();

    {
        let cache_read = cache.read().await;
        if let Some(cached) = cache_read.get(&path) {
            return Ok(Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "text/html")
                .body(cached.clone().into())?);
        }
    }

    let context = Context::new();
    let template_name = match path.as_str() {
        "/" => "home.html",
        "/contact" => "contact.html",
        _ => return Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body("404 Not Found".into())?),
    };

    let rendered = tera.render(template_name, &context)
        .map_err(|e| Error::from(e.to_string()))?;

    {
        let mut cache_write = cache.write().await;
        cache_write.insert(path, rendered.clone());
    }

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/html")
        .body(rendered.into())?)
}

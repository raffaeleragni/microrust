mod sample;
mod statics;

use askama::Template;
use axum::{routing::get, Router};
use std::env;

pub fn init(app: Router) -> Router {
    statics::init(sample::init(app.route("/", get(index))))
}

#[derive(Template)]
#[template(path = "index.html")]
struct Index {
    cdn_css_file: String,
}

async fn index() -> Index {
    let css = env::var("CDN_CSS_FILE").unwrap_or("styles.css".into());
    Index { cdn_css_file: css }
}

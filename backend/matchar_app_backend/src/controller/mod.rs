use axum::routing::get;

pub fn routes(router: axum::Router) -> axum::Router {
    router.route("/", get(handler))
}

async fn handler() -> &'static str {
    "Hello, World!"
}

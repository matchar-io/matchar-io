mod auth;

pub fn routes(router: axum::Router) -> axum::Router {
    let router = auth::routes(router);

    router
}

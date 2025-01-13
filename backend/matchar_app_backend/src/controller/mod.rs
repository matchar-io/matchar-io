mod auth;
mod me;

pub fn routes(router: axum::Router) -> axum::Router {
    let router = auth::routes(router);
    let router = me::routes(router);

    router
}

mod auth;
mod me;
mod user;
mod ws;

pub fn routes(router: axum::Router) -> axum::Router {
    let router = auth::routes(router);
    let router = me::routes(router);
    let router = user::routes(router);
    let router = ws::routes(router);

    router
}

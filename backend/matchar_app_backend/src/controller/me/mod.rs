mod information;

use axum::routing::get;

pub fn routes(router: axum::Router) -> axum::Router {
    let router = router.route("/me", get(information::handler));

    router
}

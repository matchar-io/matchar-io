mod connect;

use axum::routing::any;

pub fn routes(router: axum::Router) -> axum::Router {
    router.route("/ws", any(connect::handler))
}

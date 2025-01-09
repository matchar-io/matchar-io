mod google_authorize;
mod google_callback;

use axum::routing::post;

pub fn routes(router: axum::Router) -> axum::Router {
    let router = router.route(
        "/api/auth/google/authorize",
        post(google_authorize::handler),
    );
    let router = router.route("/api/auth/google/callback", post(google_callback::handler));

    router
}

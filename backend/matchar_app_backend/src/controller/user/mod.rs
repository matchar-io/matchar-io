mod find_one;

use axum::routing::get;

pub fn routes(router: axum::Router) -> axum::Router {
    let router = router.route("/api/users/{user_id}", get(find_one::handler));

    router
}

use axum::Extension;
use postbox::PostOffice;

pub fn attach(axum: axum::Router) -> axum::Router {
    let office = PostOffice::new();
    let axum = axum.layer(Extension(office));

    axum
}

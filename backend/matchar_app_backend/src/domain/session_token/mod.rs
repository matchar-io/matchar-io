pub mod generated;
pub mod received;

pub use generated::*;
pub use received::*;

const KEY: &'static str = "matchar::session_token";
const EXPIRING_DAYS: time::Duration = time::Duration::days(30);

use database::DatabaseDriver;
use matchar_app_service::auth::google_callback::Repository;

pub struct Adapter<D: DatabaseDriver> {
    driver: D,
}

impl<D> Adapter<D>
where
    D: DatabaseDriver,
{
    pub const fn new(driver: D) -> Self {
        Self { driver }
    }
}

impl<D> Repository for Adapter<D>
where
    D: DatabaseDriver,
{
    //
}

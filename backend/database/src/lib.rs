#[allow(async_fn_in_trait)]
pub trait DatabaseDriver {
    type Connection;

    type Transaction;

    type Error: std::error::Error + Send + Sync;

    async fn connect(&self) -> Result<Self::Connection, Self::Error>;

    async fn begin(&self) -> Result<Self::Transaction, Self::Error>;

    async fn commit(&self, transaction: Self::Transaction) -> Result<(), Self::Error>;

    async fn rollback(&self, transaction: Self::Transaction) -> Result<(), Self::Error>;
}

#[derive(Clone)]
pub struct ConnectionPool(sqlx::PgPool);

impl ConnectionPool {
    pub async fn new(url: &str) -> Result<Self, sqlx::Error> {
        let pool = sqlx::PgPool::connect(url).await?;

        Ok(Self(pool))
    }
}

impl std::ops::Deref for ConnectionPool {
    type Target = sqlx::PgPool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DatabaseDriver for ConnectionPool {
    type Connection = sqlx::pool::PoolConnection<sqlx::Postgres>;

    type Transaction = sqlx::PgTransaction<'static>;

    type Error = sqlx::Error;

    #[inline]
    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        self.0.acquire().await
    }

    #[inline]
    async fn begin(&self) -> Result<Self::Transaction, Self::Error> {
        self.0.begin().await
    }

    #[inline]
    async fn commit(&self, transaction: Self::Transaction) -> Result<(), Self::Error> {
        transaction.commit().await
    }

    #[inline]
    async fn rollback(&self, transaction: Self::Transaction) -> Result<(), Self::Error> {
        transaction.rollback().await
    }
}

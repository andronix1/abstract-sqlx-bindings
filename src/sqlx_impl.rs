use std::sync::Arc;

use async_trait::async_trait;
use sqlx::{PgConnection, PgPool, Postgres, Transaction};

use abstract_db::{Db, DbTransaction};

pub struct SqlxTransaction<'a>(Transaction<'a, Postgres>);

impl SqlxTransaction<'_> {
    pub fn as_conn(&mut self) -> &mut PgConnection {
        &mut self.0
    }
}

#[async_trait]
impl DbTransaction for SqlxTransaction<'_> {    
    async fn commit(self) -> anyhow::Result<()> {
        self.0.commit().await?;
        Ok(())
    }

    async fn rollback(self) -> anyhow::Result<()> {
        self.0.rollback().await?;
        Ok(())
    }
}

pub struct SqlxDb(pub Arc<PgPool>);

#[async_trait]
impl Db for SqlxDb {
    type Transaction<'a> = SqlxTransaction<'a>;

    async fn begin<'a>(&self) -> anyhow::Result<Self::Transaction<'a>> {
        Ok(SqlxTransaction(self.0.begin().await?))
    }
}

impl SqlxDb {
    pub async fn connect(conn_str: &str) -> anyhow::Result<Self> {
        Ok(SqlxDb(Arc::new(PgPool::connect(conn_str).await?)))
    }
}
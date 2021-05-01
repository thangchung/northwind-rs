use async_once::AsyncOnce;
use color_eyre::Result;
use eyre::ErrReport;
use lazy_static::lazy_static;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use northwind_actix::config::Config;
use northwind_actix::run;

const DB_POOL_MAX_CONNECTIONS: u32 = 5;

lazy_static! {
    static ref DB_POOL: AsyncOnce<Result<PgPool>> =
        AsyncOnce::new(async { create_pool(&Config::from_env()?.database_url.as_str()).await });
}

pub async fn create_pool(db_uri: &str) -> Result<PgPool> {
    PgPoolOptions::new()
        .max_connections(DB_POOL_MAX_CONNECTIONS)
        .connect(&db_uri)
        .await
        .map_err(|e| -> ErrReport { e.into() })
}

pub async fn get_db_pool() -> &'static PgPool {
    DB_POOL.get().await.clone().as_ref().unwrap()
}

#[actix_web::main]
async fn main() -> Result<()> {
    // Load configuration
    // ------------------
    let settings = Config::from_env()?;

    // Install Color Eyre
    // ------------------
    color_eyre::install()?;

    // Runs migrations
    // ---------------
    if settings.database_auto_migration {
        sqlx::migrate!("./../../migrations").run(get_db_pool().await).await?;
    }

    run(settings, get_db_pool().await.to_owned()).await
}

use color_eyre::Result;
use eyre::ErrReport;
use northwind_actix::config::Config;
use northwind_actix::run;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

const DB_POOL_MAX_CONNECTIONS: u32 = 5;

async fn configure_with_db_url(db_uri: &str) -> Result<PgPool> {
    PgPoolOptions::new()
        .max_connections(DB_POOL_MAX_CONNECTIONS)
        .connect_timeout(std::time::Duration::from_secs(10))
        // .connect_lazy(&db_uri)
        .connect(&db_uri)
        .await
        .map_err(|e| -> ErrReport { e.into() })
}

#[actix_web::main]
async fn main() -> Result<()> {
    // Load configuration
    // ------------------
    let settings = Config::from_env()?;

    // Install Color Eyre
    // ------------------
    color_eyre::install()?;

    // Initialization Postgres Pool
    // ----------------------------
    let db_pool = configure_with_db_url(&settings.database_url).await?;

    // Runs migrations
    // ---------------
    if settings.database_auto_migration {
        sqlx::migrate!("./../../migrations").run(&db_pool).await?;
    }

    run(settings, db_pool).await
}

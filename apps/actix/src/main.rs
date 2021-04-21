use color_eyre::Result;
use northwind_actix::config::Config;
use northwind_actix::run;
use sqlx::PgPool;

#[actix_web::main]
async fn main() -> Result<()> {
    // Load configuration
    // ------------------
    let settings = Config::from_env()?;
    let db_url = &settings.database_url;

    // Install Color Eyre
    // ------------------
    color_eyre::install()?;

    // Initialization Postgres Pool
    // ----------------------------
    let db_pool = PgPool::connect(db_url).await?;

    // Runs migrations
    // ---------------
    if settings.database_auto_migration {
        sqlx::migrate!("./../../migrations").run(&db_pool).await?;
    }

    run(settings, db_pool).await
}

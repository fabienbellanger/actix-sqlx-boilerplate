use actix_sqlx_boilerplate::config::Config;
use actix_sqlx_boilerplate::run;
use color_eyre::Result;
use sqlx::mysql::MySqlPoolOptions;
use std::time::Duration;

#[actix_web::main]
async fn main() -> Result<()> {
    // Load configuration
    // ------------------
    let settings = Config::from_env()?;
    let db_url = &settings.database_url;

    // Install Color Eyre
    // ------------------
    color_eyre::install()?;

    // Initialisation du pool MySQL
    // ----------------------------
    // TODO: Put parameters in .env file.
    let db_pool = MySqlPoolOptions::new()
        .max_connections(100)
        .max_lifetime(Some(Duration::from_secs(30)))
        .connect_timeout(Duration::from_secs(30))
        .idle_timeout(Duration::from_secs(30))
        .min_connections(10)
        .test_before_acquire(true)
        .connect(db_url)
        .await?;

    // Runs migrations
    // ---------------
    // TODO: Ne fonctionne pas bien, essayer lors du passage en 0.5
    // if settings.database_auto_migration {
    //     sqlx::migrate!("./migrations").run(&db_pool).await?;
    // }

    run(settings, db_pool).await
}

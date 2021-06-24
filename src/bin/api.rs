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
    let db_max_connections = &settings.database_max_connections;
    let db_min_connections = &settings.database_min_connections;
    let db_max_lifetime = &settings.database_max_lifetime;
    let db_connect_timeout = &settings.database_connect_timeout;
    let db_idle_timeout = &settings.database_idle_timeout;

    // Install Color Eyre
    // ------------------
    color_eyre::install()?;

    // Initialisation du pool MySQL
    // ----------------------------
    let db_pool = MySqlPoolOptions::new()
        .max_connections(*db_max_connections)
        .min_connections(*db_min_connections)
        .max_lifetime(Some(Duration::from_secs(*db_max_lifetime)))
        .connect_timeout(Duration::from_secs(*db_connect_timeout))
        .idle_timeout(Duration::from_secs(*db_idle_timeout))
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

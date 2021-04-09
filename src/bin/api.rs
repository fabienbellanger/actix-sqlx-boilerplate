use actix_sqlx_boilerplate::config::Config;
use actix_sqlx_boilerplate::run;
use color_eyre::Result;
use sqlx::MySqlPool;

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
    let db_pool = MySqlPool::connect(db_url).await?;

    // Runs migrations
    // ---------------
    // TODO: Ne fonctionne pas bien, essayer lors du passage en 0.5
    // if settings.database_auto_migration {
    //     sqlx::migrate!("./migrations").run(&db_pool).await?;
    // }

    run(settings, db_pool).await
}

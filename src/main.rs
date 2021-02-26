use actix_sqlx_boilerplate::run;
use color_eyre::Result;

#[actix_web::main]
async fn main() -> Result<()> {
    run().await
}

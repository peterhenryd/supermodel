use dotenvy::dotenv;
use supermodel::dialect::Dialect;
use supermodel::model::utils::ModelUtils;
use supermodel::operation::utils::Utils;
use supermodel_macros::model;
use supermodel_sqlx::postgres::Postgres;

#[model(Postgres)]
#[name = "users"]
pub struct User {
    username: String,
    password: String
}

#[tokio::main]
async fn main() -> Result<(), <Postgres as Dialect>::Error> {
    dotenv().expect("initializing .env file");

    let url = std::env::var("DATABASE_URL").expect("missing DATABASE_URL in env");
    let mut connection = Postgres::create_connection(&url).await?;

    User::register().execute(&mut connection).await?;

    Ok(())
}
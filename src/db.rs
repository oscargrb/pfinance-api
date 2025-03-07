use dotenv::dotenv;
use std::env;
use tokio_postgres::{Client, Error, NoTls};

pub struct DbClient(pub Client);

pub async fn conection_db() -> Result<Client, Error> {
    dotenv().ok();

    let host = env::var("HOST").expect("se esperaba host");
    let postgres_user = env::var("POSTGRES_USER").expect("se esperaba POSTGRES_USER");
    let postgres_pwd = env::var("POSTGRES_PASSWORD").expect("se esperaba POSTGRES_PASSWORD");
    let postgres_db = env::var("POSTGRES_DB").expect("se esperaba POSTGRES_DB");

    let (client, connection) = tokio_postgres::connect(
        format!(
            "host={} user={} password={} dbname={}",
            host, postgres_user, postgres_pwd, postgres_db
        )
        .as_str(),
        NoTls,
    )
    .await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Error de Coneccion: {}", e);
        }
    });

    Ok(client)
}

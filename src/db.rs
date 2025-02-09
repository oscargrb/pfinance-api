use dotenv::dotenv;
use std::env;
use tokio_postgres::{Client, Error, NoTls};

pub struct DbClient(pub Client);

pub async fn conection_db() -> Result<Client, Error> {
    dotenv().ok();

    let (client, connection) = tokio_postgres::connect(
        format!(
            "host=${} user=${} password=${} dbname=${}",
            env::var("HOST").expect("err"),
            env::var("USER").expect("err"),
            env::var("PWD").expect("err"),
            env::var("DATABASE").expect("err"),
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

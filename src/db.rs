use tokio_postgres::{NoTls, Client, Error};

pub struct DbClient(pub Client);

pub async fn conection_db() -> Result<Client, Error> {
    let (client, connection) = tokio_postgres::connect(
        "host=localhost user=postgres password=123 dbname=finanzas_QA",
        NoTls
    ).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Error de Coneccion: {}", e);
        }
    });

    Ok(client)
}
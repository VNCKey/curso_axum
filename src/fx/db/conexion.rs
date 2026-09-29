use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    SqlitePool,
};

pub async fn conectar() -> Result<SqlitePool, sqlx::Error> {
    let conexion = SqliteConnectOptions::new()
        .filename("usuarios.db")
        .create_if_missing(true);

    SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(conexion)
        .await
}

pub async fn ejecutar_migraciones(pool: &SqlitePool) -> Result<(), sqlx::migrate::MigrateError> {
    sqlx::migrate!("./migraciones").run(pool).await
}

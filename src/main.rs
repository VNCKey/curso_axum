use api::http::route::router;
use fx::db::conexion::{conectar, ejecutar_migraciones};
use state::AppState;
use tokio::net::TcpListener;
mod api;
mod domain;
mod flows;
mod fx;
mod state;

#[tokio::main]
async fn main() {
    let pool = conectar()
        .await
        .expect("Fallo al conectar con la base de datos");
    ejecutar_migraciones(&pool)
        .await
        .expect("Fallo al ejecutar migraciones");

    let state = AppState { pool };
    let app = router(state);

    let listener = TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Fallo al enlazar el listener");

    println!("Servidor escuchando en http://127.0.0.1:3000");
    axum::serve(listener, app)
        .await
        .expect("Fallo al servir la aplicación");
}

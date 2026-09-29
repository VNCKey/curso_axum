async fn saludo() -> &'static str {
    "Hola Bienevido al curso de Axum"
}

async fn saludo_path(Path(path): Path<String>) -> String {
    format!("Hola {} bienvenido", path)
}

#[derive(Debug, Deserialize)]
pub struct Parametros {
    nombre: String,
}

async fn saludo_query(Query(parametros): Query<Parametros>) -> String {
    format!("Hola {} bienvenido", parametros.nombre)
}

#[derive(Debug, Serialize)]
pub struct SaludoResponse {
    pub nombre: String,
}

async fn saludar_json() -> Json<SaludoResponse> {
    Json(SaludoResponse {
        nombre: String::from("FerrisTelemetry"),
    })
}

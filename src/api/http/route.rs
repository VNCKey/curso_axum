use crate::api::http::usuarios::{
    crear_usuario, editar_usuario, eliminar_usuario, listar_usuarios, obtener_usuario,
};
use crate::state::AppState;
use axum::routing::{get, post};
use axum::Router;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/crear-usuario", post(crear_usuario))
        .route("/usuarios", get(listar_usuarios))
        .route(
            "/usuarios/{id}",
            get(obtener_usuario)
                .delete(eliminar_usuario)
                .put(editar_usuario),
        )
        .with_state(state)
}

use crate::{
    api::http::error::ApiError,
    flows::usuarios::{
        crear_usuario as crear_usuario_flow, editar_usuario as editar_usuario_flow,
        eliminar_usuario as eliminar_usuario_flow, listar_usuarios as listar_usuarios_flow,
        obtener_usuario as obtener_usuario_flow, CrearUsuarioCommand, EditarUsuarioCommand,
        UsuarioFlowError,
    },
    state::AppState,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CrearUsuarioRequest {
    pub nombre: String,
    pub edad: u8,
}

#[derive(Debug, Serialize)]
pub struct UsuarioResponse {
    pub nombre: String,
    pub edad: u8,
}

#[derive(Debug, Serialize)]
pub struct UsuarioDetalleResponse {
    pub id: i64,
    pub nombre: String,
    pub edad: u8,
}

#[derive(Debug, Deserialize)]
pub struct EditarUsuarioRequest {
    pub nombre: String,
    pub edad: u8,
}

pub async fn crear_usuario(
    State(state): State<AppState>,
    Json(request): Json<CrearUsuarioRequest>,
) -> Result<(StatusCode, Json<UsuarioResponse>), ApiError> {
    let command = CrearUsuarioCommand {
        nombre: request.nombre,
        edad: request.edad,
    };

    let usuario = crear_usuario_flow(command, &state.pool)
        .await
        .map_err(convertir_usuario)?;

    let usuario_response = Json(UsuarioResponse {
        nombre: usuario.nombre().to_string(),
        edad: usuario.edad(),
    });
    Ok((StatusCode::CREATED, usuario_response))
}

fn convertir_usuario(error: UsuarioFlowError) -> ApiError {
    match error {
        UsuarioFlowError::DatosInvalidos(_) => ApiError::DatosInvalidos,
        UsuarioFlowError::UsuarioInvalido(_) => ApiError::DatosInvalidos,
        UsuarioFlowError::BaseDeDatos(e) => ApiError::BaseDeDatos(e),
        UsuarioFlowError::NoEncontrado => ApiError::NoEncontrado,
    }
}

pub async fn listar_usuarios(
    State(state): State<AppState>,
) -> Result<(StatusCode, Json<Vec<UsuarioDetalleResponse>>), ApiError> {
    let usuarios = listar_usuarios_flow(&state.pool)
        .await
        .map_err(convertir_usuario)?;

    let usuarios_response = Json(
        usuarios
            .into_iter()
            .map(|usuario| UsuarioDetalleResponse {
                id: usuario.id,
                nombre: usuario.nombre,
                edad: u8::try_from(usuario.edad).unwrap_or_default(),
            })
            .collect::<Vec<_>>(),
    );
    Ok((StatusCode::OK, usuarios_response))
}

pub async fn obtener_usuario(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<(StatusCode, Json<UsuarioResponse>), ApiError> {
    let usuario = obtener_usuario_flow(&state.pool, id)
        .await
        .map_err(convertir_usuario)?
        .ok_or(ApiError::NoEncontrado)?;

    let usuario_response = Json(UsuarioResponse {
        nombre: usuario.nombre,
        edad: u8::try_from(usuario.edad).unwrap_or_default(),
    });
    Ok((StatusCode::OK, usuario_response))
}

pub async fn eliminar_usuario(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<StatusCode, ApiError> {
    eliminar_usuario_flow(&state.pool, id)
        .await
        .map_err(convertir_usuario)?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn editar_usuario(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(usuario): Json<EditarUsuarioRequest>,
) -> Result<StatusCode, ApiError> {
    let command = EditarUsuarioCommand {
        nombre: usuario.nombre,
        edad: usuario.edad,
    };

    editar_usuario_flow(&state.pool, id, command)
        .await
        .map_err(convertir_usuario)?;

    Ok(StatusCode::NO_CONTENT)
}

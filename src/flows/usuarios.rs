use crate::domain::{Usuario, UsuarioError};
use crate::fx::db::usuarios::{
    editar as editar_db, eliminar as eliminar_db, insertar as insertar_db, listar as listar_db,
    obtener as obtener_db, UsuarioRow,
};
use sqlx::SqlitePool;
use thiserror::Error;
use validator::{Validate, ValidationErrors};

#[derive(Debug, Error)]
pub enum UsuarioFlowError {
    #[error("Datos inválidos")]
    DatosInvalidos(ValidationErrors),
    #[error("Usuario inválido")]
    UsuarioInvalido(UsuarioError),
    #[error("Base de datos")]
    BaseDeDatos(#[source] sqlx::Error),
    #[error("No encontrado")]
    NoEncontrado,
}

#[derive(Debug, Clone, Validate)]
pub struct CrearUsuarioCommand {
    #[validate(length(min = 5, max = 100))]
    pub nombre: String,
    #[validate(range(min = 18, max = 120))]
    pub edad: u8,
}

pub async fn crear_usuario(
    command: CrearUsuarioCommand,
    pool: &SqlitePool,
) -> Result<Usuario, UsuarioFlowError> {
    command
        .validate()
        .map_err(UsuarioFlowError::DatosInvalidos)?;

    let usuario =
        Usuario::nuevo(command.nombre, command.edad).map_err(UsuarioFlowError::UsuarioInvalido)?;

    insertar_db(pool, &usuario)
        .await
        .map_err(UsuarioFlowError::BaseDeDatos)?;

    Ok(usuario)
}

pub async fn listar_usuarios(pool: &SqlitePool) -> Result<Vec<UsuarioRow>, UsuarioFlowError> {
    listar_db(pool).await.map_err(UsuarioFlowError::BaseDeDatos)
}

pub async fn obtener_usuario(
    pool: &SqlitePool,
    id: i64,
) -> Result<Option<UsuarioRow>, UsuarioFlowError> {
    obtener_db(pool, id)
        .await
        .map_err(UsuarioFlowError::BaseDeDatos)
}

pub async fn eliminar_usuario(pool: &SqlitePool, id: i64) -> Result<(), UsuarioFlowError> {
    let eliminado = eliminar_db(pool, id)
        .await
        .map_err(UsuarioFlowError::BaseDeDatos)?;

    if eliminado {
        Ok(())
    } else {
        Err(UsuarioFlowError::NoEncontrado)
    }
}

#[derive(Debug, Clone, Validate)]
pub struct EditarUsuarioCommand {
    #[validate(length(min = 5, max = 100))]
    pub nombre: String,
    #[validate(range(min = 18, max = 120))]
    pub edad: u8,
}

pub async fn editar_usuario(
    pool: &SqlitePool,
    id: i64,
    command: EditarUsuarioCommand,
) -> Result<(), UsuarioFlowError> {
    command
        .validate()
        .map_err(UsuarioFlowError::DatosInvalidos)?;

    let usuario = Usuario::nuevo(command.nombre.clone(), command.edad)
        .map_err(UsuarioFlowError::UsuarioInvalido)?;

    let editar = editar_db(pool, id, &usuario)
        .await
        .map_err(UsuarioFlowError::BaseDeDatos)?;

    if editar {
        Ok(())
    } else {
        Err(UsuarioFlowError::NoEncontrado)
    }
}

use crate::domain::Usuario;
use sqlx::SqlitePool;

pub async fn insertar(pool: &SqlitePool, usuario: &Usuario) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO usuarios(nombre, edad) VALUES (?, ?)")
        .bind(usuario.nombre())
        .bind(i64::from(usuario.edad()))
        .execute(pool)
        .await?;

    Ok(())
}

#[derive(Debug, sqlx::FromRow)]
pub struct UsuarioRow {
    pub id: i64,
    pub nombre: String,
    pub edad: i64,
}

pub async fn listar(pool: &SqlitePool) -> Result<Vec<UsuarioRow>, sqlx::Error> {
    let usuarios =
        sqlx::query_as::<_, UsuarioRow>("SELECT id, nombre, edad FROM usuarios ORDER BY id ASC")
            .fetch_all(pool)
            .await?;
    Ok(usuarios)
}

pub async fn obtener(pool: &SqlitePool, id: i64) -> Result<Option<UsuarioRow>, sqlx::Error> {
    sqlx::query_as::<_, UsuarioRow>("SELECT id, nombre, edad FROM usuarios WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn eliminar(pool: &SqlitePool, id: i64) -> Result<bool, sqlx::Error> {
    let resultado = sqlx::query("DELETE FROM usuarios WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(resultado.rows_affected() > 0)
}

pub async fn editar(pool: &SqlitePool, id: i64, usuario: &Usuario) -> Result<bool, sqlx::Error> {
    let resultado = sqlx::query("UPDATE usuarios SET nombre = ?, edad = ? WHERE id = ?")
        .bind(usuario.nombre())
        .bind(i64::from(usuario.edad()))
        .bind(id)
        .execute(pool)
        .await?;
    Ok(resultado.rows_affected() > 0)
}

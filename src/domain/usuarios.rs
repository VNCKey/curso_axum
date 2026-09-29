#[derive(Debug, Clone, PartialEq)]
pub enum UsuarioError {
    NombreInvalido,
    EdadInvalida,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Usuario {
    // id: i64,
    nombre: String,
    edad: u8,
}

impl Usuario {
    pub fn nombre(&self) -> &str {
        &self.nombre
    }

    pub fn edad(&self) -> u8 {
        self.edad
    }

    pub fn nuevo(nombre: String, edad: u8) -> Result<Usuario, UsuarioError> {
        if nombre.is_empty() {
            return Err(UsuarioError::NombreInvalido);
        }
        if edad == 0 {
            return Err(UsuarioError::EdadInvalida);
        }
        Ok(Usuario { nombre, edad })
    }
}

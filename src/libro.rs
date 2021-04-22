use crate::user::User;

#[derive(Debug, PartialEq, Eq)]
pub struct Libro {
    pub nombre: String,
    pub isbn: String,
    pub categoria: String,
    pub ano_publ: u32,
    pub prestado: bool,
    pub usuario_de_pres: Option<User>,
}
#[allow(dead_code)]
impl Libro {
    pub fn new(
        nombre: String,
        isbn: String,
        categoria: String,
        ano_publ: u32,
        prestado: bool,
        usuario_de_pres: Option<User>,
    ) -> Libro {
        Libro {
            nombre,
            isbn,
            categoria,
            ano_publ,
            prestado,
            usuario_de_pres,
        }
    }
}

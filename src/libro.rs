use crate::user::User;
use std::io;

#[derive(Debug, PartialEq, Eq)]
pub struct Libro {
    pub nombre: String,
    pub isbn: String,
    pub categoria: String,
    pub ano_publ: u32,
    pub prestado: bool,
    pub usuario_de_pres: Option<User>,
}
#[allow(dead_code, unused_variables)]
impl Libro {
    pub fn new() -> Libro {
        // entrada del nombre
        println!("Ingrese el nombre del libro");
        let mut nuevo_nombre = String::new();
        let entrada = io::stdin().read_line(&mut nuevo_nombre).unwrap();

        // entrada de isbn del libro
        println!("Ingrese el ISBN del libro");
        let mut nuevo_isbn = String::new();
        let entrada2 = io::stdin().read_line(&mut nuevo_isbn).unwrap();

        // entrada de cateogira del libro
        println!("Ingrese la categoria del libro");
        let mut nueva_categoria = String::new();
        let entrada2 = io::stdin().read_line(&mut nueva_categoria).unwrap();

        // entrada del año de publicacion
        println!("Ingrese el año de publicacion del libro");
        let mut nuevo_anio = String::new();
        let entrada2 = io::stdin().read_line(&mut nuevo_anio).unwrap();
        let anio_publicado: u32 = nuevo_anio.trim().parse().unwrap();

        // retorno del libro con los datos ingesados, usuario prestado siempre sera None
        // al igual que prestado siempre sera false, hasta que este sea solicitao por un usuario
        Libro {
            nombre: nuevo_nombre,
            isbn: nuevo_isbn,
            categoria: nueva_categoria,
            ano_publ: anio_publicado,
            prestado: false,
            usuario_de_pres: None,
        }
    }
}

use io::BufRead;

use crate::libro::Libro;
use std::io;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct User {
    pub nombre: String,
    pub mem_act: bool,
    pub top_de_libro: u8,
}

#[allow(unused_variables, dead_code)]
impl User {
    pub fn new() -> User {
        let entrada = io::stdin();
        println!("Ingrese su nombre");
        let nombre = entrada.lock().lines().next().unwrap().unwrap();
        println!("Recuerde que el tope de libros que puede prestar es: 2");
        User {
            nombre,
            mem_act: true,
            top_de_libro: 0,
        }
    }
    pub fn buscar_libro(lista: &mut Vec<Libro>, nombre_libro: String) {
        for val in lista.iter() {
            if val.nombre.lines().next().unwrap() == nombre_libro.lines().next().unwrap() {
                println!("El nombre del libro es: {}\nEl isbn es: {}\nLa categoria es: {}\nEl ano de publicacion es: {}", 
                val.nombre.lines().next().unwrap(), 
                val.isbn.lines().next().unwrap(), 
                val.categoria.lines().next().unwrap(), 
                val.ano_publ);
            } else {
                println!("No se ha encontrado ningun libro con ese nombre");
            }
        }
    }
    pub fn incrementar_libro(&mut self) {
        self.top_de_libro += 1;
    }
    pub fn decrementar_libro(&mut self) {
        self.top_de_libro -= 1;
    }
}

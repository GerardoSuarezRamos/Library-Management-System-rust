use crate::libro::Libro;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct User {
    pub nombre: String,
    pub mem_act: bool,
    pub top_de_libro: bool,
}

#[allow(unused_variables, dead_code)]
impl User {
    pub fn new(nombre: String, mem_act: bool) -> User {
        User {
            nombre,
            mem_act,
            top_de_libro: false,
        }
    }
    pub fn buscar_libro(prestamo: &mut Vec<Libro>, nombre_libro: String) {
        for val in prestamo.iter() {
            if val.nombre == nombre_libro {
                println!("{:?}", val);
            }
        }
    }
}

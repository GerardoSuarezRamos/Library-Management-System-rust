#[allow(unused_variables)]
mod servicios_de_libreria {
    use crate::{libro::Libro, user::User};

    pub fn _prestar_libro(usuario: User, lista: &Vec<Libro>, nombre: String) {
        /* Me pesa lo mismo si intento eliminar  un registor, el problema en si existe
        en el prestamo de la variable */
    }
    pub fn _insertar_nuevo_libro(libro: Libro, lista: &mut Vec<Libro>) {
        lista.push(libro);
    }
    pub fn _eliminar_libro(nombre: String, lista: &mut Vec<Libro>) {
        /* Necesito saber como puedo modificar los datos de una estructura desde una funcion,
        por otro lado necesito enteder mas sobre modulos */
    }
}

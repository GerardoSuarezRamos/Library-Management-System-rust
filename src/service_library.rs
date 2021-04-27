#[allow(unused_variables, dead_code)]
pub mod servicios_de_libreria {
    use crate::{libro::Libro, user::User};

    pub fn _prestar_libro(usuario: &mut User, lista: &mut Vec<Libro>, nombre: String) {
        if usuario.top_de_libro <= 2 {
            for val in lista.iter_mut() {
                if val.prestado != true {
                    if val.nombre == nombre {
                        val.prestado = true;
                        val.usuario_de_pres = Some(usuario.clone());
                        usuario.incrementar_libro();
                        println!("ha prestado el libro!");
                    }
                } else {
                    println!("El libro se encuentra prestado");
                }
            }
        }
    }
    pub fn devolucion_libro(mut usuario: User, nombre: String, lista: &mut Vec<Libro>) {
        for val in lista.iter_mut() {
            if val.nombre == nombre {
                usuario.decrementar_libro();
                val.prestado = false;
                val.usuario_de_pres = None;
                println!("Ha devuelto el libro");
            }
        }
    }
}

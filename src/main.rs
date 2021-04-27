/* es importante resaltar que al trabajar con modulos, siempre se utilizan las rutas relativas
al crate que estas utilizando como interno */

/* autor: Gerardo Suarez Ramos */

// imports
use std::io;
use user::User;
// modulos necesarios para correr el programa
mod libro;
mod service_library;
mod user;

// llamada a librerias

#[allow(unused_variables, dead_code)]
fn main() {
    // "Registros" llevado sobre vectores dinamicos
    let mut _lista_de_libros: Vec<libro::Libro> = Vec::new();
    let mut _usuarios: Vec<user::User> = Vec::new();
    loop {
        // Opciones de trabajo
        println!("========================================================================================================================");
        println!("\t\t\t\t\t\t\tBIENVENIDO A LA BIBLIOTECA");
        println!("========================================================================================================================");
        println!("\t¿Que desea hacer?");
        println!("\t1. Crear un nuevo usuario");
        println!("\t2. Buscar un libro");
        println!("\t3. Insertar un nuevo libro");
        println!("\t4. Prestar un libro");
        println!("\t5. Devolver un libro");
        println!("\t6. Salir");
        println!("> ");

        // Eleccion del usuario
        let mut nuevo = String::new();
        let entrada = io::stdin().read_line(&mut nuevo);
        let eleccion: u8 = nuevo.trim().parse().unwrap();

        if eleccion == 6 {
            break;
        }
        println!("cantidad de usuarios: {}", _usuarios.len());
        println!("cantidad de libros: {}", _lista_de_libros.len());
        match eleccion {
            1 => {
                let usuario: User = user::User::new();
                _usuarios.push(usuario);
            }
            2 => {
                println!("Ingrese el nombre del libro deseado");
                println!(">");
                let mut nombre = String::new();
                let entrada = io::stdin().read_line(&mut nombre);
                user::User::buscar_libro(&mut _lista_de_libros, nombre);
            }
            3 => {
                let nuevo: libro::Libro = libro::Libro::new();
                _lista_de_libros.push(nuevo);
            }
            4 => {
                // Recibir nombre de usuario para buscarlo en la lista
                println!("¿Cual es el nombre de usuario?");
                let mut nombre_de_usuario = String::new();
                let entrada = io::stdin().read_line(&mut nombre_de_usuario).unwrap();

                // Comprobacion de usuario para el prestamo
                for val in _usuarios.iter_mut() {
                    // Recibir nombre del libro para prestar
                    if val.nombre.lines().next().unwrap()
                        == nombre_de_usuario.lines().next().unwrap()
                    {
                        println!("¿Cual libro desea prestar?");
                        let mut nombre_de_libro = String::new();
                        let entrada2 = io::stdin().read_line(&mut nombre_de_libro).unwrap();

                        // Realizar procedimiento de prestamo
                        service_library::servicios_de_libreria::_prestar_libro(
                            val,
                            &mut _lista_de_libros,
                            nombre_de_libro,
                        )
                    }
                }
            }
            5 => {
                // Recibir nombre de usuario para buscarlo en la lista
                println!("¿Cual es el nombre de usuario?");
                let mut nombre_de_usuario = String::new();
                let entrada = io::stdin().read_line(&mut nombre_de_usuario).unwrap();

                for val in _usuarios.iter() {
                    if val.nombre.lines().next().unwrap()
                        == nombre_de_usuario.lines().next().unwrap()
                    {
                        // Recibir nombre del libro para prestar
                        println!("¿Cual libro desea devolver?");
                        let mut nombre_de_libro = String::new();
                        let entrada2 = io::stdin().read_line(&mut nombre_de_libro).unwrap();

                        // Realizar procedimiento de prestamo
                        service_library::servicios_de_libreria::devolucion_libro(
                            val.clone(),
                            nombre_de_libro,
                            &mut _lista_de_libros,
                        )
                    }
                }
            }
            _ => {
                println!("Eleccion equivocada")
            }
        }
    }
}

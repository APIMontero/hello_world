/*
    Explicación del código
        use std::env;: Importa el módulo env para acceder a variables de entorno y argumentos de línea de comandos.
        use std::path::Path;: Importa el módulo path para trabajar con rutas de archivos.
        let os = env::consts::OS;: Obtiene el sistema operativo anfitrión y lo almacena en la variable os.
        println!("Sistema operativo anfitrión: {}", os);: Imprime el sistema operativo anfitrión.
        let ruta_archivo = env::current_dir().unwrap();: Obtiene la ruta del directorio actual y la almacena en la variable ruta_archivo.
        println!("Ruta del archivo en disco: {}", ruta_archivo.display());: Imprime la ruta del directorio actual.
        let ruta_especifica = Path::new("/home/usuario/mi_archivo.txt");: Crea una ruta específica al archivo "/home/usuario/mi_archivo.txt".
        println!("Ruta específica del archivo: {}", ruta_especifica.display());: Imprime la ruta específica del archivo.

    Cómo ejecutar el código
        Guarda el código en un archivo llamado main.rs.
        Abre una terminal en el directorio donde guardaste el archivo.
        Compila el código con el comando rustc main.rs.
        Ejecuta el código con el comando ./main.
*/

use std::env;
use std::path::Path;

fn main() {
    // Obtener el sistema operativo anfitrión
    let os = env::consts::OS;
    println!("Sistema operativo anfitrión: {}", os);

    // Obtener la ruta del archivo en disco
    let ruta_archivo = env::current_dir().unwrap(); // Ruta del directorio actual
    println!("Ruta del archivo en disco: {}", ruta_archivo.display());

    // También puedes especificar una ruta específica
    let ruta_especifica = Path::new("/home/david/Informaciones.mpi");
    println!("Ruta específica del archivo: {}", ruta_especifica.display());
}
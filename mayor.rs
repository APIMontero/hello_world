use std::io;

// Función para leer un número desde la consola
fn leer_numero() -> i32 {
    loop {
        let mut input = String::new();
        println!("Introduce un número:");
        io::stdin().read_line(&mut input).expect("Error al leer la entrada");

        match input.trim().parse() {
            Ok(num) => return num, // Si la conversión es exitosa, retorna el número
            Err(_) => println!("Entrada inválida. Inténtalo de nuevo."), // Si no, pide reintentar
        }
    }
}

// Función para determinar el mayor de dos números
fn determinar_mayor(num1: i32, num2: i32) -> i32 {
    if num1 > num2 {
        num1
    } else {
        num2
    }
}

fn mayor_de_tres(num1: i32, num2: i32, num3: i32) -> i32{
    let mayor = determinar_mayor(num1, num2);
    let mayor3 = determinar_mayor(mayor, num3);

    mayor3
}

// Función principal del programa
fn main() {
    let num1 = leer_numero();
    let num2 = leer_numero();
    let num3 = leer_numero();

    let mayor = mayor_de_tres(num1, num2, num3);

    println!("El mayor de los tres números es: {}", mayor);
}

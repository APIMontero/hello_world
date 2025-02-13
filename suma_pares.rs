use std::io;

// Leer un numero desde teclado, cuidadndo excepciones numéricas
fn leer_numero() -> i32 {
    // Leer hasta que la entrada sea correcta
    loop {
        let mut input = String::new();
        println!("Introduce un número:");
        io::stdin().read_line(&mut input).expect("Error al leer la entrada");

        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("Entrada inválida. Inténtalo de nuevo."),
        }
    }
}

// Cálculo de la suma de números pares
fn calcular_suma_pares(vector: &Vec<i32>) -> i32 {
    let mut suma = 0;
    // Recorre los elementos de un veector determinando cual es par y realizando la suma.
    for &elemento in vector {
        if elemento % 2 == 0 {
            suma += elemento;
        }
    }
    suma
}

// Función principal
fn main() {
    let num = leer_numero();
    let mut numeros = Vec::new();
    for i in 1..=num {
        numeros.push(i);
    }

    let suma_pares = calcular_suma_pares(&numeros);
    println!("La suma de los números pares del 1 al {} es: {}", num, suma_pares);
}


use std::io;
use std::io::prelude::*;

fn leer_desde_consola() -> String {
    let mut input = String::new();
    println!("Por favor, introduce un valor:");
    io::stdin().read_line(&mut input).expect("Error al leer desde la consola");
    input.trim().to_string() // Eliminar espacios en blanco y convertir a String
}

fn imprimir_valor(valor: &str) {
    println("El valor introducido es: {}", valor);
}

fn main() {
    let valor = leer_desde_consola();
    imprimir_valor(&valor);
}
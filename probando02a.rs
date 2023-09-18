use std::io; // biblioteca io para manejar la entrada 


/*
Sentencias de control:
if sin () {bloque}
match (switch más amigable)
while
for para iterar colecciones al estilo python
loop bucle infinito (break)
*/


fn main() {
    // Solicitar la entrada del usuario
    println!("Por favor, ingrese su edad: ");

    // Crear una nueva cadena mutable (String) para almacenar la entrada del usuario.
    let mut input = String::new();

    // Leer la entrada del usuario y manejar errores
    io::stdin()
        .read_line(&mut input)
        .expect("Error al leer la entrada.");

    // Convertir la entrada a un número entero (u32).
    let age: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Entrada no válida. Debe ingresar un número entero.");
            return;
        }
    };

    // Tomar una decisión basada en la edad
    if age < 18 {
        println!("Eres menor de edad.");
    } else {
        println!("Eres mayor de edad.");
    }
}


/*

Operadores aritméticos:

    +: Suma.
    -: Resta.
    *: Multiplicación.
    /: División.
    %: Módulo (resto de la división).



Operadores aritméticos:

    +: Suma.
    -: Resta.
    *: Multiplicación.
    /: División.
    %: Módulo (resto de la división).


Operadores lógicos:

    &&: AND lógico.
    ||: OR lógico.
    !: NOT lógico (negación).


Operadores bit a bit:

    &: AND a nivel de bits.
    |: OR a nivel de bits.
    ^: XOR a nivel de bits.
    <<: Desplazamiento a la izquierda.
    >>: Desplazamiento a la derecha.



*/

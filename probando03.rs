
/*

Multiplos de 3 y 5 de 1 a 100


*/

fn main() {
    let mut number = 1;

    while number <= 100 {
        if number % 3 == 0 || number % 5 == 0 {
            println!("Número múltiplo de 3 o 5: {}", number);
        }
        number += 1;
    }
}


/*

Entrada/Salida (I/O):

    std::io::stdin(): Obtiene una instancia de entrada estándar.
    std::io::stdout(): Obtiene una instancia de salida estándar.
    println!(): Imprime texto en la consola con formato.
    eprintln!(): Imprime un mensaje de error en la consola con formato.
    format!(): Crea una cadena de texto formateada sin imprimirla.

Conversiones de Datos:

    to_string(): Convierte tipos en cadenas de texto.
    parse(): Convierte cadenas de texto en tipos.

Gestión de Vectores (Vec) y Arrays:

    Vec::new(): Crea un nuevo vector vacío.
    vec![]: Crea un vector con elementos iniciales.
    len(): Devuelve la longitud de un vector o una cadena.
    push(): Agrega un elemento al final de un vector.
    pop(): Elimina y devuelve el último elemento de un vector.
    get(): Obtiene un elemento de un vector por índice de manera segura.
    contains(): Verifica si un vector contiene un elemento.

Gestión de Cadenas (String):

    String::new(): Crea una nueva cadena vacía.
    to_string(): Convierte otros tipos en cadenas.
    len(): Devuelve la longitud de una cadena.
    push_str(): Agrega una cadena al final de otra.
    chars(): Obtiene un iterador de caracteres de una cadena.
    split(): Divide una cadena en partes utilizando un delimitador.

Operaciones Matemáticas:

    +, -, *, /: Operadores aritméticos para sumar, restar, multiplicar y dividir.
    %: Operador de módulo para obtener el resto de una división.
    sqrt(): Calcula la raíz cuadrada de un número.
    pow(): Calcula la potencia de un número.

Manejo de Errores:

    Result<T, E>: Tipo de resultado que indica éxito o error.
    Ok(): Constructor para un resultado exitoso.
    Err(): Constructor para un resultado de error.
    match: Estructura de control para manejar resultados.

Gestión de Archivos (std::fs):

    std::fs::File::open(): Abre un archivo para lectura.
    std::fs::File::create(): Crea un archivo nuevo.
    read_to_string(): Lee contenido de un archivo en una cadena.
    write(): Escribe datos en un archivo.

Concurrencia (std::thread):

    std::thread::spawn(): Crea un nuevo hilo.
    join(): Espera a que un hilo termine su ejecución.

*/


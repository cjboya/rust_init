//vectores

fn main() {
    // Declarar un array de números enteros con tamaño 4
    let numeros = [1, 2, 3, 4];
    // Acceder a elementos del array por índice (los índices comienzan en 0)
    let primer_numero = numeros[0];
    let tercer_numero = numeros[2];
    println!("Primer número: {}", primer_numero);
    println!("Tercer número: {}", tercer_numero);
    // Calcular la longitud del array
    let longitud = numeros.len();
    println!("Longitud del array: {}", longitud);

    let mut numeros: Vec<i32> = Vec::new();

    // Agregar elementos al vector
    numeros.push(1);
    numeros.push(2);
    numeros.push(3);

    // Acceder a elementos del vector por índice
    let primer_numero = numeros[0];
    let tercer_numero = numeros[2];

    println!("Primer número: {}", primer_numero);
    println!("Tercer número: {}", tercer_numero);

    // Calcular la longitud del vector
    let longitud = numeros.len();
    println!("Longitud del vector: {}", longitud);

    // Iterar sobre los elementos del vector
    for numero in &numeros {
        println!("Número: {}", numero);
    }

}


/*

Métodos de vectores en Rust junto con una breve descripción de lo que hacen:

    new(): Crea un nuevo vector vacío.

    with_capacity(capacity: usize): Crea un nuevo vector con una capacidad inicial especificada.

    push(item: T): Agrega un elemento al final del vector.

    pop(): Elimina y devuelve el último elemento del vector.

    len(): Devuelve la cantidad de elementos en el vector.

    is_empty(): Devuelve true si el vector está vacío, de lo contrario, false.

    capacity(): Devuelve la capacidad actual del vector.

    reserve(additional: usize): Aumenta la capacidad del vector para acomodar un número adicional de elementos sin necesidad de realocación.

    shrink_to_fit(): Reduce la capacidad del vector para que coincida con la cantidad de elementos actualmente almacenados.

    clear(): Elimina todos los elementos del vector.

    contains(&item: &T): Comprueba si el vector contiene un elemento específico.

    get(index: usize) -> Option<&T>: Obtiene una referencia al elemento en la posición especificada o devuelve None si el índice está fuera de rango.

    first() -> Option<&T>: Obtiene una referencia al primer elemento del vector o devuelve None si el vector está vacío.

    last() -> Option<&T>: Obtiene una referencia al último elemento del vector o devuelve None si el vector está vacío.

    insert(index: usize, item: T): Inserta un elemento en una posición específica del vector, desplazando los elementos existentes.

    remove(index: usize) -> T: Elimina y devuelve el elemento en la posición especificada del vector.

    swap(index1: usize, index2: usize): Intercambia dos elementos en el vector.

    sort(): Ordena los elementos del vector.

    iter() -> Iter<T>: Obtiene un iterador inmutable sobre los elementos del vector.

    iter_mut() -> IterMut<T>: Obtiene un iterador mutable sobre los elementos del vector.

    into_iter() -> IntoIter<T>: Convierte el vector en un iterador que consume los elementos.

*/
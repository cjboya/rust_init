// Definir una función llamada "saludar" que toma un parámetro "nombre" de tipo &str
fn saludar(nombre: &str) {
    println!("¡Hola, {}!", nombre);
}

fn main() {
    // Llamar a la función "saludar" con un argumento
    saludar("Juan");
    saludar("María");
}

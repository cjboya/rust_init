/* 
Tipos de datos escalares (primitivos):
i8, i16, i32, i64, i128: Números enteros con signo de diferentes tamaños.
u8, u16, u32, u64, u128: Números enteros sin signo de diferentes tamaños.
f32, f64: Números de punto flotante de precisión simple y doble.
char: Representa un carácter Unicode.
bool: Representa un valor booleano (true o false).

Tipos de datos compuestos:
tuple: Una colección ordenada de elementos de diferentes tipos. Por ejemplo, (1, "hola", true).
array: Una colección de elementos del mismo tipo con tamaño fijo conocido en tiempo de compilación. Por ejemplo, [1, 2, 3].
slice: Una vista de una parte de un array o vector.
str: Una cadena de texto inmutable codificada en UTF-8.
String: Una cadena de texto mutable alojada en el montón.
struct: Un tipo de datos personalizado que agrupa campos con diferentes tipos.
enum: Un tipo de datos personalizado que representa un conjunto discreto de valores posibles.
option: Un tipo que representa un valor opcional, que puede ser Some(valor) o None.

Tipos de datos de colecciones:
Vec<T>: Un vector dinámico que puede crecer o reducir su tamaño.
HashMap<K, V>: Una tabla hash que asigna claves a valores.
HashSet<T>: Un conjunto que almacena elementos únicos.
BTreeMap<K, V>: Un mapa ordenado basado en árboles.
BTreeSet<T>: Un conjunto ordenado basado en árboles.

Tipos de datos de puntero y referencia:
&T: Una referencia inmutable a un valor.
&mut T: Una referencia mutable a un valor.
*const T y *mut T: Punteros crudos a memoria.
Box<T>: Un puntero inteligente que posee un valor en el montón.
Rc<T>: Contador de referencias para datos compartidos (solo para tipos que implementan Clone).
Arc<T>: Contador de referencias atómico para datos compartidos (para concurrencia).
*/

fn main() {
    println!("¡Hola, mundo!");
}

//online compiler https://www.programiz.com/rust/online-compiler/


/*

Por defecto, las variables en Rust son inmutables. Esto significa que, 
una vez que asignas un valor a una variable, no puedes cambiar ese valor. 
Si intentas hacerlo, el compilador te dará un error.

Si deseas que una variable sea mutable y pueda cambiar su valor,
debes declararla explícitamente como mutable utilizando la palabra clave mut.


let mut y = 5; // y es mutable y tiene el valor 5
y = 10;

*/


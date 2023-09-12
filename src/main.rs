//// 2023/09/23 // 14:10 //

// Administración de la memoria

// Stack vs Heap (Pila vs Montón)

// Stack (Pila)
// La pila almacena valores en el orden en que los obtiene
// y elimina los valores en el order opuesto.
// Esto se conove como LIFO

// Heap (Montón)
// Los datos con un tamaño desconocido en el momento de la
// complicación o con un tamaño que podría cambiar deben almacenarse
// en el heap.
// EL montón está menos organizado

// Reglas de propiedad
// 1. Cada valor en Rust tiene una variable que se llama propietario.
// 2. Solo puede haber un propietario a la vez.
// 3. Cuando el propietario se sale del ámbito, el valor se eliminará.

// Ámbito de una variable
// Él ámbito de una variable es el rango dentro de un programa en el que
// esa variable es válida.

// Habitualmente, el ámbito es el espacio entre las llaves {}
// en que está comprendido.

fn main() {

    let i: i32 = 7;
    let j; i32 = i;

    println!("j: {}",j);
    println!("i: {}",i);

    let cadena = String::from("Primera cadena");
    let cadena2 = cadena1;

    println!("Cadena2: {}", cadena2);
    println!("Cadena1: {}", cadena1);

    

}











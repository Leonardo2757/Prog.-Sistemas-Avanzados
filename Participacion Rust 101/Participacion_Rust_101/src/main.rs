fn main() {
    //let x = 5;
    //x = 10;

    //let mut y = 10;
    //y = 20;

    let x = 5;
    {
        let x = x + 1; //Nueva variable
        println!("El valor de x es: {}", x);
    }
    println!("El valor de x es: {}", x);

    let entero: i32 = 42;
    let flotante: f64 = 3.1416;
    let boleano: bool = true;
    let caracter: char = 'a';
    let tupla: (i32, f64, char) = (42, 3.1416, 'a');
    let array: [i32; 3] = [1, 2, 3];

    println!("Tupla(tupla) forma1: {:?}", tupla);
    println!("Tupla(tupla) forma2: ({}, {}, {})", tupla.0, tupla.1, tupla.2);


}

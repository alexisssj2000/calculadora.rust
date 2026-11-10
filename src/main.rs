fn main() {
    println!("‡--------------------------------→");
    println!("Hola, mundo!");

    let nombre = "Alexis";
    let edad: u8 = 24;
    result();
    println!("Hola {}, tienes {} años", nombre, edad);
}
fn sumar(a: i32, b: i32) -> i32 {
    a + b
}

fn result() {
    let resultado = sumar(10, 20);
    println!("Resultado: {}", resultado);
}
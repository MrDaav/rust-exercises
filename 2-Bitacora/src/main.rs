use std::io;

fn main() {
    let mut balance: i32 = 0;
    let mut entrada = String::new();
    let mut valido = true;

    io::stdin().read_line(&mut entrada).expect("Fallo al leer");

    for caracter in entrada.trim().chars() {
        if caracter == '[' {
            balance += 1;
        } else if caracter == ']' {
            balance -= 1;
        }
        if balance < 0 {
            valido = false;
            break;
        }
    }

    if valido && balance == 0 {
        println!("CORRECTO");
    } else {
        print!("INCORRECTO");
    }
}

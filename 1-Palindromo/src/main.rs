use std::io;

fn es_palindromo(palabra: &str) -> bool {
    let clean: String = palabra.chars()
    .filter(|caracter| !caracter.is_whitespace())
    .map(|caracter| caracter.to_ascii_lowercase())
    .collect();

    clean.chars().eq(clean.chars().rev())
}

fn main() {
    let mut entrada = String::new();

    io::stdin().read_line(&mut entrada)
    .expect("No se pudo leer");
    println!("{}", if es_palindromo(&entrada){
        "SI"
    } else {
        "NO"
    });

}
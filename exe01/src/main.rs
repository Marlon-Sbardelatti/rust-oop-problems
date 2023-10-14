mod concorrente;
mod prova;
use concorrente::Concorrente;
use prova::Prova;
fn main() {
    let c1 = Concorrente::new(
        String::from("Marlon"),
        String::from("backend"),
        23,
        String::from("4002-8922"),
        String::from("Brusque - SC"),
    );

    let p1 = Prova::new(&c1, 9.3);
    println!("{}", p1.imprimir());
}

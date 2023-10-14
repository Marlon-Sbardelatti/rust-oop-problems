mod familia;
mod pessoa;
use familia::Familia;
use pessoa::Pessoa;

fn main() {
    let pessoa1 = Pessoa::new(String::from("Rickert"), 15);
    let pessoa2 = Pessoa::new(String::from("Guts"), 22);
    let pessoa3 = Pessoa::new(String::from("Caska"), 23);
    let pessoa4 = Pessoa::new(String::from("Judeau"), 18);

    let mut familia = Familia::new();

    familia.adicionar_membro(&pessoa1);
    familia.adicionar_membro(&pessoa2);
    familia.adicionar_membro(&pessoa3);
    familia.adicionar_membro(&pessoa4);

    // println!("{}", familia.mais_velho());
    //
    println!("{}", familia.imprimir());
}

mod setor_pessoal;
mod funcionario;
use funcionario::Funcionario;
use setor_pessoal::SetorPessoal;

fn main() {
    let mut setor = SetorPessoal::new();
    let funcionario1= Funcionario::new(1, String::from("Guts"), 23, 5000.00, String::from("Espadachim"));
    let funcionario2= Funcionario::new(2, String::from("Caska"), 24, 3000.00, String::from("Comandante"));

    // let copy = &funcionario1;
    // println!("{}", copy.to_string());
    setor.add_funcionario(funcionario1);
    setor.add_funcionario(funcionario2);
    setor.print_all();
    println!("{}", setor.imprimir_folha());

    println!("{}", setor.valor_total_folha());
    println!("{}", setor.maior_salario());
    println!("{}", setor.buscar_departamento(23));
    println!("{}", setor.buscar_funcao(String::from("ESPADACHIM")));
    println!("{}", setor.imprimir_folha_crescente());

    
}

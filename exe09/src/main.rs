mod multa;
mod proprietario;
mod veiculo;
use std::{collections::HashMap, io};

use multa::Multa;
use proprietario::Proprietario;
use veiculo::Veiculo;
fn main() {
    let m = Multa::new(
        String::from("Av Carlos Renaux"),
        String::from("Gravissima"),
        19000.0,
        true,
    );
    let p = Proprietario::new(
        String::from("Marlon"),
        String::from("081.136.189-61"),
        String::from("Brusque - SC"),
    );
    let mut v = Veiculo::new(String::from("10"), &p, 'P', 'G', 70000.0, 0.0);
    v.addMulta(&m);

    let mut map: HashMap<&str, &Veiculo> = HashMap::new();
    map.insert(v.get_placa(),&v);
    println!("Insira o numero da placa: ");
    let mut result = String::new();
    io::stdin()
        .read_line(&mut result)
        .expect("Failed to read line");

    println!("{}", buscar(result.as_str(), &map));
    // loop {
    //    let mut input = String::new();

    //    println!("1 - Buscar placa:\n2- Sair:\n");
    //    io::stdin().read_line(&mut input).expect("Failed to read line");

    //    match input.trim() {
    //        "1" => {
    //            println!("Insira o numero da placa: ");
    //            let mut result = String::new();
    //            io::stdin().read_line(&mut result).expect("Failed to read line");
    //            let busca = buscar(result.as_str(), &map);
    //            if busca == true{
    //                let value = map.get(&result.as_str());
    //                println!("{:?}", value);
    //            }else {
    //                println!("false")
    //            }
    //        }
    //       "2" => break,
    //       _ => println!("numero invalido")
    //    }
    // }
}

pub fn buscar(placa: &str, map: &HashMap<&str, &Veiculo>) -> bool {
    println!("{}", &placa);
    if map.contains_key(&placa) {
        return true;
    }
    return false;
}

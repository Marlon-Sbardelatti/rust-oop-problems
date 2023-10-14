// a) Crie vários objetos da classe Site (Nome, endereço IP e status de acesso)

// b) Crie um objeto tipo HashMap que armazenará objetos do tipo Site, onde a chave corresponde ao nome do site

// c) Dentro do main (DicionarioDeIP) crie um método chamado inserirSite que recebe como parâmetro um objeto da classe Site e o objeto HashMap.
// Dentro deste método insira o par [Nome do site] – [Site] no HashMap

// e) Dentro do main (DicionarioDeIP) faça um método que imprima na tela o Nome, endereço IP e status do site armazenadas no HashMap.
mod site;
use std::collections::HashMap;

use site::Site;
fn main() {
    let site1 = Site::new(
        String::from("Youtube.com"),
        String::from("192.127.50.55"),
        true,
    );
    let site2 = Site::new(
        String::from("Amazon.com"),
        String::from("192.127.80.55"),
        true,
    );
    let site3 = Site::new(
        String::from("Netflix.com"),
        String::from("192.127.75.40"),
        false,
    );
    let site4 = Site::new(
        String::from("Furb.com"),
        String::from("192.127.30.25"),
        false,
    );

    let mut sites: HashMap<String, &Site> = HashMap::new();
    inserir_site(&site1, &mut sites);
    inserir_site(&site2, &mut sites);
    inserir_site(&site3, &mut sites);
    inserir_site(&site4, &mut sites);
    // println!("{}", acessar_site(String::from("Youtube.com"), &sites));
    println!("{}", imprimir(&sites));
}

fn inserir_site<'a>(site: &'a Site, map: &mut HashMap<String, &'a Site>) {
    map.insert(site.get_nome().to_string(), site);
    println!("Site inserted {}", site.get_nome())
}

fn acessar_site<'a>(nome: String, map: &HashMap<String, &'a Site>) -> String {
    match map.get(&nome) {
        Some(site) => {
            if *site.get_status() {
                return site.to_string();
            } else {
                return String::from("O site esta bloqueado");
            }
        }
        None => {
            return String::from("Site não encontrado");
        }
    }
}
// d) Dentro do main (DicionarioDeIP) crie um método chamado acessarSite que recebe como parâmetro uma String com o [Nome do site] e o objeto HashMap.
// Se o status do site estiver definido como acesso livre, retorne o objeto Site, mostrando as informações [nome e endereço IP do site] ao usuário.
// Agora se o status do site estiver definido como bloqueado, retorne uma mensagem de erro,
// informando ao usuário sobre a indisponibilidade de uso do site (site bloqueado)

// e) Dentro do main (DicionarioDeIP) faça um método que imprima na tela o Nome, endereço IP e status do site armazenadas no HashMap.

fn imprimir(map: &HashMap<String, &Site>) -> String{
    let mut dados = String::from("Sites no hash:\n");
    for (nome, site) in  map{
        dados.push_str(&format!("{}\n", site.to_string()));
    }

    dados
}

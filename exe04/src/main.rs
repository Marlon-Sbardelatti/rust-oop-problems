mod revisor;
mod artigo;
use std::collections::HashMap;

use revisor::Revisor;
use artigo::Artigo;
fn main() {
    let mut artigos: HashMap<String, &Artigo> = HashMap::new();

    let mut artigo1 = Artigo::new(String::from("Berserk"), String::from("Miura"));

    let revisor1 = Revisor::new(String::from("Marlon"), String::from("Obra prima"), String::from("Aprovado"));
    let revisor2 = Revisor::new(String::from("Matheus"), String::from("Muito bom"), String::from("Aprovado"));
    let revisor3 = Revisor::new(String::from("Victor"), String::from("Excelente"), String::from("Aprovado"));

    artigo1.adicionar_revisor(&revisor1);
    artigo1.adicionar_revisor(&revisor2);
    artigo1.adicionar_revisor(&revisor3);

    artigos.insert(artigo1.get_nome().to_string(), &artigo1);

    let mut artigo2 = Artigo::new(String::from("Berserk"), String::from("Miura"));
    artigo2.adicionar_revisor(&revisor1);
    artigo2.adicionar_revisor(&revisor2);
    artigo2.adicionar_revisor(&revisor3);

    artigos.insert(artigo2.get_nome().to_string(), &artigo2);
    imprimir_map(&artigos);
}

fn imprimir_map(map: &HashMap<String, &Artigo>) {
   for (nome, artigo) in map {
      println!("{}", artigo.to_string());
   } 
}

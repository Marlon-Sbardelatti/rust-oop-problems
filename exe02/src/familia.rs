use crate::pessoa::Pessoa;
pub struct Familia<'a> {
    familiares: Vec<&'a Pessoa>,
}

impl<'a> Familia<'a> {
    // pub fn new(familiares: Vec<&'a Pessoa>) -> Self {
    //     Self { familiares }
    // }

    pub fn new() -> Familia<'a> {
        Familia {
            familiares: Vec::new(),
        }
    }

    pub fn adicionar_membro(&mut self, pessoa: &'a Pessoa) {
        self.familiares.push(pessoa);
        println!("{} adicionado a família.", pessoa.get_nome());
    }

    pub fn mais_velho(&self) -> String {
        let mut maior = 0;
        for p in &self.familiares {
            if p.get_idade() > &maior {
                maior = *p.get_idade();
            }
        }

        let mut dados = String::from("A(s) pessoa(s) mais velha é:\n");
        for p in &self.familiares {
            if p.get_idade() == &maior {
                dados.push_str(&format!("Nome: {}\n", p.get_nome()));
            }
        }
        dados
    }

    pub fn imprimir(&self) -> String {
        let mut dados = String::from("Membros da família:\n");

        for p in &self.familiares {
            dados.push_str(&format!("Nome: {}\n", p.get_nome()));
        }
        dados
    }
}

// a) Faça um método para adicionar membros à família

// b) Faça um método que retorne a pessoa mais velha da família

// c) Faça um método que imprime o nome dos membros da família

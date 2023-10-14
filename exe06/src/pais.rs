// 6. Escreva uma classe que represente um “País”. Um país tem como atributos o seu nome e a sua dimensão em
// Km2 e uma lista de países com os quais ele faz fronteira.
// Represente a classe e forneça os seus membros a seguir:

use std::cell::RefCell;
use std::rc::Rc;

pub struct Pais {
    nome: String,
    dimensao: f64,
    capital: String,
    vizinhos: Vec<Rc<RefCell<Pais>>>,
}

impl Pais {
    pub fn new(nome: String, capital: String, dimensao: f64) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            nome,
            dimensao,
            vizinhos: Vec::new(),
            capital,
        }))
    }
    pub fn adicionar_vizinho(&mut self, pais: Rc<RefCell<Pais>>) {
        self.vizinhos.push(pais);
    }
    pub fn equals(&self, outro: &Pais) -> bool {
        self.nome == outro.nome && self.capital == outro.capital
    }

    pub fn faz_fronteira(&self, outro: &Pais) -> bool {
        self.vizinhos
            .iter()
            .any(|vizinho| vizinho.borrow().equals(outro))
    }

    pub fn lista_vizinhos(&self, outro: &Pais) -> Vec<String> {
        let mut vizinhos_comuns = Vec::new();
        for pais1 in &self.vizinhos {
            for pais2 in &pais1.borrow().vizinhos {
                if pais2.borrow().equals(outro) {
                    vizinhos_comuns.push(pais1.borrow().nome.clone());
                }
            }
        }
        vizinhos_comuns
    }
    // pub struct Pais<'a> {
    //     pub nome: String,
    //     pub capital: String,
    //     pub dimensao: f64,
    //     vizinhos: Vec<&'a Pais<'a>>,
    // }

    // impl<'a> Pais<'a> {
    //     pub fn new(nome: String, capital: String, dimensao: f64) -> Self {
    //         Self {
    //             nome,
    //             dimensao,
    //             vizinhos: Vec::new(),
    //             capital,
    //         }
    //     }
    // pub fn get_nome(&self) -> &String {
    //    &self.nome
    // }
    // pub fn get_capital(&self) -> &String {
    //    &self.capital
    // }
    // pub fn get_dimensao(&self) -> &f64{
    //    &self.dimensao
    // }

    // pub fn adicionar_vizinho(&mut self, pais: &'a Pais) {
    //     self.vizinhos.push(pais);
    // }

    // // a) Um método que permita verificar se dois países são iguais. Dois países são iguais se tiverem o mesmo nome e a mesma capital.
    // // A assinatura deste método deve ser:
    // // − public boolean equals(Pais outro)

    // pub fn equals(&self, outro: &Pais) -> bool {
    //     self.nome == outro.nome && self.capital == outro.capital
    // }

    // // b) Um método que informe se um outro país é seu limítrofe (faz fronteira)

    // pub fn faz_fronteira(&self, outro: &Pais) -> bool {
    //     let mut found = false;
    //     for pais in &self.vizinhos {
    //         if pais.nome == outro.nome {
    //             found = true;
    //         }
    //     }
    //     found
    // }

    // // c) Um método que receba um outro país como parâmetro e retorne uma lista de vizinhos comuns aos dois países. Obs.:
    // // Considere que um país tem no máximo 40 outros países com os quais faz fronteira.

    // pub fn lista_vizinhos(&self, outro: &Pais) -> Vec<String> {
    //     let mut vizinhos_comuns = Vec::new();
    //     for pais1 in &self.vizinhos {
    //         for pais2 in &outro.vizinhos {
    //             if pais1.equals(pais2) {
    //                 vizinhos_comuns.push(pais1.nome.clone());
    //             }
    //         }
    //     }
    //     vizinhos_comuns
    // }
}

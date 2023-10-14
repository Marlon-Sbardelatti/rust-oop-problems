use crate::concorrente::Concorrente;

pub struct Prova<'a> {
    concorrente: &'a Concorrente,
    nota: f64,
}

impl<'a> Prova<'a> {
    pub fn new(concorrente: &'a Concorrente, nota: f64) -> Self {
        Self { concorrente, nota }
    }

    pub fn imprimir(&self) -> String {
       format!("{}", self.concorrente.to_string()) 
    }

    // pub fn lala(&self) -> String {
    //    String::from("lala") 
    // }
}


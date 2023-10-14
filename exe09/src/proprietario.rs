#[derive(Debug)]
pub struct Proprietario {
    nome: String,
    cpf: String,
    endereco: String,
}

impl Proprietario {
    pub fn new(nome: String, cpf: String, endereco: String) -> Self {
        Self {
            nome,
            cpf,
            endereco,
        }
    }

    pub fn to_string(&self) -> String {
       format!("Nome: {}, CPF: {}, Endereço: {}", self.nome, self.cpf, self.endereco) 
    }
}

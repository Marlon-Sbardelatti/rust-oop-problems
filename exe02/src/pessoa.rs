pub struct Pessoa {
    nome: String,
    idade: u32,
}

impl Pessoa {
    pub fn new(nome: String, idade: u32) -> Self {
        Self { nome, idade }
    }

    pub fn get_nome(&self) -> &String {
        &self.nome
    }

    pub fn get_idade(&self) -> &u32{
        &self.idade
    }

    pub fn to_string(&self) -> String {
       format!("Nome: {}, Idade: {}", self.get_nome(), self.get_idade()) 
    }
}


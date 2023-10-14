// a) Crie três atributos privados: nome do site (String), endereço IP do site (String), status do site (acesso livre ou bloqueado) (boolean)

// b) Sobrescreva o método toString() da classe Object para gerar uma String com os três atributos da classe.
//

#[derive(Debug)]
pub struct Site {
    nome: String,
    ip: String,
    status: bool,
}

impl Site {
    pub fn new(nome: String, ip: String, status: bool) -> Self {
        Self { nome, ip, status }
    }

    pub fn get_nome(&self) -> &String {
        &self.nome
    }
    pub fn get_ip(&self) -> &String {
        &self.ip
    }
    pub fn get_status(&self) -> &bool {
        &self.status
    }

    pub fn to_string(&self) -> String {
        format!(
            "Nome: {}, IP: {}, Status: {}",
            self.nome, self.ip, self.status
        )
    }
}

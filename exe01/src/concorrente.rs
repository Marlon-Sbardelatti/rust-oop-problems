pub struct Concorrente {
    nome: String,
    area: String,
    numero: u32,
    telefone: String,
    endereco: String,
}

impl Concorrente {
    pub fn new(
        nome: String,
        area: String,
        numero: u32,
        telefone: String,
        endereco: String,
    ) -> Self {
        Self {
            nome,
            area,
            numero,
            telefone,
            endereco,
        }
    }

    pub fn to_string(&self) -> String {
        format!(
            "Nome: {} - Área: {} - Inscrição: {} - Telefone: {} - Endereço: {}",
            self.nome, self.area, self.numero, self.telefone, self.endereco
        )
    }
}

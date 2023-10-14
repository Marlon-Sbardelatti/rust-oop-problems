pub struct Revisor {
    nome: String,
    conteudo: String,
    parecer: String,
}

impl Revisor {
    pub fn new(nome: String, conteudo: String, parecer: String) -> Self {
        Self {
            nome,
            conteudo,
            parecer,
        }
    }

    pub fn to_string(&self) -> String {
        format!("Nome Revisor: {}, Conteudo: {}, Parecer: {}", self.nome, self.conteudo, self.parecer)
    }
}

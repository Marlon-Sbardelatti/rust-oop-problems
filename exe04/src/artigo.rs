use crate::revisor::Revisor;

pub struct Artigo<'a> {
    titulo: String,
    nome: String,
    revisores: Vec<&'a Revisor>,
}

impl<'a> Artigo<'a> {
    pub fn new(titulo: String, nome: String) -> Self {
        Self {
            titulo,
            nome,
            revisores: Vec::new(),
        }
    }

    pub fn get_nome(&self) -> &String {
       &self.nome 
    }

    pub fn adicionar_revisor(&mut self, revisor: &'a Revisor) {
        self.revisores.push(revisor);
    }
    pub fn to_string(&self) -> String {
        let mut result = String::new();
        result.push_str(&format!("Titulo: {}, Autor: {}\n", self.titulo, self.nome));
        for rev in &self.revisores {
            result.push_str(&format!("{}\n", rev.to_string()));
        }

        result
    }
}

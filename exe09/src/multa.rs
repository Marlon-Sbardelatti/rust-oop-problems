#[derive(Debug)]
pub struct Multa {
    //local, tipo, valor e status
    local: String,
    tipo: String,
    valor: f64,
    status: bool,
}

impl Multa {
    pub fn new(local: String, tipo: String, valor: f64, status: bool) -> Self {
        Self {
            local,
            tipo,
            valor,
            status,
        }
    }

    pub fn get_status(&self) -> &bool {
       &self.status 
    }
    pub fn get_valor(&self) -> &f64 {
       &self.valor
    }
    pub fn to_string(&self) -> String {
        format!(
            "Local: {}, Tipo: {}, Valor: {}, Status: {}",
            self.local, self.tipo, self.valor, self.status
        )
    }
}

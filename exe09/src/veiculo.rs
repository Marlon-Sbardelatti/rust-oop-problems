use crate::multa::Multa;
use crate::proprietario::Proprietario;
#[derive(Debug)]
pub struct Veiculo<'a> {
    placa: String,
    proprietario: &'a Proprietario,
    tipo: char,
    combustivel: char,
    valor: f64,
    carga: f64,
    multas: Vec<&'a Multa>,
}

impl<'a> Veiculo<'a> {
    pub fn new(placa: String, proprietario: &'a Proprietario, tipo: char, combustivel: char, valor: f64, carga: f64) -> Self {
        Self {
            placa,
            proprietario,
            tipo,
            combustivel,
            valor,
            carga,
            multas: Vec::new(),
        }
    }
    pub fn get_placa(&self) -> &str{
       &self.placa 
    }

    pub fn addMulta(&mut self, multa: &'a Multa) {
        self.multas.push(multa);
    }

    pub fn calcular_ipva(&self) -> f64 {
        if self.tipo == 'C' {
            if self.carga >= 1000.0 {
                self.valor * 0.09
            } else {
                self.valor * 0.06
            }
        } else {
            if self.combustivel == 'A' {
                self.valor * 0.03
            } else {
                self.valor * 0.04
            }
        }
    }

    pub fn calcular_licenciamento(&self) -> f64 {
        let mut total = 0.0;
        let mut found = false;

        for multa in &self.multas {
            if multa.get_status() == &true {
               if !found {
                  found = true; 
                  total = total + self.calcular_ipva();
               } 

               total = total + multa.get_valor();
            }
        }

        total
    }

    pub fn to_string(&self) -> String {
       format!("Placa: {}", self.placa) 
    }
}

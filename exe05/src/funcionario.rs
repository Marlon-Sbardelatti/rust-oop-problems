#[derive(Debug)]
pub struct Funcionario {
    // Define your Employee struct here
    matricula: u32,
    nome: String,
    departamento: u32,
    salario: f64,
    funcao: String,
}

impl Funcionario {
    pub fn new(
        matricula: u32,
        nome: String,
        departamento: u32,
        salario: f64,
        funcao: String,
    ) -> Funcionario {
        // Initialize your Employee struct here
        Funcionario {
            // Initialize fields here
            matricula,
            nome,
            departamento,
            salario,
            funcao,
        }
    }
    pub fn get_nome(&self) -> &str {
        &self.nome
    }
    pub fn get_salario(&self) -> &f64 {
        &self.salario
    }
    pub fn get_funcao(&self) -> &String {
        &self.funcao
    }
    pub fn get_departamento(&self) -> &u32 {
        &self.departamento
    }

    pub fn get_matricula(&self) -> &u32 {
        &self.matricula
    }

    pub fn to_string(&self) -> String {
        format!(
            "Nome: {}, Matricula: {}, Departamento: {}, Salário: {}, Função: {}",
            self.nome, self.matricula, self.departamento, self.salario, self.funcao
        )
    }
}

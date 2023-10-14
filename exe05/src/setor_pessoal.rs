use crate::funcionario::Funcionario;
pub struct SetorPessoal {
    funcionarios: Vec<Funcionario>,
}

impl SetorPessoal {
    pub fn new() -> SetorPessoal {
        SetorPessoal {
            funcionarios: Vec::new(), 
        }
    }

    pub fn add_funcionario(&mut self, funcionario: Funcionario) {
        self.funcionarios.push(funcionario);
    }

    pub fn print_all(&self) {
        for func in &self.funcionarios {
            println!("{:?}", func);
        }
    }

    pub fn imprimir_folha(&self) -> String {
        let mut dados = String::new();
        for func in &self.funcionarios {
            dados.push_str(&format!(
                "Nome: {}, Salario: {}\n",
                func.get_nome(),
                func.get_salario()
            ));
        }
        dados
    }

    pub fn valor_total_folha(&self) -> String {
        let mut total = 0.0;
        for func in &self.funcionarios {
            total = total + func.get_salario();
        }
        let result = format!("O valor total da folha é: R${}", total);
        result
    }

    pub fn maior_salario(&self) -> String {
        let mut maior = 0.0;
        for func in &self.funcionarios {
            if func.get_salario() > &maior {
                maior = *func.get_salario();
            }
        }
        let mut dados = format!("O maior salário é de: R${}\n", maior);

        for func in &self.funcionarios {
            if func.get_salario() == &maior {
                dados.push_str(&format!("Nome: {}\n", func.get_nome()));
            }
        }
        dados
    }

    pub fn buscar_departamento(&self, numero_departamento: u32) -> String {
        let mut dados = format!(
            "Os funcionarios no departamento {} são:\n",
            numero_departamento
        );
        for func in &self.funcionarios {
            if func.get_departamento() == &numero_departamento {
                dados.push_str(&format!(
                    "Nome: {}, função: {}\n",
                    func.get_nome(),
                    func.get_funcao()
                ));
            }
        }
        dados
    }

    pub fn buscar_funcao(&self, funcao: String) -> String {
        let mut dados = format!("Os funcionarios que exercem a função {} são:\n", funcao);
        for func in &self.funcionarios {
            if funcao.to_lowercase() == *func.get_funcao().to_lowercase() {
                dados.push_str(&format!("Nome: {}\n", func.get_nome(),));
            }
        }
        dados
    }

    pub fn imprimir_folha_crescente(&self) -> String {
        let mut new_vec: Vec<(String, f64)> = Vec::new();
        let mut dados = String::new();

        for func in &self.funcionarios {
            new_vec.push((func.get_nome().to_string(), *func.get_salario()));
        }

        new_vec.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        for (name, salary) in &new_vec {
            dados.push_str(&format!("Nome: {}, Salário: R${:.2}\n", name, salary));
        }
        dados
    }
}

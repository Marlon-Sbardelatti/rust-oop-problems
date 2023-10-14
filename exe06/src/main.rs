mod pais;
use pais::Pais;
fn main() {
    let mut p1 = Pais::new(String::from("Brazil"), String::from("Brasilia"), 256.00);
    let mut p2 = Pais::new(String::from("Argentina"), String::from("Buenos Aires"), 256.00);
    let p3 = Pais::new(String::from("Uruguai"), String::from("Montevideu"), 256.00);
    let p4 = Pais::new(String::from("Estados Unidos"), String::from("Washington"), 256.00);

    p1.adicionar_vizinho(p2.clone());
    p1.adicionar_vizinho(p3.clone());
    p2.borrow_mut().adicionar_vizinho(p3.clone());

    let vizinhos_comuns = p1.borrow().lista_vizinhos(&p2.borrow());
    println!("Países que fazem fronteira comuns entre Brasil e Argentina: {:?}", vizinhos_comuns);
    // let mut p1 = Pais::new(String::from("Brazil"), String::from("Brasilia"), 256.00);
    // let mut p2 = Pais::new(String::from("Argentina"), String::from("Buenos Aires"), 256.00);
    // let p3 = Pais::new(String::from("Uruguai"), String::from("Montevideu"), 256.00);
    // let p4 = Pais::new(String::from("Estados Unidos"), String::from("Washington"), 256.00);
    // // println!("{}", p1.equals(&p3));
    // // p1.adicionar_vizinho(&p2);
    // // p1.adicionar_vizinho(&p3);
    // // p2.adicionar_vizinho(&p3);
    //  p1.adicionar_vizinho(&p2);
    // p1.adicionar_vizinho(&p3);

    // p2.adicionar_vizinho(&p3);

    // // Now you can safely call lista_vizinhos with p2
    // let vizinhos_comuns = p1.lista_vizinhos(&p2);
    // println!("Países que fazem fronteira comuns entre Brasil e Argentina: {:?}", vizinhos_comuns);


    // println!("{:?}", p1.lista_vizinhos(&p2));

    // println!("{}", p1.faz_fronteira(&p4));
}
// 6. Escreva uma classe que represente um “País”. Um país tem como atributos o seu nome e a sua dimensão em
// Km2 e uma lista de países com os quais ele faz fronteira.
// Represente a classe e forneça os seus membros a seguir:

// a) Um método que permita verificar se dois países são iguais. Dois países são iguais se tiverem o mesmo nome e a mesma capital.
// A assinatura deste método deve ser:
// − public boolean equals(Pais outro)

// b) Um método que informe se um outro país é seu limítrofe (faz fronteira)

// c) Um método que receba um outro país como parâmetro e retorne uma lista de vizinhos comuns aos dois países. Obs.:
// Considere que um país tem no máximo 40 outros países com os quais faz fronteira.

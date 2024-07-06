fn main() {
    //as variáveis em Rust são imutáveis por padrão
    //usa-se let e um nome para definir uma variável 
    
    let nome: &str = "Gabriel";
    let mut idade: i32 = 19; //o mut torna uma variável em mutável
    idade = idade + 1;
    println!("Olá, {nome}, você tem {idade} anos");
    
    const LINGUAGEM: &str = "Rust"; //definindo uma constante
    println!("Linguagem: {LINGUAGEM}");
}

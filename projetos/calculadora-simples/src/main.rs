use std::io;

fn main(){
  let mut operacao = String::new();
  let mut num1 = String::new();
  let mut num2 = String::new();
  
  println!("Escolha uma operação (+ , - , x , /) : ");
  io::stdin().read_line(&mut operacao).expect("Falha ao ler a linha");
  let operacao = operacao.trim();
  
  if operacao == "+" {
    println!("Digite o primeiro número: ");
    io::stdin().read_line(&mut num1).expect("Falha ao ler a linha");
    println!("Digite o segundo número: ");
    io::stdin().read_line(&mut num2).expect("Falha ao ler a linha");
    
    let num1 = num1.trim().parse::<i32>().expect("error");
    let num2 = num2.trim().parse::<i32>().expect("error");
    
    println!("Resultado: "); soma(num1, num2);
  }
  
  else if operacao == "-" {
    println!("Digite o primeiro número: ");
    io::stdin().read_line(&mut num1).expect("error");
    println!("Digite o segundo número: ");
    io::stdin().read_line(&mut num2).expect("error");
    
    let num1 = num1.trim().parse::<i32>().expect("error");
    let num2 = num2.trim().parse::<i32>().expect("error");
    
    println!("Resultado: "); sub(num1, num2);
  }
  
  else if operacao == "x" {
    println!("Digite o primeiro número: ");
    io::stdin().read_line(&mut num1).expect("error");
    println!("Digite o segundo número: ");
    io::stdin().read_line(&mut num2).expect("error");
    
    let num1 = num1.trim().parse::<i32>().expect("error");
    let num2 = num2.trim().parse::<i32>().expect("error");
    
    println!("Resultado: "); multi(num1, num2);
  }
  
  else if operacao == "/" {
    println!("Digite o primeiro número: ");
    io::stdin().read_line(&mut num1).expect("error");
    println!("Digite o segundo número: ");
    io::stdin().read_line(&mut num2).expect("error");
    
    let num1 = num1.trim().parse::<i32>().expect("error");
    let num2 = num2.trim().parse::<i32>().expect("error");
    
    println!("Resultado: "); div(num1, num2);
  }
  
  else {
    println!("Operação inválida");
  }
}

fn soma(num1: i32, num2: i32){
  println!("{}", num1 + num2);
}

fn sub(num1: i32, num2: i32){
  println!("{}", num1 - num2);
}

fn multi(num1: i32, num2: i32){
  println!("{}", num1 * num2);
}

fn div(num1: i32, num2: i32){
  println!("{}", num1 / num2);
}
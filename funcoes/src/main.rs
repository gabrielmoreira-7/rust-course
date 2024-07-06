fn main() {
  println!("SOMA: {}", soma(5, 2));
  println!("DIVISÃO: {}", div(10, 2));
}

fn soma(num1: i32, num2: i32) -> i32 {
  num1 + num2
}

fn div(num1: i32, num2: i32) -> i32 {
  num1 / num2
}
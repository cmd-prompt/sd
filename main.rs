fn check_armstrong(number:u32) {
  let digits : Vec<u32> = number.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect();

  use std::convert::TryInto;
  let sum_digit : u32 = digits.len().try_into().unwrap();
  let mut sum = 0 ;

  for item in digits{
    sum = sum + u32::pow(item, sum_digit);

  }
  if sum == number {
  println!("{} is an armstrong number" ,number);

  } else {
  println!("{} is not an armstrong number" ,number)
    
  }
}

  fn main() {
    use std::io::{stdin,stdout,Write};
    loop {
      let mut number = String::new();
      print!("Input your number: ");
      let _=stdout().flush();
      stdin().read_line(&mut number).expect("Did not enter a correct string");
      if let Some('\n')=number.chars().next_back() {
          number.pop();
      }
      if let Some('\r')=number.chars().next_back() {
          number.pop();
      }
      check_armstrong(number.parse::<u32>().unwrap());
    }
    
  }

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
    check_armstrong(153)
  }

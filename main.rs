fn check_if_leap_year(number:u32) {
  if number % 4 == 0  { 
    if number % 100 >= 1{
      println!("{} is a leap year", number)
    }else  if number % 400 == 0 {
      println!("{} is a leap year", number)
    }else{
      println!("{} is not a leap year", number)
    }
    }
  else{
    println!("{} is not a leap year", number)
  }}


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
      check_if_leap_year(number.parse::<u32>().unwrap());
    }
    
  }
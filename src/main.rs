fn main() {
    let _number: i32 = 99;
    let second_variable: i32 = 999;

    let mut  _number_b: i32 = 1;
    println!("second_variable is {}", second_variable);

    _number_b=80;
    println!("_number_b is {}", _number_b);

    test_for();
    test_if(40);
}

fn test_for() {
    
      for i  in 0..10 {
        println!("{}", i);
      }
      
      let mut i = 0;
      
      while i < 10 {
          println!("hello");
          i = i + 1;
      }

      let _result_a: u32 = sum(10, 20);
      println!("result_a = {}", _result_a);

  }
  
fn test_if(data: u32) {
    

    if data > 49 {
        println!("Passed!");
      } else {
        println!("F!");
      }


  }
  fn sum(a: u32, b: u32) -> u32 {
    return a + b;
  }
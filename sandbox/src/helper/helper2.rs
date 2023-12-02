use crate::helper::constants::*;

pub fn get_string() -> String {
  let a = String::from("Hello world");
  a
}

pub fn return_pi() -> f64 {
  _PI
}

pub fn mod_val(input: &mut i64) {
  *input = 2* (*input);
}


#[cfg(test)]
mod tests {
  use crate::helper::helper2::*;

  #[test]
  fn test_five() {
    // print!(_PI);
    let val = return_pi() - 3.1415926;
    assert_eq!(val < 0.1, true);

    assert_eq!(2, _BEST_NUMBER);
  }
  #[test]
  fn test_input() {
    let mut first_val: i64 = 5;
    mod_val(&mut first_val);
    assert_eq!(first_val, 10);
  }
}


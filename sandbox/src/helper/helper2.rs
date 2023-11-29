use crate::helper::constants::*;

pub fn get_string() -> String {
  let a = String::from("Hello world");
  a
}

pub fn return_pi() -> f64 {
  _PI
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
}


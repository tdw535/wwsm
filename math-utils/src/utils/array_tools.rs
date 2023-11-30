
// pub fn tranpose_2d_complex (
//   vec: &mut Vec<Complex<f64>>, nx: usize, ny: usize) {
//     for row in 0..nx {
//       for col in 0..ny {
//         let mut temp = vec[ny*row + col];
//         vec[ny*row + col] = vec[nx*col + row];
//         vec[nx*col + row] = temp;
//       }
//     }
// }
pub fn tranpose_2d (vec: &mut Vec<i64>, nx: usize, ny: usize) {
    for row in 0..nx {
      for col in 0..ny {
        let mut temp = vec[ny*row + col];
        vec[ny*row + col] = vec[nx*col + row];
        vec[nx*col + row] = temp;
      }
    }
}



#[cfg(test)]
mod tests {
  use crate::utils::*;
  // use rustfft::{num_complex::Complex};
  #[test]
  fn test_tranpose() {
    let nx = 4;
    let ny = 3;
    let mut v: Vec<i64> = (1..13).collect();
    for el in &v {
      print!("{} ", el);
    }
    print!("\n");
    array_tools::tranpose_2d (&mut v, nx, ny);
    for el in &v {
      print!("{} ", el);
    }     
    assert_eq!(true, false); 
  }

}
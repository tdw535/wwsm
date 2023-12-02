
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
pub fn tranpose_2d<T: Copy> (vec: &mut Vec<T>, n_row: usize, n_col: usize) {
  let mut temp: Vec<T> =  vec.clone();
    for row in 0..n_row {
      for col in 0..n_col {
        temp[n_row*col + row] = vec[n_col*row + col];
      }
    }
  *vec = temp;
}



#[cfg(test)]
mod tests {
  use crate::utils::*;
  use rustfft::{num_complex::Complex};
  #[test]
  fn test_tranpose_int() {
    let n_row:usize = 2;
    let n_col:usize = 3;
    let upto:i64 = (n_row*n_col) as i64;
    let mut v: Vec<i64> = (0..upto).collect();
    for el in &v {
      print!("{} ", el);
    }
    print!("\n");
    array_tools::tranpose_2d(&mut v, n_row, n_col);
    for el in &v {
      print!("{} ", el);
    }     
    assert_eq!(v, [0, 3, 1, 4, 2, 5]); 
  }

  #[test]
  fn test_tranpose_complex() {
    let n_row:usize = 2;
    let n_col:usize = 3;
    let upto:i64 = (n_row*n_col) as i64;
    let zero: Complex<f64> = Complex::new(0.0,0.0);

    let mut v: Vec<Complex<f64>> = Vec::with_capacity(6);
    for ii in 0..upto {
      v.push(zero);
    }
    for el in &v {
      print!("{} ", el);
    }
    print!("\n");
    array_tools::tranpose_2d(&mut v, n_row, n_col);
    for el in &v {
      print!("{} ", el);
    }     
    assert_eq!(v, [zero, zero, zero, zero, zero, zero]); 
  }

}

pub struct Vector2D<T> {
  num_row: usize,
  num_col: usize,
  vec: Vec<T>
}


impl<T: Default> Vector2D<T> {
  pub fn new(num_row: usize, num_col: usize) -> Vector2D<T> {
    let vec_size: usize = num_row*num_col;
    let mut vec: Vec<T> = Vec::with_capacity(vec_size);
    for ind in 0..vec_size {
      vec.push(Default::default());
    }
    Vector2D {num_row, num_col, vec} 
  }
  pub fn len(self) -> usize {
    self.num_row * self.num_col
  }
  pub fn get(self, row: usize, col: usize) -> usize {
    row*self.num_col + col
  }
}


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
  #[test]
  fn test_vector2d() {
    let default_value: i64 = 0;
    let nrow: usize = 2;
    let ncol: usize = 3;
    let vec_2d = array_tools::Vector2D::<Complex<i64>>::new(nrow, ncol);
    let complex_zero = Complex::new(0,0);
    for el in &vec_2d.vec {
      assert_eq!(el.re, complex_zero.re);
      assert_eq!(el.im, complex_zero.im); 
    }   
    assert_eq!(vec_2d.len(), nrow*ncol);
    assert_eq!(*vec_2d.get(3,2), complex_zero);
    

  }
}
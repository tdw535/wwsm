
use std::fmt; // Import `fmt`
use core::ops::Add;
pub struct Vector2D<T> {
  num_row: usize,
  num_col: usize,
  vec_size: usize,
  vec: Vec<T>
}


impl<T: Add + Copy + Default> Vector2D<T> {
  pub fn new(num_row: usize, num_col: usize) -> Vector2D<T> {
    let vec_size: usize = num_row*num_col;
    let mut vec: Vec<T> = Vec::with_capacity(vec_size);
    for _ind in 0..vec_size {
      vec.push(Default::default());
    }
    Vector2D {num_row, num_col, vec_size, vec} 
  }
  #[inline]
  pub fn len(&self) -> usize {
    self.vec_size
  }  

  pub fn tranpose_2d (&self) -> Vector2D<T> {
    // Column and row dim need to be switched
    let mut tranposed: Vector2D<T> =  Vector2D::<T>::new(self.num_col, self.num_row);
      for row in 0..self.num_row {
        for col in 0..self.num_col {
          tranposed.vec[self.num_row*col + row] = self.vec[self.num_col*row + col];
        }
      }
      tranposed
  }

  pub fn get_row(&mut self, row:usize) -> &mut [T] {
    &mut self.vec[row*self.num_col..(row+1)*self.num_col]
  }

}

impl<T: fmt::Display> fmt::Display for Vector2D<T> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      // Use `self.number` to refer to each positional data point.
      let mut space_separated = String::new();
      for row in 0..self.num_row {
        for col in 0..self.num_col {
          space_separated.push_str(&self[row][col].to_string());
          space_separated.push_str(" ");
        }
        space_separated.push_str("\n");
      }
      write!(f,"{}",space_separated)
  }
}

impl<T> std::ops::Index<usize> for Vector2D<T> {
  type Output = [T];

  fn index(&self, row: usize) -> &[T] {
    let start = self.num_col * row;
    &self.vec[start .. start + self.num_col]
  }
}


impl<T> std::ops::IndexMut<usize> for Vector2D<T> {

  fn index_mut(&mut self, row: usize) -> &mut [T] {
    let start = self.num_col * row;
    &mut self.vec[start .. start + self.num_col]
  }   
}

// impl<T: Add> core::ops::Add<Vector2D<T>> for Vector2D<T> {
//   type Output = Vector2D<T>;

//   fn add(self, _rhs: Vector2D<T>) -> Vector2D<T> {
//     let mut result: Vector2D<T> =  Vector2D::<T>::new(self.num_row, self.num_col);

//     for ind in 0..self.vec_size {
//       result.vec[ind] = self.vec[ind] + _rhs.vec[ind];
//     }
//     result
//   }
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
    for _ii in 0..upto {
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
    let nrow: usize = 2;
    let ncol: usize = 3;
    let mut vec_2d = array_tools::Vector2D::<Complex<i64>>::new(nrow, ncol);
    let complex_zero = Complex::new(0,0);
    for el in &vec_2d.vec {
      assert_eq!(el.re, complex_zero.re);
      assert_eq!(el.im, complex_zero.im); 
    }   
    let vec_len = vec_2d.len();
    vec_2d[1][2] = Complex::new(1,2);


    assert_eq!(vec_len, nrow*ncol);
    assert_eq!(vec_2d[1][2], Complex::new(1,2));
    let mut vec_t = vec_2d.tranpose_2d();
    assert_eq!(vec_2d[1][2], vec_t[2][1]);
    vec_t[2][1] = Complex::new(4,2);
    assert_eq!(Complex::new(4,2), vec_t[2][1]);
  }

  #[test]
  fn test_get_row() {
    let nrow: usize = 2;
    let ncol: usize = 3;
    let mut vec_2d = array_tools::Vector2D::<Complex<i64>>::new(nrow, ncol);
    vec_2d[0][0] = Complex::new(1,2);
    vec_2d[1][0] = Complex::new(5,2);

    let mut vec_row = vec_2d.get_row(1);
    vec_row[1] = Complex::new(-1,-2);

    let mut vec_row = vec_2d.get_row(0);
    vec_row[2] = Complex::new(-7,-5);
    
    print!("{}", vec_2d);
    assert_eq!(vec_2d[0][2],Complex::new(-7,-5));
    assert_eq!(vec_2d[1][1],Complex::new(-1,-2));
  }

  // #[test]
  // fn test_add() {
  //   let nrow: usize = 2;
  //   let ncol: usize = 3;
  //   let mut vec_2d = array_tools::Vector2D::<Complex<i64>>::new(nrow, ncol);
  //   let mut vec_second = array_tools::Vector2D::<Complex<i64>>::new(nrow, ncol);
  //   vec_2d[0][0] = Complex::new(1,2);
  //   vec_2d[1][0] = Complex::new(5,2);

  //   vec_second[0][0] = Complex::new(1,2);
  //   vec_second[1][0] = Complex::new(6,3);    

  //   let mut result_add = vec_2d + vec_second;
  //   print!("{}", result_add);


  // }
}
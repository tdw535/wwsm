
use std::fmt; // Import `fmt`

// Maybe want to use builder pattern here?

pub struct Vector2D<T> {
  num_row: usize,
  num_col: usize,
  vec_size: usize,
  vec: Vec<T>
}



impl<T: Default + Copy> Vector2D<T> {
  pub fn new(num_row: usize, num_col: usize) -> Vector2D<T>
  where  
    T: Default, 
    T: Copy,
  {
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

  pub fn tranpose_2d (&self) ->  Vector2D<T> {
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

use std::ops::Add;
impl<T: Default + Copy> core::ops::Add<&Vector2D<T>> for &Vector2D<T> where T: Add<T, Output=T>{
  type Output =  Vector2D<T>;

  fn add(self, _rhs: &Vector2D<T>) ->  Self::Output {
    let mut result: Vector2D<T> =  Vector2D::<T>::new(self.num_row, self.num_col);

    for ind in 0..self.vec_size {
      result.vec[ind] = self.vec[ind] + _rhs.vec[ind];
    }
    result
  }
}

use std::ops::Mul;
impl<T: Default + Copy> core::ops::Mul<T> for &Vector2D<T> where T: Mul<T, Output=T>{
  type Output =  Vector2D<T>;

  fn mul(self, _s: T) ->  Self::Output {
    let mut result: Vector2D<T> =  Vector2D::<T>::new(self.num_row, self.num_col);

    for ind in 0..self.vec_size {
      result.vec[ind] = self.vec[ind] * _s;
    }
    result
  }
}

// pub fn tranpose_2d<T: Copy> (vec: &mut Vec<T>, n_row: usize, n_col: usize) {
//   let mut temp: Vec<T> =  vec.clone();
//     for row in 0..n_row {
//       for col in 0..n_col {
//         temp[n_row*col + row] = vec[n_col*row + col];
//       }
//     }
//   *vec = temp;
// }



#[cfg(test)]
mod tests {
  use crate::utils::*;
  use rustfft::{num_complex::Complex};

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

    let vec_row = vec_2d.get_row(1);
    vec_row[1] = Complex::new(-1,-2);

    let vec_row = vec_2d.get_row(0);
    vec_row[2] = Complex::new(-7,-5);
    
    print!("{}", vec_2d);
    assert_eq!(vec_2d[0][2],Complex::new(-7,-5));
    assert_eq!(vec_2d[1][1],Complex::new(-1,-2));
  }

  #[test]
  fn test_add() {
    let nrow: usize = 2;
    let ncol: usize = 3;
    let mut vec_2d = array_tools::Vector2D::<Complex<i64>>::new(nrow, ncol);
    let mut vec_second = array_tools::Vector2D::<Complex<i64>>::new(nrow, ncol);
    vec_2d[0][0] = Complex::new(1,2);
    vec_2d[1][0] = Complex::new(5,2);
    vec_2d[0][2] = Complex::new(-1,2);
    vec_2d[1][2] = Complex::new(-4,-2);    

    vec_second[0][0] = Complex::new(1,2);
    vec_second[1][0] = Complex::new(6,3);    

    vec_second[0][2] = Complex::new(-6,4);
    vec_second[1][2] = Complex::new(-2,-1);   


    let mut vec_expected = array_tools::Vector2D::<Complex<i64>>::new(nrow, ncol);
    vec_expected[0][0] = Complex::new(2,4);
    vec_expected[1][0] = Complex::new(11,5); 
    vec_expected[0][2] = Complex::new(-7,6);
    vec_expected[1][2] = Complex::new(-6,-3); 
       
    print!("{}", vec_2d);
    print!("\n");
    print!("{}", vec_second);
    print!("\n");
    let result_add = &vec_2d + &vec_second;
    print!("{}", result_add);
    print!("\n");
    print!("{}", vec_second);
    print!("\n");
    print!("{}", vec_2d);

    assert_eq!(result_add[1][2],vec_expected[1][2]);
    assert_eq!(vec_second[1][2],Complex::new(-2,-1))
  }
  #[test]
  fn test_multiply() {
    let nrow: usize = 2;
    let ncol: usize = 3;
    let mut vec_2d = array_tools::Vector2D::<Complex<i64>>::new(nrow, ncol);
    vec_2d[0][0] = Complex::new(1,2);
    vec_2d[1][0] = Complex::new(5,2);

    let scalar:Complex<i64> = Complex::new(3,4);

    let mut vec_expected = array_tools::Vector2D::<Complex<i64>>::new(nrow, ncol);
    vec_expected[0][0] = Complex::new(-5,10);
    vec_expected[1][0] = Complex::new(7,26); 

    print!("{}", vec_2d);
    print!("\n");
    let result_multiply = &vec_2d*scalar;
    print!("{}", result_multiply);
    print!("\n");
    print!("{}", vec_2d);
    print!("\n");
    assert_eq!(result_multiply[1][2],vec_expected[1][2]);
    assert_eq!(result_multiply[0][0],vec_expected[0][0]);
    assert_eq!(result_multiply[1][0],vec_expected[1][0]);

  }  

}
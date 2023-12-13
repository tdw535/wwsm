use crate::utils::array_tools::Vector2D;

use rustfft::{num_complex::Complex};

pub struct ShallowWaterSolution {
  u: Vector2D<Complex<f64>>,
  v: Vector2D<Complex<f64>>,
  h: Vector2D<Complex<f64>>
}

impl ShallowWaterSolution {
  pub fn new(dim_1: usize, dim_2: usize) -> ShallowWaterSolution {
    let mut u: Vector2D<Complex<f64>> = Vector2D::<Complex<f64>>::new(dim_1,dim_2);
    let mut v: Vector2D<Complex<f64>> = Vector2D::<Complex<f64>>::new(dim_1,dim_2);
    let mut h: Vector2D<Complex<f64>> = Vector2D::<Complex<f64>>::new(dim_1,dim_2);
    ShallowWaterSolution{u, v, h}
  }
}

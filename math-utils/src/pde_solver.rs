
use crate::utils::fft2d_manager;
use crate::utils::array_tools::Vector2D;

use rustfft::{num_complex::Complex};

pub struct PDESolver {
  dim_1: usize,
  dim_2: usize,
  width_1: f64,
  width_2: f64,
  dx_1: f64,
  dx_2: f64,
  fft2d_manager: fft2d_manager::FFT2DManager,
  sol: Vector2D<Complex<f64>>
}

impl PDESolver {
  pub fn new(dim_1: usize, dim_2: usize, width_1: f64, width_2: f64) -> PDESolver {
    let fft2d_manager = fft2d_manager::FFT2DManager::new(dim_1,dim_2);
    let dx_1 = width_1/(dim_1 as f64);
    let dx_2 = width_2/(dim_2 as f64);
    let mut sol: Vector2D<Complex<f64>> = Vector2D::<Complex<f64>>::new(dim_1,dim_2);
    fft2d_manager::init_vec(&mut sol,width_1, width_2, dim_1, dim_2, &f64::cos);

    PDESolver{dim_1, dim_2, width_1, width_2, dx_1, dx_2, fft2d_manager, sol}
  }
  pub fn get_dx1(self) -> f64 {
    self.dx_1
  }
  pub fn get_dx2(self) -> f64 {
    self.dx_2
  }  

  // dt(h) + H(dx(u) + dy(v)) = 0
  // dt(u) - fv = -gdx(h) - ku
  // dt(v) + fu = -gdy(h) - kv

  // Add time step approach
  //  y_{n+1}=y_{n}+h*f(t_{n},y_{n}).}
}



#[cfg(test)]
mod tests {
  use crate::pde_solver::*;

  #[test]
  fn test_setup() {
    let solver = PDESolver::new(4,8,1.0,1.0);
    let val:f64 = solver.get_dx1();
    assert_eq!(1.0/4.0, val);
  }
}
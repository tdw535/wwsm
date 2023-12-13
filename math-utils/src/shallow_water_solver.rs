
use crate::utils::fft2d_manager;

use crate::shallow_water_solution;

use crate::utils::array_tools::Vector2D;

use rustfft::{num_complex::Complex};



pub struct ShallowWaterSolver {
  dim_1: usize,
  dim_2: usize,
  width_1: f64,
  width_2: f64,
  dx_1: f64,
  dx_2: f64,
  fft2d_manager: fft2d_manager::FFT2DManager,
  sol: shallow_water_solution::ShallowWaterSolution,
  sol_hat: shallow_water_solution::ShallowWaterSolution
}


impl ShallowWaterSolver {
  pub fn new(dim_1: usize, dim_2: usize, width_1: f64, width_2: f64) -> ShallowWaterSolver {
    let fft2d_manager = fft2d_manager::FFT2DManager::new(dim_1,dim_2);
    let dx_1 = width_1/(dim_1 as f64);
    let dx_2 = width_2/(dim_2 as f64);

    let sol = shallow_water_solution::ShallowWaterSolution::new(dim_1, dim_2);
    let sol_hat = shallow_water_solution::ShallowWaterSolution::new(dim_1, dim_2);

    // Read in data
    // Fourier transform sol into sol hat

    ShallowWaterSolver{dim_1, dim_2, width_1, width_2, dx_1, dx_2, fft2d_manager, sol, sol_hat}
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

  // U_{n+1} = U_{n} + dt [-H*(ik u_hat + il v_hat), f*v_hat - g(ik * h_hat) - k*u_hat, -f*u_hat - g(il*h_hat) - k*u_hat]

  // Add time step approach
  //  y_{n+1}=y_{n}+h*f(t_{n},y_{n}).}
  // pub fn euler_time_step_iterate(){
  //   sol = sol + dt*RHS(sol)
  // }
}



#[cfg(test)]
mod tests {
  use crate::shallow_water_solver::*;

  #[test]
  fn test_setup() {
    let solver = ShallowWaterSolver::new(4,8,1.0,1.0);
    let val:f64 = solver.get_dx1();
    assert_eq!(1.0/4.0, val);
  }
}
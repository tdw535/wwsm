
use crate::utils::constants::_PI;
use crate::utils::fft2d_manager;

use crate::utils::array_tools;
use crate::utils::array_tools::Vector2D;


use rustfft::{num_complex::Complex};


pub struct WaveEquationSolver {
  dim_1: usize,
  dim_2: usize,
  width_1: f64,
  width_2: f64,
  dx_1: f64,
  dx_2: f64,
  fft2d_manager: fft2d_manager::FFT2DManager,
  sol: Vector2D<Complex<f64>>,
  sol_hat: Vector2D<Complex<f64>>,
  sol_real: Vector2D<f64>
}


impl WaveEquationSolver {
  pub fn new(dim_1: usize, dim_2: usize, width_1: f64, width_2: f64) -> WaveEquationSolver {
    let fft2d_manager = fft2d_manager::FFT2DManager::new(dim_1,dim_2);
    let dx_1: f64 = width_1/(dim_1 as f64);
    let dx_2: f64 = width_2/(dim_2 as f64);

    let mut sol: Vector2D<Complex<f64>> = Vector2D::new(dim_1, dim_2);
    let mut sol_hat: Vector2D<Complex<f64>> = Vector2D::new(dim_1, dim_2);
    let mut sol_real: Vector2D<f64> = Vector2D::new(dim_1, dim_2);

    WaveEquationSolver{dim_1, dim_2, width_1, width_2, dx_1, dx_2, fft2d_manager, sol, sol_hat, sol_real}
  }
  pub fn get_dx1(self) -> f64 {
    self.dx_1
  }
  pub fn get_dx2(self) -> f64 {
    self.dx_2
  }
  // Should ensure that dims and widths don't change
  pub fn init(&mut self, init_val: Vector2D<f64>) {
    // Convert f64 -> c64
    self.sol = array_tools::convert_vector2d_real_to_complex(init_val);
    self.dim_1 = self.sol.get_num_row();
    self.dim_2 = self.sol.get_num_col();
    self.dx_1 = self.width_1/(self.dim_1 as f64);
    self.dx_2 = self.width_2/(self.dim_2 as f64);
    // Get sol_hat
    // 
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
  use crate::wave_equation_solver::*;

  #[test]
  fn test_setup() {
    let wx = 2.0*_PI;
    let solver = WaveEquationSolver::new(4,8,wx,wx);
    let val:f64 = solver.get_dx1();
    println!("{}", val);
    assert_eq!(wx/4.0, val);
  }

}
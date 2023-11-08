mod fft_manager

pub mod math_utils {
  pub struct FFT2DManager {
    dim_x: usize,
    dim_y: usize,
    manager_x: FFTManager,
    manager_y: FFTManager,
  }
  impl FFT2DManager {
    pub fn new(dim_x: usize, dim_y: usize) -> FFT2DManager {
      let mut manager_x = FFTManager::new();
      let mut manager_y = FFTManager::new();          
      FFT2DManager{dim_x, dim_y, manager_x, manager_y}      
    }
    pub fn fft2d_forward(&self,buffer: &mut Vec<Complex<f64>>) -> bool {
      // Apply forward on each vector slice (in parrallel and then same for transpose)
      // self.plan_forward.process(buffer);
      true
  }
}

#[cfg(test)]
mod tests {
    use super::math_utils::*;
    use rustfft::{num_complex::Complex};
    #[test]
    // Test sin(X)
    fn test_sin() {
      assert_eq!(true,false);
    }
}
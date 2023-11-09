

pub mod math_utils {

  mod fft_manager;
  mod constants;

  use rustfft::{FftPlanner, num_complex::Complex};


  pub struct FFT2DManager {
    dim_1: usize,
    dim_2: usize,
    manager_1: FFTManager,
    manager_2: FFTManager,
  }
  impl FFT2DManager {
    pub fn new(dim_1: usize, dim_2: usize) -> FFT2DManager {
      let mut manager_1 = FFTManager::new(dim_1);
      let mut manager_2 = FFTManager::new(dim_2);          
      FFT2DManager{dim_1, dim_2, manager_1, manager_2}      
    }
    
    pub fn fft2d_forward(&self,buffer: &mut Vec<Complex<f64>>) -> bool {
      // Apply forward on each vector slice (in parrallel and then same for transpose)
      // self.plan_forward.process(buffer);
      for row in 0..dim_2 {
        self.manager_1.plan_forward.process(buffer[row*dim_1..(row+1)*dim_1]);
      }
      true
    }
  }
  
  pub fn init_vec(vec: &mut Vec<Complex<f64>>, width_x: f64, width_y: f64, nx: usize, ny: usize, f: &dyn Fn(f64) -> f64) {

    let dx:f64 = (2.0*_PI)/(width_x*((nx) as f64));
    let dy:f64 = (2.0*_PI)/(width_y*((ny) as f64));
    for jj in 0..ny {
      for ii in 0..nx {
        let grid_value = Complex::new(f(dx* (ii as f64)),0.0);
        vec.push(grid_value);
      }
    }
  }
}

#[cfg(test)]
mod tests {
    use super::math_utils::*;
    use rustfft::{num_complex::Complex};
    #[test]
    // Test sin(X)
    fn test_sin() {
      let width = 1.0;
      let nx: usize = 32;
      let ny: usize = 1;
      let mut v: Vec<Complex<f64>> = Vec::new();
      let mut v2: Vec<Complex<f64>> = Vec::new();
      init_vec(&mut v,width,width, nx,ny, &f64::sin);

      for el in &v {
        print!("{} ", el);
      }        
      assert_eq!(true,false);
    }
}
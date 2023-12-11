
use crate::utils::constants::*;
use crate::utils::array_tools::*;
use crate::utils::fft_manager::*;


use rustfft::{num_complex::Complex};


pub struct FFT2DManager {
  dim_1: usize,
  dim_2: usize,
  manager_1: FFTManager,
  manager_2: FFTManager,
}
impl FFT2DManager {
  pub fn new(dim_1: usize, dim_2: usize) -> FFT2DManager {
    let manager_1 = FFTManager::new(dim_1);
    let manager_2 = FFTManager::new(dim_2);          
    FFT2DManager{dim_1, dim_2, manager_1, manager_2}      
  }
  
  pub fn fft2d_forward(&self,buffer: &mut Vector2D<Complex<f64>>) -> bool {
    // Apply forward on each vector slice (in parrallel and then same for transpose)
    // self.plan_forward.process(buffer);
    
    for row in 0..self.dim_1 {
      self.manager_2.fft_forward(&mut buffer.get_row(row));
    }
    let mut buffer_tranposed = buffer.tranpose_2d();
    for col in 0..self.dim_2 {
      self.manager_1.fft_forward(&mut buffer_tranposed.get_row(col));
    }    
    *buffer = buffer_tranposed.tranpose_2d();

    true
  }

  pub fn fft2d_inverse(&self,buffer: &mut Vector2D<Complex<f64>>) -> bool {
    // Apply forward on each vector slice (in parrallel and then same for transpose)
    // self.plan_forward.process(buffer);
    
    for row in 0..self.dim_1 {
      self.manager_2.fft_inverse(&mut buffer.get_row(row));
    }
    let mut buffer_tranposed = buffer.tranpose_2d();
    for col in 0..self.dim_2 {
      self.manager_1.fft_inverse(&mut buffer_tranposed.get_row(col));
    }    
    *buffer = buffer_tranposed.tranpose_2d();

    true
  }

  pub fn fft2d_normalized_inverse(&self,buffer: &mut Vector2D<Complex<f64>>) -> bool {
    self.fft2d_inverse(buffer);
    self.fft2d_normalize(buffer);
    true
  }        

  fn fft2d_normalize(&self, buffer: &mut Vector2D<Complex<f64>>) {
    let norm_factor = 1.0/buffer.len() as f64;
    let c_n_factor = Complex::new(norm_factor,0.0);
    let mut result =  &(*buffer) * c_n_factor;
    *buffer = result;
  }        
}
pub fn init_vec(v2d: &mut Vector2D<Complex<f64>>, width_x: f64, width_y: f64, nx: usize, ny: usize, f: &dyn Fn(f64) -> f64) {

  let dx:f64 = (2.0*_PI)/(width_x*((nx) as f64));
  let dy:f64 = (2.0*_PI)/(width_y*((ny) as f64));
  for ii in 0..nx {
    for jj in 0..ny {
      let grid_value = Complex::new(f(1.0*dy* (jj as f64)*dx* (ii as f64)),0.0);
      v2d[ii][jj] = grid_value;
    }
  }
}


#[cfg(test)]
mod tests {
  use crate::utils::*;
  use rustfft::{num_complex::Complex};
  use more_asserts as ma;
  #[test]
  // Test sin(X)
  fn test_sin() {
    let width = 1.0;
    let nx: usize = 5;
    let ny: usize = 5;
    let mut v: array_tools::Vector2D<Complex<f64>> = array_tools::Vector2D::<Complex<f64>>::new(nx,ny);
    let mut v2: array_tools::Vector2D<Complex<f64>> = array_tools::Vector2D::<Complex<f64>>::new(nx,ny);
    fft2d_manager::init_vec(&mut v,width,width, nx,ny, &f64::cos);
    fft2d_manager::init_vec(&mut v2,width,width, nx,ny, &f64::cos);

    let fft2d_manager = fft2d_manager::FFT2DManager::new(nx, ny);


    fft2d_manager.fft2d_forward(&mut v);



    for row in 0..nx {
      for col in 0..ny {
        let re_val = v[row][col].re;
        let im_val = v[row][col].im;
        if im_val.abs() > 0.1 || re_val.abs() > 0.1 {
          println!("re:{0}, im:{1} ",re_val, im_val);
          println!("k:{0}, l:{1} ",row, col);
        }

      }
    }  

    fft2d_manager.fft2d_normalized_inverse(&mut v);

    print!("\n");
    for row in 0..nx {
      for col in 0..ny {
        let re_val = v[row][col].re;
        // let im_val = v[row][col].im;
          print!("{} ", re_val);
      }
      print!("\n");

    }
    print!("\n");
    for row in 0..nx {
      for col in 0..ny {
        let val1 = v[row][col];
        let val2 = v2[row][col];
        let diff_real = (val1.re - val2.re).abs();
        let diff_im = (val1.im - val2.im).abs();
        print!("{} ", diff_real);
        ma::assert_le!(diff_real, 1e-3);
        ma::assert_le!(diff_im, 1e-3);
      }
      print!("\n");
    }


  }
}
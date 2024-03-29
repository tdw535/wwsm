use crate::asset_reader::*;
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
    
    // self.init();

    ShallowWaterSolution{u, v, h}
  }

  // fn init(self) -> () {
  //   let file_path: String = "/home/dev/Projects/wwsm/assets/initial_values_test.csv".to_string()
    
  //   let asset_reader = AssetReader::new(file_path);
  //   let init_vals: Vector2D<f64> = match asset_reader.read_initial_values() {
  //     Ok(init_vals)  => init_vals,
  //     Err(e) => panic!("{}", e),
  //   };
  // }
}

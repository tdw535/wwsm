use wasm_bindgen::prelude::*;

use math_utils::asset_reader::*;
use math_utils::utils::array_tools::Vector2D;

#[wasm_bindgen]

pub struct Scene {
  height: Vector2D<f64>,
}

#[wasm_bindgen]
impl Scene {
  pub fn new() -> Scene {
    let file_path: String = "/home/dev/Projects/wwsm/assets/initial_values_test.csv".to_string();
    
    let asset_reader = AssetReader::new(file_path);
    let mut height: Vector2D<f64> = match asset_reader.read_initial_values() {
      Ok(height)  => height,
      Err(e) => panic!("{}", e),
    };

    Scene {height}
  }
}

#[cfg(test)]
mod tests {
  use crate::display_scene::*;
  use more_asserts as ma;
  

  #[test]
  pub fn test_init_vals(){
    let scene = Scene::new();

    let val_as_array = [1.0, 0.0, -1.0, 0.0]; 
    let expected_values: Vec<f64> = val_as_array.to_vec(); 
    print!("Values are");
    for ii in 0..5 {
      for jj in 0..3 {
        let diff:f64 = (expected_values[jj] - scene.height[ii][jj]).abs();
        println!("{}, {}",expected_values[jj], scene.height[ii][jj]);
        ma::assert_le!(diff, 1e-3);
      }
    }

  }

}
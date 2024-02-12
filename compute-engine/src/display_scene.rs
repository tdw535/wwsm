use wasm_bindgen::prelude::*;
use futures::executor::block_on;


use crate::http_handler::HttpHandler;
use math_utils::asset_reader::*;
use math_utils::utils::array_tools::Vector2D;


#[wasm_bindgen]
// #[derive(Clone, Copy)]
pub struct DisplayScene {
  // height: i32,
  // height_accessible: i32
  row: usize,
  col: usize,
  height: Vector2D<f64>,
  height_accessible: Vec<f64> // So that JS can access as array
}

#[wasm_bindgen]
impl DisplayScene {
  pub fn new(row_:i32, col_: i32, array_: &JsValue) -> DisplayScene {
    let row: usize = row_ as usize;
    let col: usize = col_ as usize;

    let mut height: Vector2D<f64> = Vector2D::new(row, col);
    height.set_vec(vec_);
    // let mut height_err = asset_reader.read_initial_values();
    // let mut height_result = asset_reader.read_initial_values();
    // height = height_result?;

    // let mut height = Vector2D::<f64>::new(2,3);
    let mut height_accessible = height.as_vec();
    // let mut height = height;
    // let mut height_accessible = 2*height;


    
    DisplayScene {row, col, height, height_accessible}
  }
  pub fn readin(&mut self) -> Result<(), JsError> {
    let file_path: String = "/home/dev/Projects/wwsm/assets/initial_values_test.csv".to_string();
    
    let asset_reader = AssetReader::new(file_path);

    if let Ok(height_result) = asset_reader.read_initial_values() {
      self.height = height_result;
      self.height_accessible = self.height.as_vec();
    } else {
      // Send error?
      return Err(JsError::new("Error reading in"))
    }    
    Ok(())
  }

  pub fn get_init_val(&mut self) -> bool {
  
  let mut handler = HttpHandler::new("http://localhost:5057/a".to_string());

  //  if let Ok(val) = block_on(handler.test_request()) {
  //   println!("{:?}", val);
  //  } else {
  //   println!("error");
  //  }
   let vec_val = block_on(handler.request_init_val_and_parse_response());
   self.height = vec_val;
   let (row, col) = self.height.get_dim();
   self.row = row;
   self.col = col;

   true
  }

  pub fn get_row(&self) -> usize {
    self.row
    // self.height[0][0]
  }
  pub fn get_col(&self) -> usize {
    self.col
  }
  pub fn height_accessible_js(&mut self) -> *const f64 {
    self.height_accessible = self.height.as_vec();
    self.height_accessible.as_ptr()
  }
}

// #[cfg(test)]
// mod tests {
//   use crate::display_scene::*;
//   use more_asserts as ma;
  

//   #[test]
//   pub fn test_init_vals(){
//     let scene = DisplayScene::new();

//     let val_as_array = [1.0, 0.0, -1.0, 0.0]; 
//     let expected_values: Vec<f64> = val_as_array.to_vec(); 
//     print!("Values are");
//     for ii in 0..5 {
//       for jj in 0..3 {
//         let diff:f64 = (expected_values[jj] - scene.height[ii][jj]).abs();
//         println!("{}, {}",expected_values[jj], scene.height[ii][jj]);
//         ma::assert_le!(diff, 1e-3);
//       }
//     }

//   }

// }
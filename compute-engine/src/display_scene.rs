use wasm_bindgen::prelude::*;
use futures::executor::block_on;


// use crate::http_handler::HttpHandler;
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
  height_next: Vector2D<f64>,
  height_accessible: Vec<f64> // So that JS can access as array
}

#[wasm_bindgen]
impl DisplayScene {
  pub fn new(row_:i32, col_: i32, array_: &[f64]) -> DisplayScene {
    let row: usize = row_ as usize;
    let col: usize = col_ as usize;

    let mut height: Vector2D<f64> = Vector2D::new(row, col);
    height.set_vec(array_.to_vec());
    let mut height_accessible: Vec<f64> = height.as_vec();
    let mut height_next: Vector2D<f64> = Vector2D::new(row, col);
    // let mut height_err = asset_reader.read_initial_values();
    // let mut height_result = asset_reader.read_initial_values();
    // height = height_result?;

    // let mut height = Vector2D::<f64>::new(2,3);
    // let mut height_accessible = height.as_vec();
    // let mut height = height;
    // let mut height_accessible = 2*height;


    
    DisplayScene {row, col, height, height_next, height_accessible}
  }
  // pub fn readin(&mut self) -> Result<(), JsError> {
  //   let file_path: String = "/home/dev/Projects/wwsm/assets/initial_values_test.csv".to_string();
    
  //   let asset_reader = AssetReader::new(file_path);

  //   if let Ok(height_result) = asset_reader.read_initial_values() {
  //     self.height = height_result;
  //     self.height_accessible = self.height.as_vec();
  //   } else {
  //     // Send error?
  //     return Err(JsError::new("Error reading in"))
  //   }    
  //   Ok(())
  // }

  // pub fn get_init_val(&mut self) -> bool {
  
  //   let mut handler = HttpHandler::new("http://localhost:5057/a".to_string());

  //   //  if let Ok(val) = block_on(handler.test_request()) {
  //   //   println!("{:?}", val);
  //   //  } else {
  //   //   println!("error");
  //   //  }
  //   let vec_val = block_on(handler.request_init_val_and_parse_response());
  //   self.height = vec_val;
  //   let (row, col) = self.height.get_dim();
  //   self.row = row;
  //   self.col = col;

  //   true
  // }

  pub fn get_row(&mut self) -> usize {
    self.row.clone()
    // self.height[0][0]
  }
  pub fn get_col(&mut self) -> usize {
    self.col.clone()
  }
  pub fn height_accessible_js(&mut self) -> *const f64 {
    self.height_accessible = self.height.as_vec();
    self.height_accessible.as_ptr()
  }

  // clean up
  pub fn simple_diffusive_update(&mut self) {
    // let w_grid: f64 =  2.0*3.14159;
    // let delrow: f64 = w_grid/(self.row as f64);
    // let delcol: f64 = w_grid/(self.col as f64);
    // let delrowsq: f64 = delrow*delrow;
    // let delcolsq: f64 = delcol*delcol;
    // let delt: f64 = 0.1;
    // let steps: usize = 10;

    // Do boundary conditions later
    // let mut iip: i64 = 0; // ii previous
    // let mut iin: i64 = 0; // ii next
    // let mut jjp: i64 = 0;
    // let mut jjn: i64 = 0;

    for ii in 0..self.row {
      for jj in 0..self.col {
        self.height[ii][jj] = 0.999*self.height[ii][jj];
      }
    }
    // handle boundary later
    // for tt in 0..steps {
    //   for ii in 0..self.row {
    //     iip = (ii as i64) - 1;
    //     iin = (ii as i64) + 1;
    //     if iip == -1 {
    //       iip = (self.row as i64) - 1;
    //     }
    //     if iin == (self.row as i64) - 1 {
    //       iin = 0
    //     }
    //     for jj in 0..self.col {
    //       jjp = (jj as i64) - 1;
    //       jjn = (jj as i64) + 1;          
    //       if jjp == -1 {
    //         jjp = (self.col as i64) - 1;
    //       }
    //       if jjn == (self.col as i64) - 1 {
    //         jjn = 0
    //       }

    //       self.height_next[ii][jj] =  self.height[ii][jj] +
    //         delrowsq/delt*(self.height[iip as usize][jj] + self.height[iin as usize][jj] - 2.0*self.height[ii][jj]) +
    //         delcolsq/delt*(self.height[ii][jjp as usize] + self.height[ii][jjn as usize] -  2.0*self.height[ii][jj])
    //     }
    //   }
    //   for ii in 0..self.row {
    //     for jj in 0..self.col {
    //       self.height[ii][jj] = self.height_next[ii][jj];
    //     }
    //   }
    // }
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
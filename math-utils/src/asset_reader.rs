use crate::utils::array_tools::Vector2D;

use std::error::Error;

use csv::{ReaderBuilder, StringRecord};

pub struct AssetReader {
  file_path: String
}

impl AssetReader {
  pub fn new(file_path: String) ->  AssetReader {
    AssetReader{file_path}
  }
  pub fn read_initial_values(self) -> Result<Vector2D<f64>, Box<dyn Error>> {
    let file_to_read: String = self.file_path.clone();
    // println!("{}", file_to_read);
    // println!("Current dir {}", std::env::current_dir()?.display());

    let mut records: Vec<StringRecord> = Vec::new();

    let mut reader = ReaderBuilder::new().from_path(file_to_read)?;
    for result in reader.records() {
        let record = result?;
        records.push(record);
    }
    let initial_values = self.convert_record_to_vector2d(records);
    Ok(initial_values)
  }

  fn convert_record_to_vector2d(self, records: Vec<StringRecord>) -> Vector2D<f64> {
    let dim1 = records.len();
    // Assert that dim2 > 0
    let dim2 = records[0].len();

    let mut values: Vector2D<f64> = Vector2D::new(dim1, dim2);
    for jj in 0..dim1 {
      for ii in 0..dim2 {
        let value_as_float:f64 = records[jj][ii].parse().unwrap(); 
        values[jj][ii] = value_as_float;
      }
    }
    values
  }
  
}

#[cfg(test)]
mod tests {
  use crate::asset_reader::*;

  use more_asserts as ma;

  #[test]
  fn test_read_initial_values() {
    let asset_reader = AssetReader::new("/home/dev/Projects/wwsm/assets/initial_values_test.csv".to_string());
    let init_vals: Vector2D<f64> = match asset_reader.read_initial_values() {
      Ok(init_vals)  => init_vals,
      Err(e) => panic!("{}", e),
    };


    let val_as_array = [1.0, 0.0, -1.0, 0.0]; 
    let expected_values: Vec<f64> = val_as_array.to_vec(); 
    print!("Values are");
    for ii in 0..5 {
      for jj in 0..3 {
        let diff:f64 = (expected_values[jj] - init_vals[ii][jj]).abs();
        println!("{}, {}",expected_values[jj], init_vals[ii][jj]);
        ma::assert_le!(diff, 1e-3);
      }
    }
    }
}

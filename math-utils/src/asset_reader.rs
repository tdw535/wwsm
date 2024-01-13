// use crate::utils::array_tools::Vector2D;

use std::error::Error;
use csv::{ReaderBuilder, StringRecord};

pub struct AssetReader {
  asset_directory: String
}

impl AssetReader {
  pub fn new(asset_directory: String) ->  AssetReader {
    AssetReader{asset_directory}
  }
  pub fn read_initial_values(self) -> Result<(), Box<dyn Error>> {
    let file_to_read: String = self.asset_directory + "/initial_values.csv";
    println!("{}", file_to_read);
    println!("Current dir {}", std::env::current_dir()?.display());
    let mut reader = ReaderBuilder::new().from_path(file_to_read)?;
    for result in reader.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::asset_reader::*;

  #[test]
  fn test_read_initial_values() {
    let asset_reader = AssetReader::new("./../assets".to_string());
    asset_reader.read_initial_values();
    assert_eq!(true, false);
  }
}

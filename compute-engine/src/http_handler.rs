use reqwest;
use serde::{Deserialize, Serialize};
use serde_json;
use schemars::{schema_for, JsonSchema};


use futures::executor::block_on;
use futures::io;



#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MyStruct {
    pub row: usize,
    pub col: usize,
    pub vals: Vec<f64>
}


pub struct HttpHandler {
    main_address: String
}


// Figure out good way to send request and parse

impl HttpHandler {
    pub fn new(main_address: String) -> HttpHandler {
        HttpHandler {main_address}
    }
    pub async fn test_request(&self) -> reqwest::Result<String> {
  //  pub async fn test_request(&self) -> Result<String, reqwest::Error> {
      // chaining .await will yield our query result
      let client = reqwest::Client::new();
      let address:String = "http://localhost:5057/a".to_string();
  
      let response = client
          .get(address)
          // confirm the request using send()
          .send()
          .await
          // the rest is the same!
          .unwrap()
          .text()
          .await;

      response

    }
    pub async fn test_request_and_parse_response(&self) -> MyStruct {

      let val_result:Result<String, reqwest::Error> = self.test_request().await;
      if let Ok(val) = val_result {
        let val_vec_result:Result<MyStruct, serde_json::Error> = serde_json::from_str(&val);
        if let Ok(val_vec) = val_vec_result {
          println!("{:?}", val_vec.vals);
          return val_vec
    
        } else {
          println!("Parsing error");
        }
       } else {
        println!("error");
       }
      // Do things just like with any other Rust data structure.
      let row:usize = 0;
      let col:usize = 0;
      let vals:Vec<f64> = Vec::new();
      MyStruct{row, col, vals}
    }
}

#[cfg(test)]
mod tests {  
use crate::http_handler::*;
use futures::executor::block_on;

  #[tokio::test]
  async fn get_response() {
    let mut handler = HttpHandler::new("http://localhost:5057/a".to_string());

   if let Ok(val) = handler.test_request().await {
    println!("{:?}", val);
   } else {
    println!("error");
   }
   let vec_val = handler.test_request_and_parse_response().await;
   for ii in 0..vec_val.row {
    for jj in 0..vec_val.col {
      let n = ii*(vec_val.col) + jj;
      print!("{} ",vec_val.vals[n])
    }
    println!("");
   }

    // let val_as_array = [1.0, 0.0, -1.0, 0.0]; 
    // print!("Values are");
    // for ii in 0..5 {
    //   for jj in 0..3 {
    //     let expected_value:f64 = (ii + jj) as f64;
    //     let diff:f64 = (expected_value - scene.height[ii][jj]).abs();
    //     println!("{}, {}",expected_value, scene.height[ii][jj]);
    //     ma::assert_le!(diff, 1e-3);
    //   }
    // }
  }

}
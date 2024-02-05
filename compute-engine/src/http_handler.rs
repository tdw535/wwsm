use reqwest;
use futures::executor::block_on;
use futures::io::Error;



pub struct HttpHandler {
    main_address: String
}

impl HttpHandler {
    pub fn new(main_address: String) -> HttpHandler {
        HttpHandler {main_address}
    }

   pub async fn test_request(&self) -> Result<String, reqwest::Error> {
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
      println!("{:?}", response);
      response
      
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
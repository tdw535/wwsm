mod helper;

use reqwest;
use futures::executor::block_on;


#[tokio::main]
async fn main() {
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
}
// fn main() {
//     let future = hello_world(); // Nothing is printed
//     block_on(future); // `future` is run and "hello, world!" is printed
//     // print!("{}", helper::helper2::get_string());

//     // let address:String = "http://localhost:5057/a".to_string();
//     // let body = reqwest::get(address).await?.text().await?;
//     // println!("body = {:?}", body);

// }




use std::collections::HashMap;
use serde::{Deserialize, Serialize};
#[tokio::test]
async fn get_pets()-> Result<(), Box<dyn std::error::Error>>{
    let resp =  reqwest::
        get("https://httpbin.org/ip")
        // get(format!("https://petstore.swagger.io/v2/pet/findByStatus?status={}", "available"))
        .await?
        .json::<HashMap<String, String>>()
        .await?;

    println!("{resp:#?}");
    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewsJson {
    by: String,
    pub id: u32,
    pub parent: Option<u32>,
    pub text: Option<String>,
    time: u32,
}
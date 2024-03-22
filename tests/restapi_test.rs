use actix_web::{client::Client};
use serde::{Deserialize, Serialize};
#[tokio::test]
async fn get_pets(){
    let json = Client::default()
        .get(format!("https://petstore.swagger.io/v2/pet/findByStatus?status={}", "available"))
        .send()
        .await?
        .json::<NewsJson>()
        .await?;
    Ok(json);
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewsJson {
    by: String,
    pub id: u32,
    pub parent: Option<u32>,
    pub text: Option<String>,
    time: u32,
}
//use std::collections::HashMap;
use serde::{Deserialize};
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, AUTHORIZATION};
use reqwest::Client;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use std::error::Error;
#[derive(Deserialize, Debug)]
struct Ip{
    origin: String
}
#[tokio::test]
async fn get_pets()-> Result<(), Box<dyn std::error::Error>>{
    let resp =  reqwest::
        get("https://httpbin.org/ip")
        // get(format!("https://petstore.swagger.io/v2/pet/findByStatus?status={}", "available"))
        .await?
        .json::<Ip>()
        .await?;

    println!("ip {}", resp.origin);
    Ok(())
}
#[derive(Deserialize, Debug)]
struct Issue {
    id: String,
    key: String,
    summary: String,
    status: String,
}

#[derive(Deserialize, Debug)]
struct Issues {
    issues: Vec<Issue>,
}
#[tokio::test]
async fn jira_issues()-> Result<(), Box<dyn Error>>{
    let url = "https://dayforce.atlassian.net/rest/api/3/search";
    let username = "k.zhang@ceridian.com";
    let password = "[your token]";
    let jql =  "cf[10020] = 'Payroll.INS 24.1.1.S2' and status in ('Open', 'In Progress') and assignee = 'kevin zhang'";

    let auth = format!("{}:{}", username, password);
    let auth = format!("Basic {}", STANDARD.encode(&auth));

    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&auth)?);
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let client = Client::new();
    let res = client.post(url)
        .headers(headers)
        .body(format!(r#"{{"jql":"{}"}}"#, jql))
        .send()
        .await?;
    //let res_json: Issues = res.json().await?;

    let res_body = res.text().await?;
    println!("Response: {:?}", res_body);
    // println!("Response: {:?}", res_json);
    //
    // for issue in res_json.issues{
    //     println!("ID: {}, Summary: {}", issue.id, issue.summary);
    //
    // }

    Ok(())
}
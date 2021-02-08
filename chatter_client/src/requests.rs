use serde_json::json;

use crate::utils;


pub async fn request_new_user(user: utils::NewUserPayload) {
    let client = reqwest::Client::new();

    let res = client.post("http://chatter-server:8088/user/new")
        .header("Content-Type", "application/json")
        .body(json!(user).to_string())
        .send()
        .await;
}


pub async fn test_connection() {

        let res = reqwest::get("http://chatter-server:8088/test").await.unwrap();
        println!("Status: {}", res.status());
        println!("Headers:\n{:#?}", res.headers());

        let body = res.text().await.unwrap();
        println!("Body:\n{}", body);

}

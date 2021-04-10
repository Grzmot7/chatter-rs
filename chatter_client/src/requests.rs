use serde_json::json;
use serde::{ Deserialize, Serialize };
use crate::utils;

#[derive(Serialize, Deserialize)]
struct ChatsPayload {
        success: bool,
        message: Option<Vec<u64>>,
}

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

pub async fn request_login(user: utils::NewUserPayload) -> Result<utils::LoggedUser, String> {
        let client = reqwest::Client::new();
        let res = client.post("http://chatter-server:8088/user/login")
                .header("Content-Type", "application/json")
                .body(json!(user).to_string())
                .send()
                .await;

        let response = match res {
                Ok(r) => r.text().await,
                Err(_) => {
                        let err_m = String::from("Server connection error.");
                        return Err(err_m);
                },
        };


        let payload: utils::LoginPayload = serde_json::from_str(
                &response.unwrap()
                .to_string()
        )
                .expect("Something went wrong serialiing web json.");

        match payload.success {
                true => return { Ok(
                        utils::LoggedUser {
                                id: payload.id,
                                username: payload.username,
                        }
                )},
                false => {
                        let err_m = String::from("Bad username or password");
                        return Err(err_m);
                },
        };
}

pub async fn get_chats(id: u64) -> Result<Vec<u64>, String> {
        let client = reqwest::Client::new();
        let mut err_m = String::new();
        
        let res = client.post("http/://chatter-server:8088/user/chats")
                .header("Content-Type", "application/json")
                .body(json!({
                        "id": id,
                })
                        .to_string()
                )
                .send()
                .await;

        let payload = match res {
                Ok(r) => r.text().await,
                Err(_) => {
                        let err_m = String::from("Server connection error.");
                        return Err(err_m);
                },
        };

        let payload = match payload {
                Ok(r) => r,
                Err(_) => {
                        let err_m = String::from("Error in response body.");
                        return Err(err_m);
                },
        };

        let chats: ChatsPayload = serde_json::from_str(&payload.to_string()).expect("something went wrong serializing web json");

        match chats.success {
                true => {
                        if let Some(p) = chats.message {
                                return Ok(p);
                        } else {
                                let err_m = String::from("You don't have any chats yet, creat a new chat.");
                                return Err(err_m);
                        };
                },
                false => {
                        let err_m = String::from("Error retrieving chat list");
                        return Err(err_m);
                },
        };
}

pub async fn post_message(message: utils::NewMessage) {
        let client = reqwest::Client::new();
        let res = client.post("http//chatter-server:8088/chat/post_message")
                .header("Content_Type", "applications/json")
                .body(json!(
                        {
                        "c_id": message.c_id,
                        "message": message.message,
                        "author": message.username,
                        }
                ).to_string())
                .send()
                .await
                .unwrap();
}
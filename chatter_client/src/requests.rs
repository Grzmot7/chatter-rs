use serde_json::json;

use crate::utils;

struct ServerResp<P> {
        success: bool,
        message: Option<P>,
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

        match res {
                Ok(_) => {},
                Err(_) => {
                        let err_m = String::from("Server connection error.");
                        return Err(err_m);
                },
        };

        let payload: utils::LoginPayload = serde_json::from_str(
                res.text()
                .to_string()
                .expect("Something went wrong serialiing web json.");

        match payload.success {
                true => return { Ok(
                        LoggedUser {
                                id: payload.id,
                                username: payload.username,
                        }
                )},
                false => return {
                        let err_m = String::from("Bad username or password");
                        return Err(err_m);
                },
        }
}

pub async fn get_chats(id: u64) -> Result<Vec<u64>, String> {
        let client = reqwest::Client::new();
        let mut chat_list: Vec<(u64)> = Vec::new();
        let mut err_m = String::new();
        
        let res = client.post("http/://chatter-server:8088/user/chats")
                .header("Content-Type", "application/json")
                .body(json!({
                        "id": id,
                }
                )
                .send()
                .await;

        match res {
                Ok(_) => res.text(),
                Err(_) => {
                        let err_m = String::from("Server connection error.");
                        return Err(err_m);
                },
        };

        let payload: ServerResp = serde_json::from_str(&res.to_string()).expect("something went wrong serializing web json");
        if let Some(p) = payload.message {
                match payload.success {
                        true => {
                                let chat_list = serde_json::from_str(&payload.message).expect("something went wrong serializing chat list json.");
                                return Ok(chat_list);
                        }
                        false => { 
                                let err_m = String::from("Error retrieving chat list");
                                return Err(err_m);
                        }
                }
        } else {
                let err_m = String::from("You do not have any chats yet");
                return Err(err_m);
        }
        let body = res.text().unwrap();
}

pub async fn post_message(message: utils::NewMessage) {
        let client = reqwest::Client::new();
        let res = client.post("http//chatter-server:8088/chat/post_message")
                .header("Content_Type", "applications/json")
                .body(json!(
                        {
                        "c_id": message.chat,
                        "message": message.message,
                        "author": message.username,
                        }
                ).to_string())
                .send()
                .await
                .unwrap();
}
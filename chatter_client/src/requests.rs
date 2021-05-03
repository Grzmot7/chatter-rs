use std::collections::HashMap;
use serde_json::json;
use serde::{ Deserialize, Serialize };
use crate::utils;

#[derive(Serialize, Deserialize)]
struct ChatsPayload {
        success: bool,
        message: HashMap<u64, String>,
}

#[derive(Serialize, Deserialize)]
struct ServResponse {
        message: String,
        success: bool,
}

#[derive(Serialize, Deserialize)]
struct ChatHash {
        success: bool,
        message: HashMap<u64, String>,
}

#[derive(Serialize, Deserialize)]
struct Messages {
        success: bool,
        message: Vec<(String, String)>,
}


pub async fn request_new_user(user: utils::NewUserPayload) -> Result<String, String> {
        let client = reqwest::Client::new();
        let res = client.put("http://chatter-server:8088/user/new")
                .header("Content-Type", "application/json")
                .body(json!(user).to_string())
                .send()
                .await;

        let res = match res {
                Ok(r) => r,
                Err(_) => return Err(String::from("Could not connect to server.")),
        };

        let res = match res.text().await {
                Ok(r) => r,
                Err(_) => return Err(String::from("Server response error.")),
        };

        println!("{}", res);

        let payload: Result<ServResponse, serde_json::Error> = serde_json::from_str(&res.to_string());

        let payload = match payload {
                Ok(p) => p,
                Err(_) => return Err(String::from("Error serializing web json.")),
        };

        match payload.success {
                true => return Ok(payload.message),
                false => return Err(payload.message),
        };
}

pub async fn test_connection() {
        let res = reqwest::get("http://chatter-server:8088/test").await.unwrap();
        
        println!("Status: {}", res.status());
        println!("Headers:\n{:#?}", res.headers());

        let body = res.text().await.unwrap();
        println!("Body:\n{}", body);
}

pub async fn request_login(user: utils::NewUserPayload) -> Result<String, String> {
        let client = reqwest::Client::new();
        let res = client.post("http://chatter-server:8088/user/login")
                .header("Content-Type", "application/json")
                .body(json!(user).to_string())
                .send()
                .await;

        let response = match res {
                Ok(r) => r,
                Err(_) => {
                        let err_m = String::from("Server connection error.");
                        return Err(err_m);
                },
        };

        let resp = response.text().await;

        let resp = match resp {
                Ok(r) => r,
                Err(_) => return Err(String::from("Server response error.")),
        };     
        println!{"{}",resp}
        let payload: ServResponse = serde_json::from_str(resp.as_str()).expect("Error serializing login payload.");


        //let payload = match payload {
        //        Ok(p) => p,
        //        Err(_) => return Err(String::from("Error serializing web json.")),
        //};

        match payload.success {
                true => return Ok(payload.message),
                false => {
                        let err_m = String::from("Bad username or password");
                        return Err(err_m);
                },
        };
}

pub async fn get_chats(id: u64) -> Result<HashMap<u64, String>, String> {
        let client = reqwest::Client::new();
  
        let res = client.post("http://chatter-server:8088/user/chats")
                .header("Content-Type", "application/json")
                .body(json!({
                        "id": id,
                })
                        .to_string()
                )
                .send()
                .await;
        
        let res = match res {
                Ok(r) => r,
                Err(_) => {
                        let err_m = String::from("Server connection error.");
                        return Err(err_m);
                },
        };
        //let response = res.copy();        
        //println!("{}", &res.text().await.unwrap());
        match res.status().as_str() {
                "200" => match res.text().await {
                                Ok(p) => match serde_json::from_str::<ChatHash>(&p) {
                                        Ok(c) => return Ok(c.message),
                                        Err(_) => return Err(String::from("Error serializing web json")),
                                },
                                Err(_) => return Err(String::from("Error in response body.")),
                },
                "400" | "404" => match res.text().await {
                                Ok(r) => match serde_json::from_str::<ServResponse>(&r) {
                                        Ok(s) => return Err(s.message),
                                        Err(_) => return Err(String::from("Error serializing web json.")),
                                },
                                Err(_) => return Err(String::from("Error in response body.")),
                },
                "500" => match res.text().await {
                                Ok(r) => match serde_json::from_str::<ServResponse>(&r) {
                                        Ok(s) => return Err(s.message),
                                        Err(_) => return Err(String::from("Error serializing web json.")),
                                },
                                Err(_) => return Err(String::from("Error in response body.")),
                },
                p => {
                        println!{"{}", p};
                        return Err(String::from("Server response not recognized."));
                },
        };
}

pub async fn put_new_chat(user_1: u64, user_2: String) -> Result<String, String> {
        let client = reqwest::Client::new();
        let res = client.put("http://chatter-server:8088/message/new_chat")
                .header("Content-Type", "application/json")
                .body(json!(
                        {
                        "u_id_1": user_1,
                        "u_name_2": user_2,
                        }
                ).to_string())
                .send()
                .await;
        
        let res = match res {
                Ok(r) => r,
                Err(_) => {
                        let err_m = String::from("Error contacting server");
                        return Err(err_m);
                },
        };

        let payload = match res.text().await {
                Ok(p) => p,
                Err(_) => return Err(String::from("Error extracting response body.")),
        };

        println!("{}", payload);

        let payload: ServResponse = serde_json::from_str(&payload).expect("Error in serializing web response.");

        match payload.success {
                true => return Ok(payload.message),
                false => return Err(payload.message),
        };

}

pub async fn post_message(message: String, author: u64, c_id: u64) {
        let client = reqwest::Client::new();
        let res = client.put("http://chatter-server:8088/message/new")
                .header("Content-Type", "application/json")
                .body(json!(
                        {
                        "c_id": c_id,
                        "message": message,
                        "author": author,
                        }
                ).to_string())
                .send()
                .await
                .unwrap();
}

pub async fn get_messages(c_id: u64) -> Result<Vec<(String, String)>, String> {
        let client = reqwest::Client::new();
        let res = client.post("http://chatter-server:8088/message/get_messages")
                .header("Content-Type", "application/json")
                .body(json!({
                        "id": c_id,
                })
                .to_string())
                .send()
                .await;
        
        let res = match res {
                Ok(r) => r,
                Err(_) => return Err(String::from("Error contacting server.")),
        };


        match res.status().as_str() {
                "200" => match res.text().await {
                                Ok(p) => match serde_json::from_str::<Messages>(&p) {
                                        Ok(c) => return Ok(c.message),
                                        Err(_) => return Err(String::from("Error serializing web json")),
                                },
                                Err(_) => return Err(String::from("Error in response body.")),
                },
                "400" | "404" => match res.text().await {
                                Ok(r) => match serde_json::from_str::<ServResponse>(&r) {
                                        Ok(s) => return Err(s.message),
                                        Err(_) => return Err(String::from("Error serializing web json.")),
                                },
                                Err(_) => return Err(String::from("Error in response body.")),
                },
                "500" => match res.text().await {
                                Ok(r) => match serde_json::from_str::<ServResponse>(&r) {
                                        Ok(s) => return Err(s.message),
                                        Err(_) => return Err(String::from("Error serializing web json.")),
                                },
                                Err(_) => return Err(String::from("Error in response body.")),
                },
                p => {
                        println!{"{}", p};
                        return Err(String::from("Server response not recognized."));
                },
        };
}
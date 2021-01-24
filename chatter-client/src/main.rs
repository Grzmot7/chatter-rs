

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {

    let res = reqwest::get("http://db-functions:8088/test").await?;
    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());

    let body = res.text().await?;
    println!("Body:\n{}", body);
    Ok(())
}

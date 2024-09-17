use actix_web::{get, web, App, HttpServer};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
struct CtToken {
    access_token: String,
}

#[get("/")]
async fn hello(data: web::Data<CtToken>) -> &'static str {
    let client = reqwest::Client::builder().no_gzip().build().unwrap();

    #[derive(Deserialize)]
    struct CtCart {
        id: String,
        version: i32,
    }

    let create_cart_body = json!({
        "currency": "USD",
        "country": "US"
    });

    let response = client
        .post("https://api.us-central1.gcp.commercetools.com/test-marcelo-soria/carts")
        .bearer_auth(data.access_token.clone())
        .body(create_cart_body.to_string())
        .send()
        .await
        .unwrap()
        .json::<CtCart>()
        .await;

    let cart_response_value = response.unwrap();

    let cart_id = cart_response_value.id;
    let cart_version = cart_response_value.version;

    let update_cart_body = json!({
        "version": cart_version,
        "actions": [
            {
                "action": "addLineItem",
                "sku": "RCC-09",

            }
        ]
    });
    let updated_cart_response = client
        .post(format!(
            "https://api.us-central1.gcp.commercetools.com/test-marcelo-soria/carts/{}",
            cart_id,
        ))
        .bearer_auth(data.access_token.clone())
        .body(update_cart_body.to_string())
        .send()
        .await
        .unwrap()
        //.text()
        .json::<CtCart>()
        .await;

    let update_cart_response_value = updated_cart_response.unwrap();

    println!("cart id {:?}", cart_id);
    println!("updatedCart id {:?}", update_cart_response_value.id);

    "OK"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let username = "";
    let password: Option<String> = Some("".to_string());
    let client = reqwest::Client::builder().no_gzip().build().unwrap();
    let response = client
    .post("https://auth.us-central1.gcp.commercetools.com/oauth/token?grant_type=client_credentials&scope=manage_project:test-marcelo-soria")
    .basic_auth(username, password)
    .header("content-length", 0)
    .send()
    .await
    .unwrap()
    .json::<CtToken>()
    .await;

    let access_token = response.unwrap().access_token;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(CtToken {
                access_token: access_token.to_string(),
            }))
            .service(hello)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

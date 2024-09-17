use reqwest::{Client, RequestBuilder};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[macro_use]
extern crate rocket;

use rocket::State;

#[derive(Deserialize)]
struct CtToken {
    access_token: String,
}

#[get("/")]
async fn index(token: &State<CtToken>) -> &'static str {
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
        .bearer_auth(token.access_token.clone())
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
    let updateCartResponse = client
        .post(format!(
            "https://api.us-central1.gcp.commercetools.com/test-marcelo-soria/carts/{}",
            cart_id,
        ))
        .bearer_auth(token.access_token.clone())
        .body(update_cart_body.to_string())
        .send()
        .await
        .unwrap()
        .json::<CtCart>()
        .await;

    let update_cart_response_value = updateCartResponse.unwrap();

    println!("cart id {:?}", cart_id);
    println!("updatedCart id {:?}", update_cart_response_value.id);

    return "OK";
}

#[launch]
async fn rocket() -> _ {
    let username = "BJ_5OQdxbb4K8Y_fpO4yLTSV";
    let password: Option<String> = Some("Bn75YSNN70Yr8fdkPjtMN08v8-CAoQVL".to_string());
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

    println!("Access token {:?}", access_token);

    //rocket::build().mount("/", routes![index])
    rocket::build()
        .manage(CtToken { access_token })
        .mount("/", routes![index])
}

/* #[derive(Deserialize, Serialize)]
struct CreateCartRequest<'a> {
    product_key: &'a str,
}

/* #[derive(Deserialize, Serialize)]
struct Cart {
    id: String,
    // Other cart properties
}
 */
async fn create_cart(product_key: &str) -> Result<String, reqwest::Error> {
    let client = Client::new();
    let url = format!("https://api.commercetools.com/{}", "your_project_key");

    let cart_draft = serde_json::to_value(CreateCartRequest { product_key });

    let body = "{id:23}";

    //    let response = client.post(url.as_str()).body(cart_draft).send().await?;
    let response = client.post(url.as_str()).body(body).send().await?;

    /*     let response = client
        .post(url.as_str())
        .header("Authorization", format!("Bearer {}", "your_project_key"))
        .body(serde_json::to_value(&cart_draft)?)
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(format!("Failed to create cart: {}", response.text().await?));
    } */

    let cart = response.text().await;
    Ok(cart);
    return cart;
    //Ok(cart)
} */

/* #[tokio::main]
fn main() {
    let product_key = "your_product_key";
    let cart = create_cart(product_key).await;

    match cart {
        Ok(cart) => println!("Cart created successfully: {}", cart.id),
        Err(err) => eprintln!("Error creating cart: {}", err),
    }
} */

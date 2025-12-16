use leptos::prelude::*;
#[cfg(feature = "ssr")]
use {tokio::sync, mongodb::{Client, bson::doc}, std::fs, futures::StreamExt, serde::{Deserialize, Serialize}};

// DBの設定
#[cfg(feature = "ssr")]
static DB_CLIENT: sync::OnceCell<Client> = sync::OnceCell::const_new();

#[cfg(feature = "ssr")]
async fn get_db_client() -> Client{
    DB_CLIENT.get_or_init(|| async {
        let path = fs::read_to_string("./passwords/mongoDB_login_url.txt").expect("Couldn't read mongoDB path file");

        Client::with_uri_str(path).await.unwrap()

    }).await.clone()
}


// DBに乗せるレコードを表すstruct


#[cfg(feature = "ssr")]
#[derive(Deserialize, Serialize)]
struct Tag{
    tag: String
}

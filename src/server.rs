use leptos::prelude::*;
#[cfg(feature = "ssr")]
use {
    futures::StreamExt,
    mongodb::{bson::doc, Client, Database},
    serde::{Deserialize, Serialize},
    std::fs,
    tokio::sync,
    jsonwebtoken::{encode, decode, EncodingKey, Header}
};

// DBの設定
#[cfg(feature = "ssr")]
static DB: sync::OnceCell<Database> = sync::OnceCell::const_new();

#[cfg(feature = "ssr")]
async fn get_db() -> Database {
    DB.get_or_init(|| async {
        let path = fs::read_to_string("./passwords/mongoDB_login_url.txt")
            .expect("Couldn't read mongoDB path file");

        Client::with_uri_str(path)
            .await
            .unwrap()
            .database("biestar")
    })
    .await
    .clone()
}

#[cfg(feature = "ssr")]
static DB_SETTING: sync::OnceCell<DbSetting> = sync::OnceCell::const_new();

#[cfg(feature = "ssr")]
#[derive(Serialize, Deserialize, Clone)]
struct DbSetting{
    password_salt: String,
    jwt: String,
}


#[cfg(feature = "ssr")]
async fn get_db_setting() -> DbSetting{
    DB_SETTING.get_or_init(|| async {
        let collection = get_db().await.collection::<DbSetting>("config");
        collection.find_one(doc!{}).await.unwrap().unwrap()
    }).await.clone()
}

// DBに乗せるレコードを表すstruct

#[cfg(feature = "ssr")]
#[derive(Deserialize, Serialize)]
struct Tag {
    tag: String,
}

#[cfg(feature = "ssr")]
#[derive(Deserialize, Serialize)]
struct User {
    name: String,
    password_hash: String,
    icon: Option<String>,
}

// 関数

#[cfg(feature = "ssr")]
static JWT_ENCODE_KEY: sync::OnceCell<EncodingKey> = sync::OnceCell::const_new();

#[cfg(feature = "ssr")]
static JWT_DECODE_KEY: sync::OnceCell<EncodingKey> = sync::OnceCell::const_new(); 

#[cfg(feature = "ssr")]
#[derive(Serialize, Deserialize)]
struct Claims{
    sub: String
}

#[cfg(feature = "ssr")]
async fn make_jwt(name: String) -> String{
    let key = JWT_ENCODE_KEY.get_or_init(|| async {
        let setting = get_db_setting().await;
        EncodingKey::from_secret(setting.jwt.as_bytes())
    }).await;
    encode(&Header::default(), &Claims{sub: name}, key).unwrap()
}

// API関数

#[server]
pub async fn sign_up(name: String, password: String) -> Result<Option<String>, ServerFnError> {
    todo!()
}

#[server]
pub async fn log_in(name: String, password: String) -> Result<Option<String>, ServerFnError> {
    todo!()
}

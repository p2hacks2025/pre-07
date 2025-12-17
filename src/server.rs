use leptos::prelude::*;
#[cfg(feature = "ssr")]
use {
    futures::StreamExt,
    mongodb::{bson::doc, Client, Database},
    serde::{Deserialize, Serialize},
    std::fs,
    tokio::sync,
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
static DB_SETTING: sync::OnceCell<Db_setting> = sync::OnceCell::const_new();

#[cfg(feature = "ssr")]
#[derive(Serialize, Deserialize, Clone)]
struct Db_setting{
    jwt_key: String,
    password_salt: String
}


#[cfg(feature = "ssr")]
async fn get_db_setting() -> Db_setting{
    DB_SETTING.get_or_init(|| async {
        let collection = get_db().await.collection::<Db_setting>("config");
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

// API関数

pub async fn sign_up(name: String, password: String) -> Result<Option<String>, ServerFnError> {
    todo!()
}

pub async fn log_in(name: String, password: String) -> Result<Option<String>, ServerFnError> {
    todo! {}
}

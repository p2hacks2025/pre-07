use leptos::{prelude::*, logging::log};
use serde::{Deserialize, Serialize};
use crate::app::LoginScreenState;

#[cfg(feature = "ssr")]
use {
    argon2::{Argon2, password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng}},
    futures::StreamExt,
    jsonwebtoken::{
        decode, encode, Algorithm::HS256, DecodingKey, EncodingKey, Header, Validation,
    },
    mongodb::{bson::doc, Client, Database},
    std::{fs, sync::LazyLock},
    tokio::sync::OnceCell,
};

// DBの設定
#[cfg(feature = "ssr")]
static DB: OnceCell<Database> = OnceCell::const_new();

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
static DB_SETTING: OnceCell<DbSetting> = OnceCell::const_new();

#[cfg(feature = "ssr")]
#[derive(Serialize, Deserialize, Clone)]
struct DbSetting {
    password_salt: String,
    jwt: String,
}

#[cfg(feature = "ssr")]
async fn get_db_setting() -> DbSetting {
    DB_SETTING
        .get_or_init(|| async {
            let collection = get_db().await.collection::<DbSetting>("config");
            collection.find_one(doc! {}).await.unwrap().unwrap()
        })
        .await
        .clone()
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
    password_salt: String,
    icon: Option<String>,
}

// 関数

#[cfg(feature = "ssr")]
static JWT_ENCODE_KEY: OnceCell<EncodingKey> = OnceCell::const_new();

#[cfg(feature = "ssr")]
static JWT_DECODE_KEY: OnceCell<DecodingKey> = OnceCell::const_new();

#[cfg(feature = "ssr")]
#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
}

#[cfg(feature = "ssr")]
async fn make_jwt(name: String) -> String {
    let key = JWT_ENCODE_KEY
        .get_or_init(|| async {
            let setting = get_db_setting().await;
            EncodingKey::from_secret(setting.jwt.as_bytes())
        })
        .await;
    encode(&Header::default(), &Claims { sub: name }, key).unwrap()
}

#[cfg(feature = "ssr")]
async fn check_jwt(name: String, jwt: String) -> bool {
    let key = JWT_DECODE_KEY
        .get_or_init(|| async {
            let setting = get_db_setting().await;
            DecodingKey::from_secret(setting.jwt.as_bytes())
        })
        .await;
    let d: Result<jsonwebtoken::TokenData<Claims>, _> = decode(jwt, key, &Validation::new(HS256));
    match d {
        Ok(token) => token.claims.sub == name,
        Err(_) => false,
    }
}

#[cfg(feature = "ssr")]
static ARGON2: LazyLock<Argon2<'static>> = LazyLock::new(Argon2::default);

// API関数

#[server]
pub async fn sign_up(name: String, password: String) -> Result<Result<String, LoginScreenState>, ServerFnError> {
    /* 
    戻り値の意図    
    1つ目のResult => サーバーの処理エラー
    2つ目のResult => Signinの処理がうまくいったかどうか
    */
    if password.chars().count() < 8{
        return Ok(Err(LoginScreenState::TooShortPassword));
    }

    let db = get_db().await;
    let user = db.collection::<User>("users");
    if let Err(_) = user.find_one(doc!{"name": &name}).await{
        return Ok(Err(LoginScreenState::NameExists));
    }

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = ARGON2.clone();
    let account = User{name: name.clone(), password_hash: argon2.hash_password(password.as_bytes(), &salt).unwrap().to_string(), password_salt: salt.to_string(), icon: None};
    let _ = user.insert_one(account).await.unwrap();
    Ok(Ok(make_jwt(name).await))
}

#[server]
pub async fn log_in(name: String, password: String) -> Result<Result<String, LoginScreenState>, ServerFnError> {
    todo!()
}

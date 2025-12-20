use crate::app::LoginScreenState;
use leptos::{logging::log, prelude::*};
use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use {
    argon2::{
        password_hash::{
            rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
        },
        Argon2,
    },
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
    icon: Option<String>,
}

#[cfg(feature = "ssr")]
#[derive(Deserialize, Serialize)]
pub struct Post {
    name: String,
    body: String,
    tag: Vec<String>,
    title: String,
    comment: Vec<(String, String)>,
    is_advanced: bool
}

// 関数

#[cfg(feature = "ssr")]
static JWT_ENCODE_KEY: OnceCell<EncodingKey> = OnceCell::const_new();

#[cfg(feature = "ssr")]
static JWT_DECODE_KEY: OnceCell<DecodingKey> = OnceCell::const_new();

#[cfg(feature = "ssr")]
#[derive(Serialize, Deserialize, Debug)]
struct Claims {
    sub: String,
    exp: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PostResult{
    Ok,
    Refuse
}

#[cfg(feature = "ssr")]
async fn make_jwt(name: String) -> String {
    let key = JWT_ENCODE_KEY
        .get_or_init(|| async {
            let setting = get_db_setting().await;
            EncodingKey::from_secret(setting.jwt.as_bytes())
        })
        .await;
    encode(&Header::default(), &Claims { sub: name, exp:1893423600}, key).unwrap() //2030年まで セキュリティ的にはあまりよろしくない
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
    log!("{:?}", d);
    match d {
        Ok(token) => token.claims.sub == name,
        Err(_) => false,
    }
}

#[cfg(feature = "ssr")]
static ARGON2: LazyLock<Argon2<'static>> = LazyLock::new(Argon2::default);

// API関数

#[server]
pub async fn sign_up(
    name: String,
    password: String,
) -> Result<Result<String, LoginScreenState>, ServerFnError> {
    /*
    戻り値の意図
    1つ目のResult => サーバーの処理エラー
    2つ目のResult => Signinの処理がうまくいったかどうか
    */
    if password.chars().count() < 8 {
        return Ok(Err(LoginScreenState::TooShortPassword));
    }

    let db_user = get_db().await.collection::<User>("users");
    if let Err(_) = db_user.find_one(doc! {"name": &name}).await {
        return Ok(Err(LoginScreenState::NameExists));
    }

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = ARGON2.clone();
    let account = User {
        name: name.clone(),
        password_hash: argon2
            .hash_password(password.as_bytes(), &salt)
            .unwrap()
            .to_string(),
        icon: None,
    };
    let _ = db_user.insert_one(account).await.unwrap();
    Ok(Ok(make_jwt(name).await))
}

#[server]
pub async fn log_in(
    name: String,
    password: String,
) -> Result<Result<String, LoginScreenState>, ServerFnError> {
    let db_user = get_db().await.collection::<User>("users");
    if let Some(user) = db_user.find_one(doc! {"name": &name}).await.unwrap() {
        let argon2 = ARGON2.clone();
        if let Ok(_) = argon2.verify_password(
            password.as_bytes(),
            &PasswordHash::new(&user.password_hash).unwrap(),
        ) {
            return Ok(Ok(make_jwt(name).await));
        }
    }
    Ok(Err(LoginScreenState::InvalidAccount))
}

#[server]
pub async fn search_tag_with_exact(tag: String) -> Result<Option<String>, ServerFnError>{
    let db_tag = get_db().await.collection::<Tag>("tags");
    let result = db_tag.find_one(doc!{"tag": tag}).await?.map(|t| t.tag);
    Ok(result)
}

#[server]
pub async fn search_tag_with_prefix(tag: String, amount: i64) -> Result<Vec<String>, ServerFnError>{
    let db_tag = get_db().await.collection::<Tag>("tags");
    let mut result = db_tag.find(doc!{"tag": {"$regex": format!("^{}", tag), "$options": "i"}}).limit(amount).await?;
    let mut out = vec![];
    while let Some(result) = result.next().await{
        out.push(result.unwrap().tag);
    }
    Ok(out)
}

#[server]
pub async fn do_post(name: String, jwt: String, title:String, body: String, tag: Option<Vec<String>>, is_advanced: bool) -> Result<PostResult, ServerFnError>{
    if !check_jwt(name.clone(), jwt).await{
        return Ok(PostResult::Refuse);
    }
    let db_post = get_db().await.collection::<Post>("posts");
    let post = Post{name, body, tag: tag.unwrap(), is_advanced, title, comment: vec![]};
    db_post.insert_one(post).await.unwrap();
    Ok(PostResult::Ok)
}

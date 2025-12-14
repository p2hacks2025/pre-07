use leptos::{prelude::*};
use once_cell::sync::OnceCell;
use std::fs;

static URL_TO_MONGO_DB: OnceCell<String> = OnceCell::new(); 

#[server]
pub async fn test() -> Result<Vec<String>, ServerFnError>{
    URL_TO_MONGO_DB.get_or_init(move || {
        fs::read_to_string("./passwords/mongoDB_login_url.txt").unwrap().trim().to_string()
    });

    Ok(vec![URL_TO_MONGO_DB.get().unwrap().to_string()])
}
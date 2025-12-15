use leptos::prelude::*;

#[server]
pub async fn test() -> Result<String,ServerFnError>{
    Ok("OK".to_string())
}
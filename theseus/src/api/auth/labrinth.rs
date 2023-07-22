//! Authentication flow interface for Labrinth
//! Used to log into a user's Modrinth account and get a token, and authenticate that way
//! See hydra.rs for authentication with a user's Minecraft account

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::config::MODRINTH_API_URL;
use reqwest::Method;
use crate::util::fetch::REQWEST_CLIENT;
use crate::util::fetch::fetch_json;

#[derive(Serialize, Deserialize, Debug)]
struct LabrinthLogin {
    pub username: String,
    pub password: String,
    pub challenge: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LabrinthSession {
    pub id: String,
    pub session: String,
    pub user_id: String,

    pub created: DateTime<Utc>,
    pub last_login: DateTime<Utc>,
    pub expires: DateTime<Utc>,
    pub refresh_expires: DateTime<Utc>,
}




pub async fn login(username : String, password : String) -> crate::Result<Option<LabrinthSession>> {

    let url = format!("{MODRINTH_API_URL}auth/login");
    let login = LabrinthLogin {
        username,
        password,
        challenge: None,
    };
    let session = REQWEST_CLIENT
        .request(Method::POST, url)
        .json(&login)
        .send()
        .await?
        .json::<LabrinthSession>()
        .await?;

    todo!();
    Ok(Some(session))
}

pub async fn logout() -> crate::Result<()> {
    todo!()
}

pub async fn refresh() -> crate::Result<()> {
    todo!()
}

pub async fn get_user() -> crate::Result<()> {
    todo!()
}
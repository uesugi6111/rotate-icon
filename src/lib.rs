use chrono::Timelike;
use chrono::Utc;
use egg_mode::{account::update_profile_image, auth, error::Result};
use once_cell::sync::Lazy;
use serde_json::Value;

static ACCESS_TOKEN: Lazy<Option<String>> = Lazy::new(|| dotenv::var("ACCESS_TOKEN").ok());
static ACCESS_TOKEN_SECRET: Lazy<Option<String>> =
    Lazy::new(|| dotenv::var("ACCESS_TOKEN_SECRET").ok());

static API_KEY: Lazy<Option<String>> = Lazy::new(|| dotenv::var("API_KEY").ok());
static API_KEY_SECRET: Lazy<Option<String>> = Lazy::new(|| dotenv::var("API_KEY_SECRET").ok());

pub async fn run() -> Result<Value> {
    let (_, t) = Utc::now().hour12();
    let file = read_file(std::path::Path::new(&format!("./img/{}.jpg", (t + 9) % 12)));
    let access = auth::Token::Access {
        consumer: auth::KeyPair::new(API_KEY.as_ref().unwrap(), API_KEY_SECRET.as_ref().unwrap()),
        access: auth::KeyPair::new(
            ACCESS_TOKEN.as_ref().unwrap(),
            ACCESS_TOKEN_SECRET.as_ref().unwrap(),
        ),
    };

    let response = update_profile_image(&file, &access).await?;
    println!("{:#?}", response);

    Ok(serde_json::to_value(response.response)?)
}

use std::io::Read;
fn read_file<P: AsRef<std::path::Path>>(file_path: P) -> Vec<u8> {
    let mut file = std::fs::File::open(file_path).expect("file open failed");
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).expect("file read failed");
    buf
}

#[cfg(test)]
mod tests {
    use super::*;
    use egg_mode::{account::UserProfile, auth, error::Result, tweet::DraftTweet};

    fn get_access() -> auth::Token {
        auth::Token::Access {
            consumer: auth::KeyPair::new(
                API_KEY.as_ref().unwrap(),
                API_KEY_SECRET.as_ref().unwrap(),
            ),
            access: auth::KeyPair::new(
                ACCESS_TOKEN.as_ref().unwrap(),
                ACCESS_TOKEN_SECRET.as_ref().unwrap(),
            ),
        }
    }

    #[tokio::test]
    async fn tweet() -> Result<()> {
        let access = get_access();
        let tweet = DraftTweet::new("適当な記事でも生成するか");
        dbg!(&tweet.send(&access).await.unwrap());

        Ok(())
    }

    #[tokio::test]
    async fn show() -> Result<()> {
        let access = get_access();

        dbg!(&egg_mode::user::show("takeda_SE", &access).await.unwrap());
        let user_profile = UserProfile {
            name: None,
            url: Some("https://atcoder.jp/users/uesugi".to_string()),
            location: None,
            description: None,
            profile_link_color: None,
        };
        let user = egg_mode::account::update_profile(user_profile, &access)
            .await
            .unwrap();
        dbg!(user);
        Ok(())
    }

    #[tokio::test]
    async fn update_banner() -> Result<()> {
        let access = get_access();
        let file = read_file(std::path::Path::new("./img/pro.png"));

        let user = egg_mode::account::update_profile_banner(&file, None, &access)
            .await
            .unwrap();
        dbg!(user);
        Ok(())
    }
}

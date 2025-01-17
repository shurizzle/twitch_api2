use twitch_api2::HelixClient;
use twitch_oauth2::{AccessToken, UserToken};

fn main() {
    use std::error::Error;
    if let Err(err) = run() {
        println!("Error: {}", err);
        let mut e: &'_ dyn Error = err.as_ref();
        while let Some(cause) = e.source() {
            println!("Caused by: {:?}", cause);
            e = cause;
        }
    }
}

#[tokio::main]
async fn run() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let _ = dotenv::dotenv();
    let mut args = std::env::args().skip(1);
    let token = std::env::var("TWITCH_TOKEN")
        .ok()
        .or_else(|| args.next())
        .map(AccessToken::new)
        .expect("Please set env: TWITCH_TOKEN or pass token as first argument");
    let token = UserToken::from_existing(
        twitch_oauth2::client::reqwest_http_client,
        token,
        None,
        None,
    )
    .await
    .unwrap();

    let client: HelixClient<'static, reqwest::Client> = HelixClient::new();

    let user = client
        .get_user_from_login(args.next().unwrap(), &token)
        .await?
        .expect("no user found");

    let channel = client
        .get_channel_from_id(&user.id, &token)
        .await?
        .expect("no channel found");

    println!("User information:\n\t{:#?}", user);
    println!("Stream information:\n\t{:#?}", channel);
    Ok(())
}

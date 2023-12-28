use matrix_sdk::{
    Client, config::SyncSettings,
    ruma::{user_id, events::room::message::SyncRoomMessageEvent},
};
use std::env;
use dotenv::dotenv;
use openai_api_rs::v1::api::Client as OtherClient;
use openai_api_rs::v1::chat_completion::{self, ChatCompletionRequest};
use openai_api_rs::v1::common::GPT4;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    // Access environment variables*
    let matrix_password = env::var("MATRIX_PASSWORD")?;
    let gpt_client = OtherClient::new(env::var("OPENAI_API_KEY").unwrap().to_string());

    let alice = user_id!("@ll52_bot:matrix.org");
    let matrix_client = Client::builder().user_id(alice.clone()).build().await?;

    // First we need to log in.
    matrix_client.login_username(alice, &matrix_password).send().await?;

    matrix_client.add_event_handler(|ev: SyncRoomMessageEvent| async move {
        println!("Received a message {:?}", ev);
    });
    
    // Syncing is important to synchronize the client state with the server.
    // This method will never return.
    matrix_client.sync(SyncSettings::default()).await;

    Ok(())
}
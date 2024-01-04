/*
use matrix_sdk::{
    Client, config::SyncSettings,
    ruma::{user_id, events::room::message::SyncRoomMessageEvent},
};
use std::env;
use dotenv::dotenv;
use openai_api_rs::v1::api::Client as OtherClient;
use openai_api_rs::v1::chat_completion::{self, ChatCompletionRequest};
use openai_api_rs::v1::common::GPT4;
use anyhow::Result;

async fn matrix_event1(client: &Client) {
    client.add_event_handler(|ev: SyncRoomMessageEvent| async move {
        //println!("Received a message {:?}", ev);
        println!("add_event_handler worked!");
    });
}

async fn gpt_event1(client: &OtherClient) -> anyhow::Result<()> {

    let req = ChatCompletionRequest::new(
        GPT4.to_string(),
        vec![chat_completion::ChatCompletionMessage {
            role: chat_completion::MessageRole::user,
            content: String::from("What is Bitcoin?"),
            name: None,
            function_call: None,
        }],
    );
    let result = client.chat_completion(req)?;
    println!("{:?}", result.choices[0].message.content);
    println!("Gpt worked!");
    Ok(())
}

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

    matrix_event1(&matrix_client);

    // Syncing is important to synchronize the client state with the server.
    // This method will never return.
    matrix_client.sync(SyncSettings::default()).await;

    gpt_event1(&gpt_client);
    
    Ok(())
}
*/
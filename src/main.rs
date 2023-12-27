use matrix_sdk::{
    Client, config::SyncSettings,
    ruma::{user_id, events::room::message::SyncRoomMessageEvent},
};
use std::env;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    // Access environment variables
    let matrix_password = env::var("MATRIX_PASSWORD")?;

    let alice = user_id!("@ll52_bot:matrix.org");
    let client = Client::builder().user_id(alice.clone()).build().await?;

    // First we need to log in.
    client.login_username(alice, &matrix_password).send().await?;

    client.add_event_handler(|ev: SyncRoomMessageEvent| async move {
        println!("Received a message {:?}", ev);
    });

    // Syncing is important to synchronize the client state with the server.
    // This method will never return.
    client.sync(SyncSettings::default()).await;

    Ok(())
}
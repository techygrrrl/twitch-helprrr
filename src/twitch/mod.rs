use tokio::sync::broadcast;
use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::message::ServerMessage;
use twitch_irc::{ClientConfig, SecureTCPTransport, TwitchIRCClient};

/// This is the function that initializes listening to a Twitch channel
pub async fn initialize_twitch_chat(username: &str, tx: broadcast::Sender<ServerMessage>) {
    println!("Initialize chat for {:?}", username);

    let config = ClientConfig::default();
    let (mut incoming_messages, client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

    let join_handle = tokio::spawn(async move {
        while let Some(message) = incoming_messages.recv().await {
            println!("🦀🟣: {:?}", message);
            let _ = tx.send(message);
        }
    });

    client.join(username.to_owned()).unwrap();

    join_handle.await.unwrap();
}

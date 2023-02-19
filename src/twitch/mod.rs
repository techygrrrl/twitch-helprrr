use twitch_irc::{ClientConfig, SecureTCPTransport, TwitchIRCClient};
use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::message::ServerMessage;

/// This is the function that initializes listening to a Twitch channel
pub async fn initialize_twitch_chat(username: &str) {
    println!("Initialize chat for {:?}", username);

    let config = ClientConfig::default();
    let (mut incoming_messages, client) = TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

    let join_handle = tokio::spawn(async move {
        while let Some(message) = incoming_messages.recv().await {
            println!("🦀🟣: {:?}", message);

            match message {
                ServerMessage::ClearChat(_) => {}
                ServerMessage::ClearMsg(_) => {}
                ServerMessage::GlobalUserState(_) => {}
                ServerMessage::Join(_) => {}
                ServerMessage::Notice(_) => {}
                ServerMessage::Part(_) => {}
                ServerMessage::Ping(_) => {}
                ServerMessage::Pong(_) => {}
                ServerMessage::Privmsg(_) => {
                    // println!("🦀🟣: {:?}", priv_message);
                }
                ServerMessage::Reconnect(_) => {}
                ServerMessage::RoomState(_) => {}
                ServerMessage::UserNotice(_) => {}
                ServerMessage::UserState(_) => {}
                ServerMessage::Whisper(_) => {}
                ServerMessage::Generic(_) => {}

                _ => {}
            }
        }
    });

    client.join(username.to_owned()).unwrap();

    join_handle.await.unwrap();
}
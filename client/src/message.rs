use std::{io, sync::Arc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::net::UdpSocket;
use bevy::prelude::Resource;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MessageType {
    Connect,
    Disconnect,
    Action,
    ConnectSuccessfull,
    ConnectFailed,
    UpdateLife,
    BallMovement,
    PlayerDeath
}

#[derive(Resource, Debug, Clone)]
pub struct UdpSocketResource {
    pub socket: Arc<UdpSocket>,
    pub username: String,
    pub id: usize
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageRecieve {
    pub message_type: MessageType,
    pub player_name: String,
    pub content: Value, 
    pub id_player: usize,
    pub player_life: i64,
    pub level: i32, 
    pub canconnect: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageSended {
    pub message_type: MessageType,
    pub player_name: String,
    pub content: Value, 
    pub id_player: usize,

}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageInfo{
    pub message_type: MessageType,
    pub content: Value, 
}

pub async fn send_message(socket: &Arc<UdpSocket>, typ: MessageType, username: String, content: Value, id_player: usize) -> io::Result<()> {
    let connect_msg = MessageSended {
        message_type: typ,
        player_name: username,
        content: content.clone(),
        id_player,
    };
    let message_data = serde_json::to_vec(&connect_msg)?;
    socket.send(&message_data).await?;  

    Ok(())
}

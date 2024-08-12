use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MessageType {
   Connect,
   ConnectSuccessfull,
   ConnectFailed,
   Disconnect,
   Action,
   UpdateLife,
   PlayerDeath
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageSended {
   pub message_type: MessageType,
   pub player_name: String,
   pub content: Value,
   pub id_player: usize,
   pub player_life: i64,
   pub level: i32, 
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageRecieve {
   pub message_type: MessageType,
   pub player_name: String,
   pub content: Value,
   pub id_player: usize,
}
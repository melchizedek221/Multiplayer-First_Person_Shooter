use local_ip_address::local_ip;
use serde_json::{Error, Value};
use std::collections::HashMap;
use std::io::Write;
use std::sync::Arc;
use tokio::net::UdpSocket;
use tokio::sync::{mpsc, Mutex};

use server::*;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Demander à l'utilisateur d'entrer un nombre
    print!("Please enter the game level: ");
    std::io::stdout().flush()?;
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let input = input.trim().to_string();
    // Convertir l'entrée en entier i32
    let number: i32 = match input.parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Invalid entry. Please enter an i32 integer.");
            return Ok(());
        }
    };
    if number < 1 || number > 3 {
        eprintln!("Number must be between 1 and 3.");
        return Ok(());
    }
    println!("Starting server...");

    let port: &str = "8081";
    let mut can = true;
    let ip = match local_ip() {
        Ok(ip) => ip,
        Err(e) => {
            eprintln!("Failed to get local IP address: {}", e);
            return Ok(());
        }
    };

    let socket_addr = format!("{}:{}", ip, port);

    // Bind the UDP socket
    let socket: Arc<UdpSocket> = Arc::new(UdpSocket::bind(socket_addr).await?);

    println!("Server listening on {}:{}", &ip, &port);

    let (tx, mut rx) = mpsc::channel(32);

    let usernames = Arc::new(Mutex::new(HashMap::new()));

    // Task to receive messages and send them through the channel
    {
        let socket = socket.clone();
        let tx = tx.clone();
        tokio::spawn(async move {
            let mut buf = vec![0; 1024];
            loop {
                let (amt, src) = match socket.recv_from(&mut buf).await {
                    Ok((amt, src)) => (amt, src),
                    Err(e) => {
                        eprintln!("Failed to receive data: {}", e);
                        continue;
                    }
                };

                let msg: Result<MessageRecieve, Error> = serde_json::from_slice(&buf[..amt]);
                match msg {
                    Ok(message) => {
                        if let Err(e) = tx.send((message, src)).await {
                            eprintln!("Failed to send message to channel: {}", e);
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to deserialize message: {}", e);
                    }
                }
            }
        });
    }

    let mut id: usize = 0;

    // Task to process messages from the channel
    while let Some((message, src)) = rx.recv().await {
        println!("Received: {:?}", message);
        // Handle the message based on its type
        match message.message_type {
            MessageType::Connect => {
                let mut usernames = usernames.lock().await;

                if usernames.contains_key(&message.player_name) {
                    // Username is taken, send a response to choose another one
                    // println!("EXISTùùù$$$$$$");
                    let response = MessageSended {
                        message_type: MessageType::ConnectFailed,
                        player_name: message.player_name,
                        content: Value::String(
                            "Username already taken. Please choose another one.".to_string(),
                        ),
                        id_player: id.clone(),
                        player_life: 0,
                        level: number,
                        canconnect: false,
                        // Add other fields here if needed
                    };

                    let response_data = serde_json::to_vec(&response).unwrap();
                    if let Err(e) = socket.send_to(&response_data, src).await {
                        eprintln!("Failed to send response: {}", e);
                    }
                } else {
                    // Username is available, add to the map and acknowledge connection
                    let player = Player {
                        player_name: message.player_name.clone(),
                        ip_address: src.to_string(),
                        id: id,
                        life: 20,
                    };
                    let life_player = player.life;

                    can = usernames.len()< 9;

                    
                    usernames.insert(message.player_name.clone(), player);
                   
                    
                    println!("usernames added to map: {:?}", &usernames);
                    let response = MessageSended {
                        message_type: MessageType::ConnectSuccessfull,
                        player_name: message.player_name,
                        content: Value::String("Connected successfully".to_string()),
                        id_player: id.clone(),
                        player_life: life_player,
                        level: number,
                        canconnect: can,
                        // Add other fields here if needed
                    };

                    let response_data = serde_json::to_vec(&response).unwrap();
                    if let Err(e) = socket.send_to(&response_data, src).await {
                        eprintln!("Failed to send response: {}", e);
                    }

                    id += 1;
                }
            }

            MessageType::Disconnect => {
                println!("player {} disconnected", &message.player_name);
                let mut usernames = usernames.lock().await;
                usernames.remove(&message.player_name);
                // Handle disconnection
            }
            MessageType::Action => {
                // Handle player action

                // println!("player position {:?}", &message);
                let clients = usernames.lock().await;
                let mut life_player_to_move = 0;
                for player in clients.values() {
                    if player.player_name == message.player_name {
                        life_player_to_move = player.life
                    }
                }
                for player in clients.values() {
                    if player.player_name != message.player_name {
                        // println!("quelque chose: {:?}", &player);
                        let response = MessageSended {
                            message_type: MessageType::Action,
                            player_name: message.player_name.clone(),
                            content: message.content.clone(),
                            id_player: message.id_player,
                            player_life: life_player_to_move,
                            level: number,
                            canconnect: can,
                        };

                        // println!("player move {:?}", response);
                        println!("content message {}", message.content.clone());

                        let response_data = serde_json::to_vec(&response).unwrap();
                        if let Err(e) = socket
                            .send_to(&response_data, player.ip_address.clone())
                            .await
                        {
                            eprintln!("Failed to send response: {}", e);
                        }
                    }
                }
            }
            MessageType::UpdateLife => {
                // println!("enter");
                let mut clients = usernames.lock().await;
                for player in clients.values_mut() {
                    if player.id == message.id_player {
                        if player.life <= 0 {
                            let response = MessageSended {
                                message_type: MessageType::PlayerDeath,
                                player_name: message.player_name.clone(),
                                content: message.content.clone(),
                                id_player: message.id_player,
                                player_life: player.life,
                                level: number,
                                canconnect: can,
                            };

                            // println!("player move {:?}", response);

                            let response_data = serde_json::to_vec(&response).unwrap();
                            if let Err(e) = socket
                                .send_to(&response_data, player.ip_address.clone())
                                .await
                            {
                                eprintln!("Failed to send response: {}", e);
                            }
                        } else {
                            player.life -= 1;
                            println!("player death {:?}", player);
                        }
                    }
                }
            }

            _ => {
                unimplemented!()
            }
        }
    }

    Ok(())
}

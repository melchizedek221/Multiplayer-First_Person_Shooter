mod arms;
mod components;
mod labyrinte;
mod message;
mod players;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;

use serde_json::{Error, Value};
use std::io::{self, Write};
use std::sync::Arc;
use tokio::net::UdpSocket;
use tokio::signal;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Receiver;

use crate::arms::*;
use crate::components::*;
use crate::labyrinte::*;
use crate::message::*;
use crate::players::*;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

// Un resource pour stocker le receveur de messages
#[derive(Resource, Debug)]
struct MessageReceiver(Receiver<MessageRecieve>);

#[derive(Event, Debug)]
struct ServerMessageReceived(MessageRecieve);

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
enum GameSet {
    NetworkInput,
    PlayerInput,
    Movement,
    Collision,
    UI,
    NetworkOutput,
}

#[tokio::main]
async fn main() -> io::Result<()> {
    print!("Enter Server IP Address (e.g., 11.11.90.13:1234): ");
    io::stdout().flush()?;
    let mut server_ip = String::new();
    io::stdin().read_line(&mut server_ip)?;
    let server_ip = server_ip.trim();

    print!("Enter Your Name: ");
    io::stdout().flush()?;
    let mut username = String::new();
    io::stdin().read_line(&mut username)?;
    let username = username.trim().to_string();

    println!("Data sent to server. Waiting for response...");
    // Créez un socket UDP pour le client
    let socket = Arc::new(UdpSocket::bind("0.0.0.0:0").await?);

    let _ = socket.connect(server_ip).await;

    let udp_socket_resource = UdpSocketResource {
        socket: socket.clone(),
        username: username.clone(),
        id: 0,
    };

    send_message(
        &socket,
        MessageType::Connect,
        username.clone(),
        Value::Null,
        0,
    )
    .await?;

    // Handle Ctrl+C
    let socket_clone = socket.clone();
    let username_clone = username.to_string();
    tokio::spawn(async move {
        signal::ctrl_c().await.unwrap();
        send_message(
            &socket_clone.clone(),
            MessageType::Disconnect,
            username_clone,
            Value::Null,
            0,
        )
        .await
        .unwrap();
        println!("Disconnect message sent to server.");
        std::process::exit(1);
    });

    
    let (tx, rx) = mpsc::channel(32);

    // Tâche pour recevoir des messages
    let socket_clone = socket.clone();
    tokio::spawn(async move {
        let mut buf = vec![0; 1024];
        loop {
            match socket_clone.recv_from(&mut buf).await {
                Ok((amt, _src)) => {
                    let response: Result<MessageRecieve, Error> = serde_json::from_slice(&buf[..amt]);
                    if let Ok(message) = response {
                        if let Err(e) = tx.send(message).await {
                            eprintln!("Failed to send message to channel: {}", e);
                        }
                    } else {
                        eprintln!("Failed to deserialize response");
                    }
                }
                Err(e) => {
                    eprintln!("Failed to receive data: {}", e);
                }
            }
        }
    });

    // Exécuter Bevy sur le thread principal
    App::new()
        .add_plugins((DefaultPlugins, RapierPhysicsPlugin::<NoUserData>::default()))
        .init_resource::<MinimapEntities>()
        .init_resource::<OtherPlayersMap>()
        .insert_resource(PlayerState { is_dead: false })
        .init_resource::<OtherBallMap>()
        .add_event::<ServerMessageReceived>()
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 0.6,
        })
        .insert_resource(RapierConfiguration {
            gravity: Vec3::ZERO,
            ..default()
        })
        .insert_resource(MessageReceiver(rx))
        .insert_resource(udp_socket_resource)
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_systems(Startup, setup)
        .configure_sets(Update, (
            GameSet::NetworkInput,
            GameSet::PlayerInput,
            GameSet::Movement,
            GameSet::Collision,
            GameSet::UI,
            GameSet::NetworkOutput,
        ).chain())
        .add_systems(Update, handle_server_messages.in_set(GameSet::NetworkInput))
        .add_systems(Update, (
            player_movement_and_rotation,
            shoot_ball,
            fps_counter_showhide,
            check_player_death
        ).in_set(GameSet::PlayerInput))
        .add_systems(Update, move_balls.in_set(GameSet::Movement))
        .add_systems(Update, check_ball_player_collisions.in_set(GameSet::Collision))
        // .add_systems(Update, handle_player_life_update.in_set(GameSet::UpdateLife))
        .add_systems(Update, (
            update_minimap_player,
            fps_text_update_system,
        ).in_set(GameSet::UI))
        .add_systems(Update, react_to_server_messages.in_set(GameSet::NetworkOutput))
        .add_systems(Update, react_to_server_ball.in_set(GameSet::NetworkOutput))

        .run();

    Ok(())
}

// Système Bevy pour traiter les messages du serveur
fn handle_server_messages(
    mut commands: Commands,
    mut message_receiver: ResMut<MessageReceiver>,
    mut udp_socket_resource: ResMut<UdpSocketResource>,
    mut server_message_events: EventWriter<ServerMessageReceived>,
    mut player_state: ResMut<PlayerState>,
) {
    while let Ok(message) = message_receiver.0.try_recv() {
        info!("Message from server: {:?}", message);
        match message.message_type {
            MessageType::ConnectSuccessfull => {
                println!(
                    "Server response: {:?}",
                    message
                );

                udp_socket_resource.id = message.id_player;
                // Mettez à jour d'autres états du jeu si nécessaire
                commands.insert_resource(udp_socket_resource.clone());
            }
            MessageType::Action => {
                server_message_events.send(ServerMessageReceived(message.clone()));
            },
            MessageType::PlayerDeath => {
                player_state.is_dead = true;
            }
            
            _ => {
                println!("Unhandled message type: {:?}", message.message_type);
            }
        }
    }
}

fn check_player_death(
    player_state: Res<PlayerState>,
    mut exit: EventWriter<bevy::app::AppExit>,
) {
    if player_state.is_dead {
        println!("Player is dead, disconnecting...");
        // Quitter l'application
        exit.send(bevy::app::AppExit);
    }
}
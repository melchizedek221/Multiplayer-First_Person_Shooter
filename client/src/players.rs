use bevy::prelude::*;
use bevy_rapier3d::prelude::Velocity;
use serde_json::json;
use tokio::runtime::Runtime;
use crate::components::*;
use crate::message::{MessageType, UdpSocketResource, send_message};
#[derive(Component)]
pub struct LastSentTransform {
    pub translation: Vec3,
    pub rotation: Quat,
}

lazy_static::lazy_static! {
    static ref TOKIO_RUNTIME: Runtime = Runtime::new().unwrap();
}


pub fn player_movement_and_rotation(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Velocity, &mut Transform, &mut LastSentTransform, &Player)>,
    udp_socket_res: Res<UdpSocketResource>,
) {
    // Destructure and clone necessary components
    let (mut velocity, mut transform, mut last_sent, player) = query.single_mut();
    let player_id = player.id;
    // let player_life = player.life;

    let mut movement = Vec3::ZERO;
    let mut rotation = 0.0;

    // Calculate forward vector
    let forward = Vec3::new(transform.forward().x, 0.0, transform.forward().z).normalize();
    if keyboard_input.pressed(KeyCode::Up) { movement += forward; }
    if keyboard_input.pressed(KeyCode::Down) { movement -= forward; }
    if keyboard_input.pressed(KeyCode::Left) { rotation += 1.3; }
    if keyboard_input.pressed(KeyCode::Right) { rotation -= 1.3; }

    // Apply movement
    if movement.length() > 0.0 {
        movement = movement.normalize();
    }
    let speed = 3.0;
    velocity.linvel = movement * speed;

    // Apply rotation
    if rotation != 0.0 {
        let rotation_speed = 1.0;
        transform.rotate_y(rotation * rotation_speed * time.delta_seconds());
    }

    // Check if the player has moved or rotated significantly
    let translation_threshold = 0.1; // Adjust this value as needed
    let rotation_threshold = 0.1; // Adjust this value as needed
    let translation_changed = (transform.translation - last_sent.translation).length_squared() > translation_threshold * translation_threshold;
    let rotation_changed = transform.rotation.angle_between(last_sent.rotation) > rotation_threshold;

    if translation_changed || rotation_changed {
        // Clone data to move into the async block
        let translation = transform.translation;
        let rotation = transform.rotation;
        let socket = udp_socket_res.socket.clone();
        let username = udp_socket_res.username.clone();

        // Update LastSentTransform
        last_sent.translation = translation;
        last_sent.rotation = rotation;

        TOKIO_RUNTIME.spawn(async move {
            let content = json!({
                "movement": translation,
                "rotation": rotation,
            });

            if let Err(e) = send_message(&socket, MessageType::Action, username, content, player_id).await {
                eprintln!("Échec de l'envoi du message : {}", e);
                println!("The server is unavaible");
            std::process::exit(1); 
            }
        });
    }
}


pub fn update_minimap_player(
    maze: Res<Maze>,
    minimap_entities: Res<MinimapEntities>,
    player_query: Query<&Transform, With<Player>>,
    mut minimap_player_query: Query<&mut Style, With<MinimapPlayer>>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        if let Some(minimap_player) = minimap_entities.player {
            if let Ok(mut style) = minimap_player_query.get_mut(minimap_player) {
                let window_width = 800.0;
                let minimap_size = window_width * 0.3;
                let tile_size = minimap_size / maze.layout[0].len() as f32;
                let x = player_transform.translation.x / maze.cell_size;
                let z = player_transform.translation.z / maze.cell_size;
                style.left = Val::Px(x * tile_size);
                style.top = Val::Px(z * tile_size);
            }
        }
    }
}

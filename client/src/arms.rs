use bevy::prelude::*;
use serde_json::Value;
use tokio::runtime::Runtime;
use crate::{components::*, send_message, MessageType, UdpSocketResource};
use serde_json::json;
// use crate::components::*;
// use tokio::runtime::Runtime;
// use crate::message::{MessageType, UdpSocketResource, send_message};


lazy_static::lazy_static! {
    static ref TOKIO_RUNTIME: Runtime = Runtime::new().unwrap();
}
// #[derive(Component)]
// pub struct LastBallTransform {
//     pub translation: Vec3,
// }
// Nouveau système pour lancer des balles
pub fn shoot_ball(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    player_query: Query<&Transform, With<Player>>,
    mut meshes: ResMut<Assets<Mesh>>, // Récupérer les ressources Mesh
    mut materials: ResMut<Assets<StandardMaterial>>, // Récupérer les ressources Material
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if let Ok(player_transform) = player_query.get_single() {
            let ball_speed = 20.0; // Vitesse de la balle
            let ball_direction = player_transform.forward().normalize();
            let ball_spawn_position = player_transform.translation + ball_direction * 2.0; // Lancer la balle devant le joueur
            commands.spawn((
                PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::UVSphere { radius: 0.1, ..default() })), // Utiliser UVSphere
                    material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()), // Couleur jaune pour la balle
                    transform: Transform::from_translation(ball_spawn_position),
                    ..default()
                },
                Ball,
            ))
            .insert(Velo(ball_direction * ball_speed)); // Ajouter une vélocité à la balle
        }
    }
}
       
// Système pour déplacer les balles
pub fn move_balls(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &Velo)>,
    udp_socket_res: Res<UdpSocketResource>,

) {
    for (entity, mut transform, velocity) in query.iter_mut() {
        transform.translation += velocity.0 * time.delta_seconds(); // Déplacer la balle
        // Vérifier si la balle est hors de l'écran ou entre en collision avec un mur
        let mut content = json!({
            "ball_movement": transform.translation,
        });
        let socket = udp_socket_res.socket.clone();
        let username = udp_socket_res.username.clone();
        
        if transform.translation.y < -50.0 || transform.translation.y > 50.0 || transform.translation.x < -50.0 || transform.translation.x > 50.0 {
            content = json!({
                "delete_ball": true,
            });
            commands.entity(entity).despawn(); // Détruire la balle si elle est hors de l'écran
        }
        
        TOKIO_RUNTIME.spawn(async move {
            if let Err(e) = send_message(&socket, MessageType::Action, username, content, 0).await {
                eprintln!("Échec de l'envoi du message : {}", e);
            }
        });
    }
   
}


// pub fn check_ball_player_collisions(
//     ball_query: Query<(&Transform, Entity), With<Ball>>,
//     player_query: Query<(&Transform, &mut OtherPlayer)>,
//     mut health_query: Query<&mut Health>,
//     mut commands: Commands,
//     udp_socket_res: Res<UdpSocketResource>,
//     // mut event_writer: EventWriter<BallCollisionEvent>,
// ) {
//     for (ball_transform, entity_ball) in ball_query.iter() {
//         for( player_transform, other_player) in player_query.iter() {
//             //let (player_position, entity) =  
//             let distance = ball_transform.translation.distance(player_transform.translation);
//             if distance < 1.0 {
//                 // println!("Distaaaaaance = {:?}", distance);
//                 // for mut health_player in health_query.iter_mut(){
//                 //     health_player.life -= 1;
//                 //     println!("Life = {:?}", health_player);
//                 // }
//                 let id = other_player.id.clone();

//                 let content = json!({
//                     "delete_ball": true,
//                 });
//                 let socket = udp_socket_res.socket.clone();
//                 let username = udp_socket_res.username.clone();
                
//                 TOKIO_RUNTIME.spawn(async move {
//                     if let Err(e) = send_message(&socket, MessageType::Action, username, content, 0).await {
//                         eprintln!("Échec de l'envoi du message : {}", e);
//                     }
//                 });
//                 commands.entity(entity_ball).despawn();
             
//                 }
//         }
//     }
// }


pub fn check_ball_player_collisions(
    ball_query: Query<(&Transform, Entity), With<Ball>>,
    mut player_query: Query<(&Transform, &mut OtherPlayer)>,
    mut commands: Commands,
    udp_socket_res: Res<UdpSocketResource>,

) {
    // println!("message receiver {:?}", message_receiver);
    for (ball_transform, entity_ball) in ball_query.iter() {
        for (player_transform, other_player) in player_query.iter_mut() {
            let distance = ball_transform.translation.distance(player_transform.translation);
            if distance < 1.0 {
                let id = other_player.id.clone();
                let socket = udp_socket_res.socket.clone();
                TOKIO_RUNTIME.spawn(async move {
                    if let Err(e) =  send_message(&socket, MessageType::UpdateLife, "".to_string(), Value::Null, id).await{
                        eprintln!("Échec de l'envoi du message : {}", e);
                    }
                });

                let content = json!({
                    "delete_ball": true,
                });
                let username = udp_socket_res.username.clone();
                let socket = udp_socket_res.socket.clone();

                TOKIO_RUNTIME.spawn(async move {
                    if let Err(e) = send_message(&socket, MessageType::Action, username, content, 0).await {
                        eprintln!("Échec de l'envoi du message : {}", e);
                    }
                });
                commands.entity(entity_ball).despawn();
            }
        }
    }
}

use crate::{
    components::*,
    message::MessageType,
    LastSentTransform, MessageReceiver, ServerMessageReceived,
};
use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use bevy_rapier3d::prelude::*;

pub fn create_minimap(
    commands: &mut Commands,
    minimap_size: f32,
    tile_size: f32,
    maze_layout: &Vec<Vec<u8>>,
    mut minimap_entities: ResMut<MinimapEntities>,
    player_entity: Entity,
) {
    let minimap_entity = commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Px(minimap_size),
                    height: Val::Px(minimap_size),
                    position_type: PositionType::Absolute,
                    left: Val::Px(10.0),
                    bottom: Val::Px(10.0),
                    ..default()
                },
                background_color: Color::rgba(0.1, 0.1, 0.1, 0.8).into(),
                ..default()
            },
            Minimap,
        ))
        .with_children(|parent| {
            for (i, row) in maze_layout.iter().enumerate() {
                for (j, &cell) in row.iter().enumerate() {
                    let entity = parent
                        .spawn((
                            NodeBundle {
                                style: Style {
                                    width: Val::Px(tile_size),
                                    height: Val::Px(tile_size),
                                    position_type: PositionType::Absolute,
                                    left: Val::Px(j as f32 * tile_size),
                                    top: Val::Px(i as f32 * tile_size),
                                    ..default()
                                },
                                background_color: if cell == 1 {
                                    Color::GRAY
                                } else {
                                    Color::BLACK
                                }
                                .into(),
                                ..default()
                            },
                            MinimapTile,
                        ))
                        .id();
                    minimap_entities.tiles.push(entity);
                }
            }

            // Spawn player marker in minimap
            let player_marker = parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Px(tile_size),
                            height: Val::Px(tile_size),
                            position_type: PositionType::Absolute,
                            ..default()
                        },
                        background_color: Color::RED.into(),
                        ..default()
                    },
                    MinimapPlayer,
                    MinimapPlayerMarker(player_entity),
                ))
                .id();
            minimap_entities.player = Some(player_marker);
        })
        .id();
    minimap_entities.tiles.push(minimap_entity);
}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    minimap_entities: ResMut<MinimapEntities>,
    mut message_receiver: ResMut<MessageReceiver>,
) {
    let mut maze_layout = vec![
        vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        vec![1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        vec![1, 0, 0, 0, 0, 1, 1, 0, 1, 1, 0, 1, 0, 1],
        vec![1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        vec![1, 0, 0, 1, 0, 1, 1, 0, 1, 1, 0, 1, 0, 1],
        vec![1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        vec![1, 0, 0, 1, 0, 1, 1, 0, 1, 1, 0, 1, 0, 1],
        vec![1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        vec![1, 0, 0, 1, 0, 1, 1, 0, 1, 1, 0, 1, 0, 1],
        vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        vec![1, 0, 0, 0, 0, 1, 1, 0, 1, 1, 0, 1, 0, 1],
        vec![1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        vec![1, 0, 0, 0, 0, 1, 1, 0, 1, 1, 0, 1, 0, 1],
        vec![1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    ];
    let maze_layout_v1 = vec![
        vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        vec![1, 0, 0, 0, 1, 1, 1, 0, 1, 0, 0, 0, 0, 1],
        vec![1, 1, 1, 0, 1, 0, 0, 0, 1, 0, 1, 1, 0, 1],
        vec![1, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1],
        vec![1, 0, 1, 1, 1, 0, 1, 0, 0, 0, 0, 1, 0, 1],
        vec![1, 0, 1, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 1],
        vec![1, 1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1],
        vec![1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 1],
        vec![1, 0, 1, 0, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1],
        vec![1, 0, 0, 1, 0, 1, 0, 0, 1, 1, 0, 0, 0, 1],
        vec![1, 0, 1, 0, 0, 0, 1, 1, 0, 1, 0, 1, 0, 1],
        vec![1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        vec![1, 0, 1, 0, 0, 0, 1, 0, 1, 1, 1, 1, 0, 1],
        vec![1, 1, 0, 0, 0, 1, 1, 0, 1, 0, 0, 0, 0, 1],
        vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    ];
    let maze_layout_v2 = vec![
        vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        vec![1, 0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 1],
        vec![1, 1, 0, 1, 1, 1, 0, 1, 1, 0, 0, 1, 0, 1],
        vec![1, 0, 0, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 1],
        vec![1, 1, 0, 0, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1],
        vec![1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 1],
        vec![1, 1, 1, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0, 1],
        vec![1, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 1],
        vec![1, 0, 1, 0, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1],
        vec![1, 0, 0, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 1],
        vec![1, 0, 1, 0, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1],
        vec![1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 1],
        vec![1, 0, 1, 0, 0, 0, 1, 0, 1, 0, 1, 0, 0, 1],
        vec![1, 0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1],
        vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    ];
    let start_positions = vec![
        //Lab1
        Vec3::new(2.0, 1.0, 1.5),    //1
        Vec3::new(24.50, 1.0, 26.5), // 2
        Vec3::new(2.14, 1.0, 26.5),  // 3
        Vec3::new(24.50, 1.0, 1.5),  // 4
        Vec3::new(8.0, 1.0, 10.),    // 5
        Vec3::new(8.12, 1.0, 16.),   // 6
        Vec3::new(20., 1.0, 16.),    // 7
        Vec3::new(20.25, 1.0, 8.),   // 8
        Vec3::new(14., 1.0, 9.),     // 9
        Vec3::new(14.25, 1.0, 18.),  // 10
        //Lab2
        Vec3::new(2.0, 1.0, 1.5),    //1
        Vec3::new(24.50, 1.0, 26.5), // 2
        Vec3::new(1.5, 1.0, 24.),  // 3
        Vec3::new(24.50, 1.0, 1.5),  // 4
        Vec3::new(10.0, 1.0, 6.),    // 5
        Vec3::new(8., 1.0, 16.),   // 6
        Vec3::new(16., 1.0, 22.),    // 7
        Vec3::new(18.17, 1.0, 14.27),   // 8
        Vec3::new(19.5, 1.0, 7.5),     // 9
        Vec3::new(14.50, 1.0, 3.4),  // 10
        //Lab3
        Vec3::new(2.0, 1.0, 1.5),    //1
        Vec3::new(24.50, 1.0, 26.5), // 2
        Vec3::new(2.14, 1.0, 26.5),  // 3
        Vec3::new(24.50, 1.0, 1.5),  // 4
        Vec3::new(7.14, 1.0, 8.36),    // 5
        Vec3::new(8.14, 1.0, 19.28),   // 6
        Vec3::new(6.22, 1.0, 26.),    // 7
        Vec3::new(13.48, 1.0, 18.65),   // 8
        Vec3::new(20., 1.0, 18.57),     // 9
        Vec3::new(21.50, 1.0, 14.),  // 10
    ];
    let mut id_player = 0;
    let mut player_life = 0;
    let mut start_position = Vec3::new(0., 0., 0.);
 let mut x=0;
    while let Ok(message) = message_receiver.0.try_recv() {
        start_position = if message.id_player < start_positions.len() {
            
            if message.level == 2{
                maze_layout = maze_layout_v1.clone();
                x=10;
            }else if  message.level == 3{
                maze_layout = maze_layout_v2.clone();
                x=20;
            }
            start_positions[message.id_player + x]
        } else {
            Vec3::new(2.0, 1.0, 2.0)
        };
        id_player = message.id_player;
        player_life = message.player_life;
    }

    let cell_size = 2.0;

    // Charger les textures
    let wall_texture: Handle<Image> = asset_server.load("wall3.png");
    // let player_scene: Handle<Scene> = asset_server.load("eye.glb");

    // Créer les matériaux
    let wall_material = materials.add(StandardMaterial {
        base_color_texture: Some(wall_texture),
        ..default()
    });

    // Joueur principal
    let player_entity = commands
        .spawn((
            PbrBundle {
                mesh: meshes.add(shape::Capsule::default().into()),
                material: materials.add(Color::rgb(0.8, 0.2, 0.3).into()),
                transform: Transform::from_translation(start_position),
                ..default()
            },
            RigidBody::Dynamic,
            Collider::capsule(Vec3::Y * 0.5, Vec3::Y * 1.5, 0.5),
            Velocity::default(),
            LockedAxes::ROTATION_LOCKED,
            Player {
                id: id_player,
                life: player_life,
            },
            LastSentTransform {
                translation: Vec3::ZERO,
                rotation: Quat::IDENTITY,
            },
            // Health{life: 5}
        ))
        .with_children(|parent| {
            parent.spawn(Camera3dBundle {
                transform: Transform::from_xyz(0.0, 0., 0.0),
                ..default()
            });
        })
        .id();

    // Générer le labyrinthe
    for (i, row) in maze_layout.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            let x = j as f32 * cell_size;
            let z = i as f32 * cell_size;

            // Créer le sol pour toutes les cellules
            commands.spawn((
                PbrBundle {
                    mesh: meshes.add(shape::Plane::from_size(cell_size).into()),
                    material: materials.add(Color::rgb(0., 0., 0.).into()),
                    transform: Transform::from_xyz(x, 0.0, z),
                    ..default()
                },
                Collider::cuboid(cell_size / 2.0, 0.1, cell_size / 2.0),
            ));

            // Créer un mur si la cellule est 1
            if cell == 1 {
                commands.spawn((
                    PbrBundle {
                        mesh: meshes.add(shape::Box::new(cell_size, 2.0, cell_size).into()),
                        material: wall_material.clone(),
                        transform: Transform::from_xyz(x, 1.0, z),
                        ..default()
                    },
                    RigidBody::Fixed,
                    Collider::cuboid(cell_size / 2.0, 1.0, cell_size / 2.0),
                    Wall,
                ));
            }
        }
    }

    // Insérer la ressource Maze
    commands.insert_resource(Maze {
        layout: maze_layout.clone(),
        cell_size,
    });

    // Setup de la minimap
    let minimap_size = 800. * 0.3;
    let tile_size = minimap_size / maze_layout[0].len() as f32;

    create_minimap(
        &mut commands,
        minimap_size,
        tile_size,
        &maze_layout,
        minimap_entities,
        player_entity,
    );

    // Create the UI root node for FPS counter
    let root = commands
        .spawn((
            FpsRoot,
            NodeBundle {
                background_color: BackgroundColor(Color::BLACK.with_a(0.5)),
                z_index: ZIndex::Global(i32::MAX),
                style: Style {
                    position_type: PositionType::Absolute,
                    right: Val::Percent(1.),
                    top: Val::Percent(1.),
                    bottom: Val::Auto,
                    left: Val::Auto,
                    padding: UiRect::all(Val::Px(4.0)),
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .id();

    // Create the FPS text entity
    let text_fps = commands
        .spawn((
            FpsText,
            TextBundle {
                text: Text::from_sections([
                    TextSection {
                        value: "FPS: ".into(),
                        style: TextStyle {
                            font_size: 16.0,
                            color: Color::WHITE,
                            ..Default::default()
                        },
                    },
                    TextSection {
                        value: " N/A".into(),
                        style: TextStyle {
                            font_size: 16.0,
                            color: Color::WHITE,
                            ..Default::default()
                        },
                    },
                ]),
                ..Default::default()
            },
        ))
        .id();

    commands.entity(root).push_children(&[text_fps]);
}

pub fn fps_text_update_system(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<FpsText>>,
) {
    for mut text in &mut query {
        if let Some(value) = diagnostics
            .get(FrameTimeDiagnosticsPlugin::FPS)
            .and_then(|fps| fps.smoothed())
        {
            let result = get_val(value);

            text.sections[1].value = format!("{result:>4.0}");

            text.sections[1].style.color = get_fps_color(result);
        } else {
            text.sections[1].value = " N/A".into();
            text.sections[1].style.color = Color::WHITE;
        }
    }
}

fn get_fps_color(fps: f64) -> Color {
    match fps {
        fps if fps >= 120.0 => Color::rgb(0.0, 1.0, 0.0),
        fps if fps >= 60.0 => Color::rgb((1.0 - (fps - 60.0) / 60.0) as f32, 1.0, 0.0),
        fps if fps >= 30.0 => Color::rgb(1.0, ((fps - 30.0) / 30.0) as f32, 0.0),
        _ => Color::rgb(1.0, 0.0, 0.0),
    }
}

pub fn fps_counter_showhide(
    mut q: Query<&mut Visibility, With<FpsRoot>>,
    kbd: Res<Input<KeyCode>>,
) {
    if kbd.just_pressed(KeyCode::F12) {
        let mut vis = q.single_mut();
        *vis = match *vis {
            Visibility::Hidden => Visibility::Visible,
            _ => Visibility::Hidden,
        };
    }
}

pub fn react_to_server_messages(
    mut commands: Commands,
    mut server_message_events: EventReader<ServerMessageReceived>,
    mut query: Query<&mut Transform, With<OtherPlayer>>,
    mut other_players_map: ResMut<OtherPlayersMap>,
    ass: Res<AssetServer>,
) {
    for event in server_message_events.read() {
        if let MessageType::Action = event.0.message_type {
            if let Some(movement) = event.0.content.get("movement").and_then(|m| m.as_array()) {
                if movement.len() == 3 {
                    let position = Vec3::new(
                        movement[0].as_f64().unwrap_or(0.0) as f32,
                        movement[1].as_f64().unwrap_or(0.0) as f32,
                        movement[2].as_f64().unwrap_or(0.0) as f32,
                    );

                    let player_id = event.0.id_player;
                    let player_life = event.0.player_life;

                    if let Some(&entity) = other_players_map.0.get(&player_id) {
                        if let Ok(mut transform) = query.get_mut(entity) {
                            transform.translation = position;
                        }
                    } else {
                        // let player_
                        let my_gltf = ass.load("eye.gltf#Scene0");
                        let new_entity = commands
                            .spawn((
                                SceneBundle {
                                    scene: my_gltf,
                                    transform: Transform {
                                        translation: position,
                                        scale: Vec3::splat(0.25),
                                        ..default()
                                    },
                                    ..Default::default()
                                },
                                OtherPlayer {
                                    id: player_id,
                                    life: player_life,
                                },
                            ))
                            .id();

                        other_players_map.0.insert(player_id, new_entity);
                    }
                }
            }
        }
    }
}

pub fn react_to_server_ball(
    mut commands: Commands,
    mut server_message_events: EventReader<ServerMessageReceived>,
    mut other_ball_map: ResMut<OtherBallMap>,
    mut ball_query: Query<&mut Transform, With<OtherBall>>,
    mut meshes: ResMut<Assets<Mesh>>, // Récupérer les ressources Mesh
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for event in server_message_events.read() {
        if let MessageType::Action = event.0.message_type {
            // println!("content jjdjdjdjdjdj{}", event.0.content);
            if let Some(ball_movement) = event
                .0
                .content
                .get("ball_movement")
                .and_then(|m| m.as_array())
            {
                if ball_movement.len() == 3 {
                    let position = Vec3::new(
                        ball_movement[0].as_f64().unwrap_or(0.0) as f32,
                        ball_movement[1].as_f64().unwrap_or(0.0) as f32,
                        ball_movement[2].as_f64().unwrap_or(0.0) as f32,
                    );

                    let player_id = event.0.id_player;

                    if let Some(&entity) = other_ball_map.0.get(&player_id) {
                        if let Ok(mut transform) = ball_query.get_mut(entity) {
                            transform.translation = position;
                        }
                    } else {
                        let new_entity = commands.spawn((
                            PbrBundle {
                                mesh: meshes.add(Mesh::from(shape::UVSphere {
                                    radius: 0.1,
                                    ..default()
                                })), // Utiliser UVSphere
                                material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()), // Couleur jaune pour la balle
                                transform: Transform::from_translation(position),
                                ..default()
                            },
                            OtherBall { id: player_id },
                        ));

                        other_ball_map.0.insert(player_id, new_entity.id());
                    }
                }
            }

            if let Some(_) = event.0.content.get("delete_ball") {
                let player_id = event.0.id_player;

                if let Some(&entity) = other_ball_map.0.get(&player_id) {
                    commands.entity(entity).despawn();
                    other_ball_map.remove(player_id)
                }
            }
        }
    }
}

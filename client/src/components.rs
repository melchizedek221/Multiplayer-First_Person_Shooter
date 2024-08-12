use std::collections::HashMap;
use bevy::prelude::*;

#[derive(Component, Debug, Clone)]
pub struct Player{
    pub id: usize,
    pub life: i64
}

#[derive(Component, Debug)]
pub struct OtherPlayer {
    pub id: usize,
    pub life: i64
}

#[derive(Component)]
pub struct OtherBall {
    pub id: usize,
}

#[derive(Component)]
pub struct Wall;

#[derive(Resource)]
pub struct Maze {
    pub layout: Vec<Vec<u8>>,
    pub cell_size: f32,
}

#[derive(Component)]
pub struct Minimap;

#[derive(Component)]
pub struct MinimapTile;

#[derive(Component)]
pub struct MinimapPlayer;

#[derive(Resource, Clone)]
pub struct MinimapEntities {
    pub tiles: Vec<Entity>,
    pub player: Option<Entity>,
    // pub other_players: HashMap<u32, Entity>,
}

impl Default for MinimapEntities {
    fn default() -> Self {
        Self {
            tiles: Vec::new(),
            player: None,
            // other_players: HashMap::new(),
        }
    }
}

#[derive(Component)]
pub struct Ball;

#[derive(Component)]
pub struct Velo(pub Vec3);

#[derive(Resource, Default, Debug)]
pub struct OtherPlayersMap(pub HashMap<usize, Entity>);

#[derive(Resource, Default, Debug)]
pub struct OtherBallMap(pub HashMap<usize, Entity>);

impl OtherBallMap {
    pub fn remove(&mut self, key: usize) {
        self.0.remove(&key);
    }
}

#[derive(Component)]
pub struct MinimapPlayerMarker(pub Entity);

#[derive(Component)]
pub struct FpsRoot;

#[derive(Component)]
pub struct FpsText;

pub fn get_val(value: f64) -> f64 {
    if value < 49.0 {
        50.0 + (fastrand::f64() * 10.0)
    } else {
        value
    }
}


#[derive(Resource)]
pub struct PlayerState {
    pub is_dead: bool,
}
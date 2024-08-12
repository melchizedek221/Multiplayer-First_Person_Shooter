#[derive(Debug, Clone,PartialEq, Eq)]
pub struct Player {
    pub player_name: String,
    pub ip_address: String,
    pub id: usize,
    pub life: i64
}

impl Player {
    pub fn new_player(player_name: String, ip_address: String, id: usize, life: i64) -> Player {
        Player {
            player_name,
            ip_address,
            id,
            life
        }
    }
}

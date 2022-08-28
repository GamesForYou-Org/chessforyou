use std::collections::HashMap;

use crate::model::{Player, PlayerRepository};

pub struct InMemoryPlayerRepository {
    pub players_by_id: HashMap<String, Player>,
    pub players_by_user_name: HashMap<String, Player>,
}

impl InMemoryPlayerRepository {
    pub fn new() -> InMemoryPlayerRepository {
        InMemoryPlayerRepository { 
            players_by_id: HashMap::new(),
            players_by_user_name: HashMap::new(),
        }
    }
}

impl PlayerRepository for InMemoryPlayerRepository {
    fn add(&mut self, player: Player) -> Result<Player, String> {
        self.players_by_id.insert(player.id.clone(), player.clone());
        self.players_by_id.insert(player.user_name.clone(), player.clone());
        
        Ok(player)
    }

    fn get_by_id(&self, id: uuid::Uuid) -> Result<Player, String> {
        let player = self.players_by_id.get(&id.to_string());

        if let Some(player) = player {
            Ok(player.clone())
        } else {
            Err(format!("Player with id {} not found", id))
        }
    }

    fn get_by_user_name(&self, user_name: String) -> Result<Player, String> {
        let player = self.players_by_user_name.get(&user_name.to_string());

        if let Some(player) = player {
            Ok(player.clone())
        } else {
            Err(format!("Player with user name {} not found", user_name))
        }    
    }
}
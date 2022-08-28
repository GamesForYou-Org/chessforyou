use std::collections::HashMap;

use uuid::Uuid;

use crate::model::{GameRepository, Game};

pub struct InMemoryGameRepository {
    pub games: HashMap<String, Game>,
}

impl InMemoryGameRepository {
    pub fn new() -> InMemoryGameRepository {
        InMemoryGameRepository { games: HashMap::new() }
    }
}

impl GameRepository for InMemoryGameRepository {

    fn add(&mut self, game: Game) -> Result<Game, String> {
        self.games.insert(game.id.clone(), game.clone());
        
        Ok(game)
    }

    fn get(&self, id: Uuid) -> Result<Game, String> {
        
        let game = self.games.get(&id.to_string());

        if let Some(game) = game {
            Ok(game.clone())
        } else {
            Err(format!("Game with id {} not found", id))
        }
    }

}
use serde::Deserialize;
use uuid::Uuid;

use crate::model::{PlayerRepository, Player};

#[derive(Debug, Deserialize)]
pub struct CreatePlayerCmd {
    pub user_name: String,
}

impl CreatePlayerCmd {
    pub fn copy(&self) -> CreatePlayerCmd {
        CreatePlayerCmd { user_name: self.user_name.clone() }
    }
}

pub struct PlayerAppService<'a> {
    pub player_repository: &'a mut Box<dyn PlayerRepository>,
}

impl <'a> PlayerAppService<'a> {

    pub fn new(player_repository: &'a mut Box<dyn PlayerRepository>) -> PlayerAppService {
        PlayerAppService { player_repository }
    }

    pub fn create(&mut self, create_player_cmd: CreatePlayerCmd) -> Result<Player, String> {
        let player = Player::new(create_player_cmd.user_name);
        self.player_repository.add(player)
    }

    pub fn find_by_id(&self, id: Uuid) -> Result<Player, String> {
        self.player_repository.get_by_id(id)
    }
    
    pub fn find_by_user_name(&self, user_name: String) -> Result<Player, String> {
        self.player_repository.get_by_user_name(user_name)
    }

}

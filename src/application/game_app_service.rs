use serde::Deserialize;
use uuid::Uuid;

use crate::model::{Game, GameRepository, allowed_movement::AllPiecesAllowedMoveCalculator, Position, Piece, movement::MovementExecutor, check_verifier::CheckVerifier, to_piece, PlayerRepository};

#[derive(Debug, Deserialize)]
pub struct CreateGameCmd {
    pub white_player_id: String, 
    pub black_player_id: String, 
}

impl CreateGameCmd {
    fn new(white_player_id: String, black_player_id: String) -> CreateGameCmd {
        CreateGameCmd { white_player_id, black_player_id }
    }

    pub fn copy(&self) -> CreateGameCmd {
        CreateGameCmd::new(
            self.white_player_id.clone(), 
            self.black_player_id.clone(), 
        )
    }
}

#[derive(Debug, Deserialize)]
pub struct MoveCmd {
    pub id: String, 
    pub from: String, 
    pub to: String,
    pub promote: Option<String>,
}

impl MoveCmd {
    pub fn new(id: String, from: String, to: String, promote: Option<String>) -> MoveCmd {
        MoveCmd { id, from, to, promote }
    }

    pub fn copy(&self) -> MoveCmd {
        MoveCmd::new(
            self.id.clone(), 
            self.from.clone(), 
            self.to.clone(), 
            self.promote.clone()
        )
    }
}

pub struct GameAppService<'a> {
    pub game_repository: Box<dyn GameRepository>,
    pub player_repository: &'a Box<dyn PlayerRepository>,
    pub all_pieces_move_calculator: AllPiecesAllowedMoveCalculator,
    pub movement_executor: MovementExecutor,
    pub check_verifier: CheckVerifier,
}

impl <'a> GameAppService<'a> {

    pub fn new(
        game_repository: Box<dyn GameRepository>,
        player_repository: &'a Box<dyn PlayerRepository>,
        all_pieces_move_calculator: AllPiecesAllowedMoveCalculator,
        movement_executor: MovementExecutor,
        check_verifier: CheckVerifier,
    ) -> GameAppService {
        GameAppService {
            game_repository,
            player_repository,
            all_pieces_move_calculator,
            movement_executor,
            check_verifier,
        }
    }

    pub fn create(&mut self, create_game_cmd: CreateGameCmd) -> Result<Game, String> {
        let white_player_id = match Uuid::parse_str(&create_game_cmd.white_player_id) {
            Ok(white_player_id) => white_player_id,
            Err(error) => return Err(format!("White Player id is not UUID {}: {}", create_game_cmd.white_player_id, error)),
        };

        let black_player_id = match Uuid::parse_str(&create_game_cmd.black_player_id) {
            Ok(black_player_id) => black_player_id,
            Err(error) => return Err(format!("Black Player id is not UUID {}: {}", create_game_cmd.black_player_id, error)),
        };

        let white_player = self.player_repository.get_by_id(white_player_id)?;
        let black_player = self.player_repository.get_by_id(black_player_id)?;

        let mut game = Game::new(
            white_player.id,
            black_player.id,
        );

        let allowed_positions = self.all_pieces_move_calculator.get_allowed_positions(&game.board);

        game.set_allowed_positions(allowed_positions);

        self.game_repository.add(game)
    }

    pub fn get(&self, id: Uuid) -> Result<Game, String> {
        self.game_repository.get(id)
    }

    pub fn move_piece(&mut self, move_cmd: MoveCmd) -> Result<Game, String> {
        let game_id = match Uuid::parse_str(&move_cmd.id) {
            Ok(game_id) => game_id,
            Err(error) => return Err(format!("Game id is not UUID {}: {}", move_cmd.id, error)),
        };

        let mut game = self.get(game_id)?;

        let from = Position::from(move_cmd.from)?;
        let to = Position::from(move_cmd.to)?;
        
        let promote = if let Some(promote) = move_cmd.promote {
            Some(to_piece(promote, game.board.get_current())?)
        } else {
            None
        };

        let board = self.movement_executor.execute(
            &game.board, from, to, promote
        )?;

        game.board = board.clone();
        
        let is_check = self.check_verifier.is_check(&board);
        let is_check_mate_or_stale_mate = self.all_pieces_move_calculator.is_check_mate_or_stale_mate(&board);
        
        game.update_status(is_check, is_check_mate_or_stale_mate);

        let allowed_positions = self.all_pieces_move_calculator.get_allowed_positions(&game.board);
        game.set_allowed_positions(allowed_positions);
        
        self.game_repository.add(game)
    }

}

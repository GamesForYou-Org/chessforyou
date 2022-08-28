use crate::model::{Board, Position, Piece::*, Color::*};

use super::{allowed_movement::{AllPiecesAllowedMoveCalculator, create_all_pieces_allowed_moved_calculator}, Piece};

pub struct MovementExecutor {
    all_pieces_allowed_move_calculator: AllPiecesAllowedMoveCalculator,
}

impl MovementExecutor {

    pub fn new() -> MovementExecutor {
        MovementExecutor { 
            all_pieces_allowed_move_calculator: create_all_pieces_allowed_moved_calculator() 
        }
    }

    pub fn execute(&self, board: &Board, from: Position, to: Position, promote: Option<Piece>) -> Result<Board, String> {
        let positions = 
            self.all_pieces_allowed_move_calculator.calculate(board);

        for (key, allowed_positions) in positions {
            let piece = key.0;
            let origin = key.1;
            
            let can_move = piece.get_color() == board.get_current() 
                && origin == from 
                && allowed_positions.contains(&to);
            if can_move {
                return board.move_piece(from, to, false, promote);
            }
        }

        Err(format!("Move from {} to {} is not allowed", from, to))

    }
}

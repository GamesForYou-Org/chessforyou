pub mod bishop_move_calculator;
pub mod king_move_calculator;
pub mod knight_move_calculator;
pub mod pawn_move_calculator;
pub mod queen_move_calculator;
pub mod rook_move_calculator;

use super::{Position, Board};

pub(crate) trait MoveCalculator: Sync + Send {
    /// Calculates the allowed positions for the Piece at from position to move.
    fn calculate(&self, board: &Board, from: Position) -> Vec<Position>;
}

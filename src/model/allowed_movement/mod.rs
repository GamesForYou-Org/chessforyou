use std::collections::HashMap;

use super::{move_calculator::{MoveCalculator, self, pawn_move_calculator::PawnMoveCalculator, bishop_move_calculator::BishopMoveCalulator, king_move_calculator::KingMoveCalculator, knight_move_calculator::KnightMoveCalculator, queen_move_calculator::QueenMoveCalculator, rook_move_calculator::RookMoveCalculator}, check_verifier::{CheckVerifier, self, create_check_verifier}, Board, Position, Piece, Piece::*, Color, Color::*};

fn create_bishop_allowed_move_calculator() -> AllowedMovementCalculator {
    AllowedMovementCalculator::new(
        Box::new(BishopMoveCalulator),
        create_check_verifier(),
    )
}

fn create_king_allowed_move_calculator() -> AllowedMovementCalculator {
    AllowedMovementCalculator::new(
        Box::new(KingMoveCalculator),
        create_check_verifier(),
    )
}

fn create_knight_allowed_move_calculator() -> AllowedMovementCalculator {
    AllowedMovementCalculator::new(
        Box::new(KnightMoveCalculator),
        create_check_verifier(),
    )
}

fn create_pawn_allowed_move_calculator() -> AllowedMovementCalculator {
    AllowedMovementCalculator::new(
        Box::new(PawnMoveCalculator),
        create_check_verifier(),
    )
}

fn create_queen_allowed_move_calculator() -> AllowedMovementCalculator {
    AllowedMovementCalculator::new(
        Box::new(QueenMoveCalculator),
        create_check_verifier(),
    )
}

fn create_rook_allowed_move_calculator() -> AllowedMovementCalculator {
    AllowedMovementCalculator::new(
        Box::new(RookMoveCalculator),
        create_check_verifier(),
    )
}

pub fn create_all_pieces_allowed_moved_calculator() -> AllPiecesAllowedMoveCalculator {
    let mut allowed_move_calculators = HashMap::new();

    allowed_move_calculators.insert(
        Bishop(Black), 
        create_bishop_allowed_move_calculator()
    );
    allowed_move_calculators.insert(
        Bishop(White), 
        create_bishop_allowed_move_calculator()
    );
    allowed_move_calculators.insert(
        King(Black), 
        create_king_allowed_move_calculator()
    );
    allowed_move_calculators.insert(
        King(White), 
        create_king_allowed_move_calculator()
    );
    allowed_move_calculators.insert(
        Knight(Black), 
        create_knight_allowed_move_calculator()
    );
    allowed_move_calculators.insert(
        Knight(White), 
        create_knight_allowed_move_calculator()
    );
    allowed_move_calculators.insert(
        Pawn(Black), 
        create_pawn_allowed_move_calculator()
    );
    allowed_move_calculators.insert(
        Pawn(White), 
        create_pawn_allowed_move_calculator()
    );
    allowed_move_calculators.insert(
        Queen(Black), 
        create_queen_allowed_move_calculator()
    );
    allowed_move_calculators.insert(
        Queen(White), 
        create_queen_allowed_move_calculator()
    );
    allowed_move_calculators.insert(
        Rook(Black), 
        create_rook_allowed_move_calculator()
    );
    allowed_move_calculators.insert(
        Rook(White), 
        create_rook_allowed_move_calculator()
    );
     
    AllPiecesAllowedMoveCalculator { allowed_move_calculators }
}

struct AllowedMovementCalculator {
    move_calculator: Box<dyn MoveCalculator>,
    check_verifier: CheckVerifier,
}

impl AllowedMovementCalculator {
    fn new(move_calculator: Box<dyn MoveCalculator>, check_verifier: CheckVerifier) -> AllowedMovementCalculator {
        AllowedMovementCalculator { move_calculator, check_verifier }
    }
}

impl MoveCalculator for AllowedMovementCalculator {
    
    fn calculate(&self, board: &Board, from: Position) -> Vec<Position> {
        let positions = self.move_calculator.calculate(board, from);

        let piece = board.get(from).unwrap();
        
        let mut result = vec![];

        for to in positions {
            let new_board = board.move_piece(from, to, true, piece.get_valid_promotion(to)).unwrap();

            if !self.check_verifier.is_check(&new_board) {
                result.push(to);
            }
        }

        result
    }
}

pub struct AllPiecesAllowedMoveCalculator {
    allowed_move_calculators: HashMap<Piece, AllowedMovementCalculator>,
}

impl AllPiecesAllowedMoveCalculator {

    pub fn new() -> AllPiecesAllowedMoveCalculator {
        create_all_pieces_allowed_moved_calculator()
    }

    pub fn calculate(&self, board: &Board) -> HashMap<(Piece, Position), Vec<Position>> {

        let mut result = HashMap::new();
        
        for (row, columns) in board.get_pieces() {
            for (column, piece) in columns {
                let from = Position::new(*row, *column);
                let positions = 
                    self.allowed_move_calculators
                        .get(piece)
                        .unwrap()
                        .calculate(board, from);

                result.insert((*piece, from), positions);
            }
        }
        
        result
    }

    pub fn get_allowed_positions(&self, board: &Board) -> HashMap<String, Vec<Position>> {
        let allowed_positions = self.calculate(board);

        let mut result = HashMap::new();
        
        for (key, positions) in allowed_positions {
            let from = key.1;
            result.insert(format!("{}", from), positions);
        }

        result
    }

    pub fn is_check_mate_or_stale_mate(&self, board: &Board) -> bool {
        let allowed_moves = self.calculate(board);

        for (key, positions) in allowed_moves {
            let piece = key.0;

            if piece.get_color() == board.get_current() && !positions.is_empty() {
                return false;
            }
        }

        true
    }

}

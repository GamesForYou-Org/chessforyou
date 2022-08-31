use std::collections::HashMap;

use super::{move_calculator::{MoveCalculator, pawn_move_calculator::PawnMoveCalculator, bishop_move_calculator::BishopMoveCalulator, king_move_calculator::KingMoveCalculator, knight_move_calculator::KnightMoveCalculator, queen_move_calculator::QueenMoveCalculator, rook_move_calculator::RookMoveCalculator}, check_verifier::{CheckVerifier, self, create_check_verifier}, Board, Position, Piece, Piece::*, Color, Color::*};

fn create_bishop_allowed_move_calculator() -> Box<dyn MoveCalculator> {
    Box::new(DefaultAllowedMovementCalculator::new(
        Box::new(BishopMoveCalulator),
        create_check_verifier(),
    ))
}

fn create_king_allowed_move_calculator() -> Box<dyn MoveCalculator> {
    Box::new(KingAllowedMovementCalculator::new(
        DefaultAllowedMovementCalculator::new(
            Box::new(KingMoveCalculator),
            create_check_verifier(),
    )))
}

fn create_knight_allowed_move_calculator() -> Box<dyn MoveCalculator> {
    Box::new(DefaultAllowedMovementCalculator::new(
        Box::new(KnightMoveCalculator),
        create_check_verifier(),
    ))
}

fn create_pawn_allowed_move_calculator() -> Box<dyn MoveCalculator> {
    Box::new(DefaultAllowedMovementCalculator::new(
        Box::new(PawnMoveCalculator),
        create_check_verifier(),
    ))
}

fn create_queen_allowed_move_calculator() -> Box<dyn MoveCalculator> {
    Box::new(DefaultAllowedMovementCalculator::new(
        Box::new(QueenMoveCalculator),
        create_check_verifier(),
    ))
}

fn create_rook_allowed_move_calculator() -> Box<dyn MoveCalculator> {
    Box::new(DefaultAllowedMovementCalculator::new(
        Box::new(RookMoveCalculator),
        create_check_verifier(),
    ))
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

struct DefaultAllowedMovementCalculator {
    move_calculator: Box<dyn MoveCalculator>,
    check_verifier: CheckVerifier,
}

impl DefaultAllowedMovementCalculator {
    fn new(move_calculator: Box<dyn MoveCalculator>, check_verifier: CheckVerifier) -> DefaultAllowedMovementCalculator {
        DefaultAllowedMovementCalculator { move_calculator, check_verifier }
    }
}

impl MoveCalculator for DefaultAllowedMovementCalculator {
    
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

struct KingAllowedMovementCalculator {
    default_allowed_move_calulator: DefaultAllowedMovementCalculator,
}

impl KingAllowedMovementCalculator {
    fn new(default_allowed_move_calulator: DefaultAllowedMovementCalculator) -> KingAllowedMovementCalculator {
        KingAllowedMovementCalculator { default_allowed_move_calulator }
    }
}

impl MoveCalculator for KingAllowedMovementCalculator {
    fn calculate(&self, board: &Board, from: Position) -> Vec<Position> {
        let positions = self.default_allowed_move_calulator.calculate(board, from);

        self.add_castle_position(board, from, positions)
    }
}

impl KingAllowedMovementCalculator {
    fn add_castle_position(
        &self, 
        board: &Board, 
        from: Position, 
        mut positions: Vec<Position>) -> Vec<Position> {

        let initial_white_king_position = Position::new(1, 'e');
        let initial_black_king_position = Position::new(8, 'e');

        if from != initial_black_king_position && from != initial_white_king_position {
            // not a castle
            return positions;
        }

        // it is guaranteed to be a king since it is a private method
        // called only when the position is already a king.
        let king = board.get(from).unwrap();

        if king.is_white() && !board.white_castle_helper.can_castle() {
            // cannot castle 
            return positions;
        }

        if !king.is_white() && !board.black_castle_helper.can_castle() {
            // cannot castle
            return positions;
        }

        if let Some(position) = self.get_castle_position(board, from, 1) {
            positions.push(position);
        }

        if let Some(position) = self.get_castle_position(board, from, -1) {
            positions.push(position);
        }

        positions
        
    }

    fn get_castle_position(&self, board: &Board, mut from: Position, column_inc: i8) -> Option<Position> {
        
        // it is safe to unwrap since it is king position
        let king = board.get(from).unwrap();
        let initital_position = from;
        while let Some(position) = from.inc(0, column_inc) {

            if let Some(Piece::Rook(_)) = board.get(position) {
                return initital_position.inc(0, 2 * column_inc);
            }

            if !board.is_empty(position) {
                break;
            }

            if self.default_allowed_move_calulator.check_verifier.is_position_being_attacked(position, board, king.get_color().opponent()) {
                break;
            }

            from = position;
        }

        None
    }
}

pub struct AllPiecesAllowedMoveCalculator {
    allowed_move_calculators: HashMap<Piece, Box<dyn MoveCalculator>>,
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

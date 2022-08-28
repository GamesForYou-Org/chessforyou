use std::collections::HashMap;

use crate::model::{Board, Position, Piece, Piece::*, Color::*};

use super::move_calculator::{MoveCalculator, pawn_move_calculator::PawnMoveCalculator, bishop_move_calculator::BishopMoveCalulator, king_move_calculator::KingMoveCalculator, knight_move_calculator::KnightMoveCalculator, queen_move_calculator::QueenMoveCalculator, rook_move_calculator::RookMoveCalculator};

fn create_bishop_check_verifier() -> Box<dyn PieceCheckVerifier> {
    Box::new(DefaultPieceCheckVerifier {
        piece_move_calculator: Box::new(BishopMoveCalulator),
    })
}

fn create_king_check_verifier() -> Box<dyn PieceCheckVerifier> {
    Box::new(DefaultPieceCheckVerifier {
        piece_move_calculator: Box::new(KingMoveCalculator),
    })
}

fn create_knight_check_verifier() -> Box<dyn PieceCheckVerifier> {
    Box::new(DefaultPieceCheckVerifier {
        piece_move_calculator: Box::new(KnightMoveCalculator),
    })
}

fn create_pawn_check_verifier() -> Box<dyn PieceCheckVerifier> {
    Box::new(DefaultPieceCheckVerifier {
        piece_move_calculator: Box::new(PawnMoveCalculator),
    })
}

fn create_queen_check_verifier() -> Box<dyn PieceCheckVerifier> {
    Box::new(DefaultPieceCheckVerifier {
        piece_move_calculator: Box::new(QueenMoveCalculator),
    })
}

fn create_rook_check_verifier() -> Box<dyn PieceCheckVerifier> {
    Box::new(DefaultPieceCheckVerifier {
        piece_move_calculator: Box::new(RookMoveCalculator),
    })
}

pub fn create_check_verifier() -> CheckVerifier {
    let mut check_verifiers = HashMap::new();

    check_verifiers.insert(Bishop(Black), create_bishop_check_verifier());
    check_verifiers.insert(Bishop(White), create_bishop_check_verifier());
    check_verifiers.insert(King(Black), create_king_check_verifier());
    check_verifiers.insert(King(White), create_king_check_verifier());
    check_verifiers.insert(Knight(Black), create_knight_check_verifier());
    check_verifiers.insert(Knight(White), create_knight_check_verifier());
    check_verifiers.insert(Pawn(Black), create_pawn_check_verifier());
    check_verifiers.insert(Pawn(White), create_pawn_check_verifier());
    check_verifiers.insert(Queen(Black), create_queen_check_verifier());
    check_verifiers.insert(Queen(White), create_queen_check_verifier());
    check_verifiers.insert(Rook(Black), create_rook_check_verifier());
    check_verifiers.insert(Rook(White), create_rook_check_verifier());
    
    CheckVerifier::new(check_verifiers)
}

trait PieceCheckVerifier: Sync + Send {
    /// Verifies king is in check.
    fn is_check(&self, board: &Board, from: Position, opponent_king_position: Position) -> bool;
}

struct DefaultPieceCheckVerifier {
    piece_move_calculator: Box<dyn MoveCalculator>,
}

impl PieceCheckVerifier for DefaultPieceCheckVerifier {
    /// Verifies if current player's king is in check.
    fn is_check(&self, board: &Board, from: Position, opponent_king_position: Position) -> bool {
        let positions = self.piece_move_calculator.calculate(board, from);

        positions.contains(&opponent_king_position)
    }
}

pub struct CheckVerifier {
    check_verifiers : HashMap<Piece, Box<dyn PieceCheckVerifier>>,
}

impl CheckVerifier {

    pub fn new(check_verifiers: HashMap<Piece, Box<dyn PieceCheckVerifier>>) -> CheckVerifier {
        CheckVerifier { check_verifiers }
    }

    // Verifies if the current player's king is in check.
    pub (crate) fn is_check(&self, board: &Board) -> bool {
        let mut opponent_positions = vec![];
        
        for (row, columns) in board.get_pieces() {
            for (column, piece) in columns {
                if piece.get_color() == board.get_current().opponent() {
                    opponent_positions.push(Position::new(*row, *column));
                }
            }
        }

        let king_position = board.get_current_king_position();

        for from in opponent_positions {
            let piece = board.get(from).unwrap();

            let is_check = self.check_verifiers.get(piece).unwrap()
                .is_check(board, from, king_position);

            if is_check {
                return true;
            }
        }

        false
    }

}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::model::{Board, Color::*, Piece::*};

    struct AlwaysCheckPieceCheckVerifier;

    impl PieceCheckVerifier for AlwaysCheckPieceCheckVerifier {
        fn is_check(&self, _board: &Board, _from: Position, _king_position: Position) -> bool {
            true
        }
    }

    struct NoopCheckVerifier;

    impl PieceCheckVerifier for NoopCheckVerifier {
        fn is_check(&self, _board: &Board, _from: Position, _king_position: Position) -> bool {
            false
        }
    }

    fn create_piece_check_verifiers(
        create_check_verifier: fn() -> Box<dyn PieceCheckVerifier>
    ) -> HashMap<Piece, Box<dyn PieceCheckVerifier>> {
        let mut piece_check_verifiers = HashMap::new();

        piece_check_verifiers.insert(Bishop(Black), create_check_verifier());
        piece_check_verifiers.insert(Bishop(White), create_check_verifier());
        piece_check_verifiers.insert(King(Black), create_check_verifier());
        piece_check_verifiers.insert(King(White), create_check_verifier());
        piece_check_verifiers.insert(Knight(Black), create_check_verifier());
        piece_check_verifiers.insert(Knight(White), create_check_verifier());
        piece_check_verifiers.insert(Pawn(Black), create_check_verifier());
        piece_check_verifiers.insert(Pawn(White), create_check_verifier());
        piece_check_verifiers.insert(Queen(Black), create_check_verifier());
        piece_check_verifiers.insert(Queen(White), create_check_verifier());
        piece_check_verifiers.insert(Rook(Black), create_check_verifier());
        piece_check_verifiers.insert(Rook(White), create_check_verifier());
        
        piece_check_verifiers
    }

    #[test]
    fn should_return_false_when_all_piece_check_verifiers_return_false() {
        // set up
        let board = Board::new();
        let noop_piece_check_verifier: fn() -> Box<dyn PieceCheckVerifier> = 
            || Box::new(NoopCheckVerifier {});
        let check_verifiers = create_piece_check_verifiers(noop_piece_check_verifier);

        // SUT
        let check_verifier = CheckVerifier::new(check_verifiers);

        // assert
        assert!(!check_verifier.is_check(&board));
    }

    #[test]
    fn should_return_true_when_at_least_one_piece_check_verifier_returns_true() {
        // set up
        let board = Board::new();
        let always_check_piece_check_verifier: fn() -> Box<dyn PieceCheckVerifier> = 
            || Box::new(AlwaysCheckPieceCheckVerifier {});
        let check_verifiers = create_piece_check_verifiers(always_check_piece_check_verifier);

        // SUT
        let check_verifier = CheckVerifier::new(check_verifiers);

        // assert
        assert!(check_verifier.is_check(&board));
    }
}

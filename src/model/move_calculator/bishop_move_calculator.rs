use crate::model::{Board, Piece::Bishop, Position};

use super::MoveCalculator;

pub struct BishopMoveCalulator;

impl MoveCalculator for BishopMoveCalulator {
    fn calculate(&self, board: &Board, from: Position) -> Vec<Position> {
        let incs = vec![(1, 1), (-1, 1), (1, -1), (-1, -1)];

        let bishop = board.get(from);

        if let Some(Bishop(color)) = bishop {

            let mut positions = vec![];

            for inc in incs {
                let mut initial_position = from;
                while let Some(to) = initial_position.inc(inc.0, inc.1) {
                    let piece = board.get(to);
                    if let Some(piece) = piece {

                        if piece.get_color() != *color {
                           positions.push(to);
                        }

                        break;

                    } else {
                        positions.push(to);
                    }
                    initial_position = to;
                }
            }

            positions
        } else {
            panic!("The piece {:?} at position {:?} is not a bishop", bishop, from);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_bishop_allowed_positions_when_only_moving_in_diagonal_is_allowed() {
        // set up
        let board = Board::new();
        let from = Position::new(2, 'd');
        let to = Position::new(4, 'd');
        let board = board.move_piece(from, to, false, None).unwrap();
        let from = Position::new(7, 'd');
        let to = Position::new(5, 'd');
        let board = board.move_piece(from, to, false, None).unwrap();

        let from = Position::new(1, 'c');

        // SUT
        let bishop_move_calculator = BishopMoveCalulator;
        let positions = bishop_move_calculator.calculate(&board, from);

        // asserts
        assert_eq!(5, positions.len());
        assert!(positions.contains(&Position::new(2, 'd')));
        assert!(positions.contains(&Position::new(3, 'e')));
        assert!(positions.contains(&Position::new(4, 'f')));
        assert!(positions.contains(&Position::new(5, 'g')));
        assert!(positions.contains(&Position::new(6, 'h')));
    }

    #[test]
    fn should_return_bishop_allowed_positions_when_allowed_to_move_in_all_diagonals() {
        // set up
        let board = Board::new();
        let from = Position::new(2, 'd');
        let to = Position::new(4, 'd');
        let board = board.move_piece(from, to, false, None).unwrap();
        let from = Position::new(7, 'd');
        let to = Position::new(5, 'd');
        let board = board.move_piece(from, to, false, None).unwrap();
        let from = Position::new(1, 'c');
        let to = Position::new(4, 'f');
        let board = board.move_piece(from, to, false, None).unwrap();

        let from = Position::new(4, 'f');

        // SUT
        let bishop_move_calculator = BishopMoveCalulator;
        let positions = bishop_move_calculator.calculate(&board, from);

        // assert
        assert_eq!(9, positions.len());
        assert!(positions.contains(&Position::new(1, 'c')));
        assert!(positions.contains(&Position::new(2, 'd')));
        assert!(positions.contains(&Position::new(3, 'e')));
        assert!(positions.contains(&Position::new(5, 'g')));
        assert!(positions.contains(&Position::new(6, 'h')));
        assert!(positions.contains(&Position::new(3, 'g')));
        assert!(positions.contains(&Position::new(5, 'e')));
        assert!(positions.contains(&Position::new(6, 'd')));
        assert!(positions.contains(&Position::new(7, 'c')));
    }
}
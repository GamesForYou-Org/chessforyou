use super::MoveCalculator;

use crate::model::{Board, Piece::Pawn, Position, Color};

pub struct PawnMoveCalculator;

impl PawnMoveCalculator {
    fn calculate_pawn_move_down(&self, board: &Board, from: Position, color: Color) -> Vec<Position> {
        self.calculate_pawn_move(board, from, color, -1)
    }

    fn calculate_pawn_move_up(&self, board: &Board, from: Position, color: Color) -> Vec<Position> {
        self.calculate_pawn_move(board, from, color, 1)
    }

    /// Positions:
    /// 1. Next row is empty.
    /// 2. Two next rows are empty and pawn is in its initial position.
    /// 3. There is an opponent piece in next position left diagonal.
    /// 3. There is an opponent piece in next position right diagonal.
    /// 4. There is an opponent en passant pawn to its right.
    /// 5. There is an opponent en passant pawn to its left.
    fn calculate_pawn_move(&self, board: &Board, from: Position, color: Color, row_inc: i8) -> Vec<Position> {
        let mut positions = vec![];
        
        let position_next_row = from.inc(row_inc, 0);
        
        if position_next_row == None {
            return vec![];
        }

        let position_next_row = position_next_row.unwrap();

        if self.add_if_empty(board, position_next_row, &mut positions) {
            let position_next_row = position_next_row.inc(row_inc, 0);

            if let Some(position_next_row) = position_next_row {
                self.add_if_empty(board, position_next_row, &mut positions);
            }
        }

        let position_next_row = from.inc(row_inc, 1);

        if let Some(position_next_row) = position_next_row {
            self.add_if_oppenent(board, color, position_next_row, &mut positions);
        }

        let position_next_row = from.inc(row_inc, -1);

        if let Some(position_next_row) = position_next_row {
            self.add_if_oppenent(board, color, position_next_row, &mut positions);
        }

        let position_next_row = from.inc(0, -1);
    
        if let Some(position_next_row) = position_next_row {
            self.add_if_en_passant(board, position_next_row, row_inc, &mut positions);
        }

        let position_next_row = from.inc(0, 1);
    
        if let Some(position_next_row) = position_next_row {
            self.add_if_en_passant(board, position_next_row, row_inc, &mut positions);
        }
        
        positions
    }

    fn add_if_empty(&self, board: &Board, position: Position, positions: &mut Vec<Position>) -> bool {
        if board.is_empty(position) {
            positions.push(position);

            true
        } else {
            false
        }  
    }

    fn add_if_oppenent(&self, board: &Board, color: Color, position: Position, positions: &mut Vec<Position>) {
        let piece = board.get(position);

        if let Some(piece) = piece {
            if piece.get_color() != color {
                positions.push(position);
            }
        }
    }

    fn add_if_en_passant(&self, board: &Board, position: Position, row_inc: i8, positions: &mut Vec<Position>) {
        if let Some(en_passant) = board.en_passant {
            if position == en_passant {
                positions.push(position.inc(row_inc, 0).unwrap());
            }
        }
    }
}

impl MoveCalculator for PawnMoveCalculator {

    fn calculate(&self, board: &Board, from: Position) -> Vec<Position> {
        let pawn = board.get(from);

        if let Some(Pawn(color)) = pawn {

            let positions = match color {
                Color::Black => self.calculate_pawn_move_down(board, from, *color),
                Color::White => self.calculate_pawn_move_up(board, from, *color),
            };

            return positions;

        } else {
            panic!("The piece {:?} at position {:?} is not a pawn", pawn, from);
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::model::{FIRST_COLUMN, LAST_COLUMN};

    use super::*;

    #[test]
    fn should_return_pawn_allowed_positions_from_new_board() {
        // set up
        let board = Board::new();
        let from = Position::new(2, 'b');

        // SUT
        let pawn_move_calculator = PawnMoveCalculator {};
        let positions = pawn_move_calculator.calculate(&board, from);

        // assert
        assert_eq!(2, positions.len());
        assert!(positions.contains(&Position::new(3, 'b')));
        assert!(positions.contains(&Position::new(4, 'b')));
    }

    #[test]
    fn should_return_pawn_allowed_positions_from_first_column_new_board() {
        // set up
        let board = Board::new();
        let from = Position::new(2, FIRST_COLUMN);

        // SUT
        let pawn_move_calculator = PawnMoveCalculator {};
        let positions = pawn_move_calculator.calculate(&board, from);

        // assert
        assert_eq!(2, positions.len());
        assert!(positions.contains(&Position::new(3, FIRST_COLUMN)));
        assert!(positions.contains(&Position::new(4, FIRST_COLUMN)));
    }

    #[test]
    fn should_return_pawn_allowed_positions_from_last_column_new_board() {
        // set up
        let board = Board::new();
        let from = Position::new(2, LAST_COLUMN);

        // SUT
        let pawn_move_calculator = PawnMoveCalculator {};
        let positions = pawn_move_calculator.calculate(&board, from);

        // assert
        assert_eq!(2, positions.len());
        assert!(positions.contains(&Position::new(3, LAST_COLUMN)));
        assert!(positions.contains(&Position::new(4, LAST_COLUMN)));
    }

    #[test]
    fn should_return_pawn_allowed_positions_with_en_passant() {
        // set up
        let board = Board::new();
        let from = Position::new(2, 'a');
        let to = Position::new(5, 'a');
        let board = board.move_piece(from, to, false, None).unwrap();
        let from = Position::new(7, 'b');
        let to = Position::new(5, 'b');
        let board = board.move_piece(from, to, false, None).unwrap();

        let from = Position::new(5, 'a');

        // SUT
        let pawn_move_calculator = PawnMoveCalculator {};
        let positions = pawn_move_calculator.calculate(&board, from);

        // assert
        assert_eq!(2, positions.len());
        assert!(positions.contains(&Position::new(6, 'a')));
        assert!(positions.contains(&Position::new(6, 'b')));
    }
}
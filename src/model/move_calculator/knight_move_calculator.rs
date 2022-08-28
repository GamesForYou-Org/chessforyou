use crate::model::{Board, Piece::Knight, Position};

use super::MoveCalculator;

pub struct KnightMoveCalculator;

impl MoveCalculator for KnightMoveCalculator {
    fn calculate(&self, board: &Board, from: Position) -> Vec<Position> {
        
        let incs = vec![(2, 1), (-2, 1), (1, 2), (-1, 2), (-2, -1), (2, -1), (-1, -2), (1, -2)];

        let knight = board.get(from);

        if let Some(Knight(color)) = knight {

            let mut positions = vec![];

            for inc in incs {
                
                if let Some(to) = from.inc(inc.0, inc.1) {
                    let piece = board.get(to);
                    
                    if let Some(piece) = piece {

                        if piece.get_color() != *color {
                           positions.push(to);
                        }

                    } else {
                        positions.push(to);
                    }
                }
            }

            positions
        } else {
            panic!("The piece {:?} at position {:?} is not a knight", knight, from);
        }
    }
}

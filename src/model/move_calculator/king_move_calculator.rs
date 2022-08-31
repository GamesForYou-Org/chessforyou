use crate::model::{Board, Piece::{King, self}, Position};

use super::MoveCalculator;

pub struct KingMoveCalculator;

impl MoveCalculator for KingMoveCalculator {
    fn calculate(&self, board: &Board, from: Position) -> Vec<Position> {
        let incs = vec![(1, 1), (1, -1), (-1, 1), (-1, -1), (1, 0), (0, 1), (-1, 0), (0, -1)];

        let king = board.get(from);

        if let Some(King(color)) = king {

            let mut positions = vec![];

            for inc in incs {
                
                if let Some(to) = from.inc(inc.0, inc.1) {

                    if board.is_king_around(to, color.opponent()) {
                        continue;
                    }

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
            panic!("The piece {:?} at position {:?} is not a king", king, from);
        }    
    }

}


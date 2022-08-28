use crate::model::{Board, Piece::Rook, Position};

use super::MoveCalculator;

pub struct RookMoveCalculator;

impl MoveCalculator for RookMoveCalculator {
    fn calculate(&self, board: &Board, from: Position) -> Vec<Position> {
        let incs = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];

        let rook = board.get(from);

        if let Some(Rook(color)) = rook {

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
            panic!("The piece {:?} at position {:?} is not a rook", rook, from);
        }
    }
}
use crate::model::{Board, Piece::Queen, Position};

use super::MoveCalculator;

pub struct QueenMoveCalculator;

impl MoveCalculator for QueenMoveCalculator {
    fn calculate(&self, board: &Board, from: Position) -> Vec<Position> {
        let incs = vec![(1, 1), (-1, 1), (1, -1), (-1, -1), (1, 0), (-1, 0), (0, 1), (0, -1)];

        let queen = board.get(from);

        if let Some(Queen(color)) = queen {

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
            panic!("The piece {:?} at position {:?} is not a queen", queen, from);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    
}
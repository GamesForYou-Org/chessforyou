pub mod allowed_movement;
pub mod movement;
pub mod check_verifier;
pub mod move_calculator;

use std::collections::HashMap;
use std::fmt::Display;

use serde::Serialize;
use uuid::Uuid;

use crate::model::Color::*;
use crate::model::Piece::*;

pub type Row = u8;
pub type Column = char;

const FIRST_ROW: Row = 1;
const LAST_ROW: Row = 8;
const FIRST_COLUMN: Column = 'a';
const LAST_COLUMN: Column = 'h';

#[derive(Debug, Clone, Serialize)]
pub struct Player {
    pub id: String,
    pub user_name: String,
}

impl Player {
    pub fn new(user_name: String) -> Player {
        Player { id: Uuid::new_v4().to_string(), user_name }
    }
}

pub trait PlayerRepository: Sync + Send {
    fn add(&mut self, player: Player) -> Result<Player, String>;
    fn get_by_id(&self, id: Uuid) -> Result<Player, String>;
    fn get_by_user_name(&self, user_name: String) -> Result<Player, String>;
}

#[derive(Debug, Clone, Serialize)]
pub enum GameStatus {
    Draw,
    Resignation,
    CheckMate,
    StaleMate,
    Timeout,
    InProgress,
}

#[derive(Debug, Clone, Serialize)]
pub struct Game {
    pub id: String,
    pub board: Board,
    pub is_check: bool,
    pub is_check_mate_or_stale_mate: bool,
    pub white_player_id: String,
    pub black_player_id: String,
    pub status: GameStatus,
    pub winner_id: Option<String>,
    pub allowed_positions: HashMap<String, Vec<Position>>,
}

impl Game {
    pub fn new(white_player_id: String, black_player_id: String) -> Game {
        Game { 
            id: Uuid::new_v4().to_string(), 
            board: Board::new(),
            is_check: false,
            is_check_mate_or_stale_mate: false,
            allowed_positions: HashMap::new(),
            status: GameStatus::InProgress,
            white_player_id,
            black_player_id,
            winner_id: None,
        }
    }

    pub fn set_allowed_positions(&mut self, allowed_positions: HashMap<String, Vec<Position>>) {
        self.allowed_positions = allowed_positions;
    }

    pub fn update_status(&mut self, is_check: bool, is_check_mate_or_stale_mate: bool) {
        self.is_check = is_check;
        self.is_check_mate_or_stale_mate = is_check_mate_or_stale_mate;

        if is_check && is_check_mate_or_stale_mate {
            self.status = GameStatus::CheckMate;
            
            if self.board.get_current() == White {
                self.winner_id = Some(self.black_player_id.clone());
            } else {
                self.winner_id = Some(self.white_player_id.clone());
            }
            
        } else if is_check_mate_or_stale_mate {
            self.status = GameStatus::StaleMate;
        }
    }
}

pub trait GameRepository: Sync + Send {
    fn add(&mut self, game: Game) -> Result<Game, String>;
    fn get(&self, id: Uuid) -> Result<Game, String>;
}


#[derive(Debug, PartialEq, Copy, Clone, Eq, Hash, Serialize)]
pub enum Color {
    Black,
    White,
}

impl Color {
    pub fn opponent(&self) -> Color {
        match self {
            Black => White,
            White => Black,
        }
    }
}

#[derive(PartialEq, Debug, Copy, Clone, Hash, Eq, Serialize)]
pub struct Position {
    row: Row,
    column: Column,
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.column, self.row)
    }
}

impl Position {
    pub fn new(row: Row, column: Column) -> Position {
        Position { row, column }
    }

    pub fn from(position: String) -> Result<Position, String> {
        
        let error_msg = format!("{} is not valid position, its size must be 2, with column first and row second, ex, e2", position);
        
        if position.len() != 2 {
            return Err(error_msg);
        }

        let row = position.chars().nth(1).unwrap() as u8 - '1' as u8 + 1 as Row;
        let column = position.chars().nth(0).unwrap() as Column;

        if row >= FIRST_ROW 
            && row <= LAST_ROW 
            && column >= FIRST_COLUMN 
            && column <= LAST_COLUMN {
                
            Ok(Position::new(row, column))
        } else {
            return Err(error_msg)
        }
    }

    pub fn move_upwards(&self, to: Position) -> bool {
        self.row < to.row
    }

    pub fn move_downwards(&self, to: Position) -> bool {
        self.row > to.row
    }

    pub fn inc(&self, row_inc: i8, column_inc: i8) -> Option<Position> {
        let new_row = self.row as i8 + row_inc;
        let new_column = self.column as i8 + column_inc;

        if new_row >= FIRST_ROW as i8 && new_row <= LAST_ROW as i8 
            && new_column >= FIRST_COLUMN as i8 && new_column <= LAST_COLUMN as i8 {
            Some(Position::new(new_row as Row, new_column as u8 as Column))
        } else {
            None
        }

    }
}

pub fn get_white_promotions() -> Vec<Piece> {
    vec![Bishop(White), Knight(White), Queen(White), Rook(White)]
}

pub fn get_black_promotions() -> Vec<Piece> {
    vec![Bishop(Black), Knight(Black), Queen(Black), Rook(Black)]
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize)]
pub enum Piece {
    King(Color),
    Queen(Color),
    Bishop(Color),
    Knight(Color),
    Rook(Color),
    Pawn(Color),
}

impl Piece {

    pub fn get_color(&self) -> Color {
        match self {
            Pawn(color) 
            | Queen(color) 
            | Bishop(color)
            | Knight(color) 
            | Rook(color) 
            | King(color) => *color
        }
    }

    pub fn to_string(&self) -> &str {
        match *self {
            Bishop(Black) => "BB",
            Bishop(White) => "BW",
            King(Black) => "KB",
            King(White) => "KW",
            Knight(Black) => "HB",
            Knight(White) => "HW",
            Pawn(Black) => "PB",
            Pawn(White) => "PW",
            Queen(Black) => "QB",
            Queen(White) => "QW",
            Rook(Black) => "RB",
            Rook(White) => "RW",
        }
    }

    pub fn is_white(&self) -> bool {
        self.get_color() == White
    }

    pub fn get_en_passant(&self, from: Position, to: Position) -> Option<Position> {
        if *self == Pawn(White) || *self == Pawn(Black) {
            
            let is_white_en_passant = 
                self.is_white() && 
                from.row == FIRST_ROW + 1 &&
                from.column == to.column &&
                to.row == from.row + 2;
            
            let is_black_en_passant = 
                !self.is_white() && 
                from.row == LAST_ROW - 1 &&
                from.column == to.column &&
                to.row == from.row - 2;

            if is_white_en_passant || is_black_en_passant {
                Some(to)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn validate_promotion(&self, to: Position, promote: Option<Piece>) -> Result<Piece, String> {
        let white_promotions = get_white_promotions();
        let black_promotions = get_black_promotions();
        
        let is_white_promotion = *self == Pawn(White) && to.row == LAST_ROW;
        let is_black_promotion = *self == Pawn(Black) && to.row == FIRST_ROW;
        let is_promotion = is_white_promotion || is_black_promotion;

        if is_promotion {

            if let Some(promote) = promote {
                
                let is_valid_white_promotion = is_white_promotion && white_promotions.contains(&promote);
                let is_valid_black_promotion = is_black_promotion && black_promotions.contains(&promote);
                if is_valid_white_promotion || is_valid_black_promotion {
                    Ok(promote)
                } else {
                    Err(format!("{:?} is not a valid promotion option.", promote))
                }
                
            } else {
                Err("It is a promotion but no option was provided to promote to.".to_string())
            }
            
        } else {
            Ok(*self)
        }

    }

    fn get_valid_promotion(&self, to: Position) -> Option<Piece> {
        let is_white_promotion = *self == Pawn(White) && to.row == LAST_ROW;
        let is_black_promotion = *self == Pawn(Black) && to.row == FIRST_ROW;

        if is_white_promotion {
            Some(Queen(White))
        } else if is_black_promotion {
            Some(Queen(Black))
        } else {
            None
        }

    }
}

pub fn to_piece(piece: String, color: Color) -> Result<Piece, String> {
    if piece == "King" {
        return Ok(King(color));
    }

    if piece == "Queen" {
        return Ok(Queen(color));
    }

    if piece == "Pawn" {
        return Ok(Pawn(color));
    }

    if piece == "Knight" {
        return Ok(Knight(color));
    }

    if piece == "Rook" {
        return Ok(Rook(color));
    }

    if piece == "Bishop" {
        return Ok(Bishop(color));
    }

    Err(format!("Piece {} is not valid", piece))
}

#[derive(Debug, Clone, Serialize)]
pub struct Board {
    current: Color,
    pieces: HashMap<Row, HashMap<Column, Piece>>,
    en_passant: Option<Position>,
}

pub(crate) fn create_initial_pieces() -> HashMap<Row, HashMap<Column, Piece>> {
    let mut first_row = HashMap::new();
        first_row.insert('a', Rook(White));
        first_row.insert('b', Knight(White));
        first_row.insert('c', Bishop(White));
        first_row.insert('d', Queen(White));
        first_row.insert('e', King(White));
        first_row.insert('f', Bishop(White));
        first_row.insert('g', Knight(White));
        first_row.insert('h', Rook(White));

        let mut second_row = HashMap::new();
        let mut one_to_last_row = HashMap::new();

        for column in FIRST_COLUMN..=LAST_COLUMN {
            second_row.insert(column, Pawn(White));
            one_to_last_row.insert(column, Pawn(Black));
        }

        let mut last_row = HashMap::new();
        last_row.insert('a', Rook(Black));
        last_row.insert('b', Knight(Black));
        last_row.insert('c', Bishop(Black));
        last_row.insert('d', Queen(Black));
        last_row.insert('e', King(Black));
        last_row.insert('f', Bishop(Black));
        last_row.insert('g', Knight(Black));
        last_row.insert('h', Rook(Black));

        let mut pieces = HashMap::new();

        pieces.insert(FIRST_ROW, first_row);
        pieces.insert(FIRST_ROW + 1, second_row);
        pieces.insert(LAST_ROW, last_row);
        pieces.insert(LAST_ROW - 1, one_to_last_row);

        pieces
}

impl Board {
    pub fn new() -> Board {
        
        let pieces = create_initial_pieces();

        Board {
            pieces, 
            current: Color::White,
            en_passant: None,
        }
    }

    fn get_en_passant(&self, from: Position, to: Position) -> Option<Position> {
        if let Some(piece) = self.get(from) {
            piece.get_en_passant(from, to)
        } else {
            None
        }
    }

    pub fn move_piece(&self, from: Position, to: Position, keep_current: bool, promote: Option<Piece>) -> Result<Board, String> {
        let piece = self.get(from);

        if let Some(piece) = piece {

            let mut pieces = HashMap::new();
            
            // copy the pieces
            for (row , columns) in self.get_pieces() {
                let mut new_row = HashMap::new();
                for (column, piece) in columns {
                    new_row.insert(*column, *piece);
                }
                pieces.insert(*row, new_row);
            }

            // perform the move

            let mut row = pieces.remove(&from.row).unwrap();
            let piece = row.remove(&from.column).unwrap();

            if !row.is_empty() {
                pieces.insert(from.row, row);
            }

            let piece = piece.validate_promotion(to, promote)?;

            if !pieces.contains_key(&to.row) {
                let mut row = HashMap::new();
                row.insert(to.column, piece);
                pieces.insert(to.row, row);
            } else {
                let mut row = pieces.remove(&to.row).unwrap();
                row.remove(&to.column);
                row.insert(to.column, piece);
                pieces.insert(to.row, row);
            }

            Ok(Board {
                pieces,
                current: if keep_current { self.current } else { self.current.opponent() },
                en_passant: self.get_en_passant(from, to),
            })
        } else {
            panic!("There is no piece at position {:?}", from);
        }
    }

    pub fn get_current(&self) -> Color {
        self.current
    }

    pub fn get_pieces(&self) -> &HashMap<Row, HashMap<Column, Piece>> {
        &self.pieces
    }

    pub fn get(&self, position: Position) -> Option<&Piece> {
        if let Some(colum_map) = self.pieces.get(&position.row) {
            colum_map.get(&position.column)
        } else {
            None
        }
    }

    pub fn is_empty(&self, position: Position) -> bool {
        self.get(position) == None
    }

    pub fn can_move(
        &self, 
        from: Position, 
        to: Position) -> bool {

        false
    }

    pub fn get_current_king_position(&self) -> Position {
        for (row, values) in &self.pieces {
            for (column, piece) in values {
                if King(self.current) == *piece {
                    return Position::new(*row, *column);
                }
            }
        }

        panic!("Board in invalid state: no king for current color {:?}", self.current);
    }

    pub fn get_oppenent_king_position(&self) -> Position {
        for (row, values) in &self.pieces {
            for (column, piece) in values {
                if King(self.current.opponent()) == *piece {
                    return Position::new(*row, *column);
                }
            }
        }

        panic!("Board in invalid state: no king for color {:?}", self.current.opponent());
    }

    pub fn is_check(&self) -> bool {
        let current_king_position = self.get_current_king_position();

        let mut opponent_positions = vec![];
        
        for (row, columns) in &self.pieces {
            for (column, piece) in columns {
                if piece.get_color() != self.current {
                    opponent_positions.push(Position::new(*row, *column));
                }
            }
        }

        for piece in opponent_positions {

        }

        false
    }

    fn is_king_around(&self, position: Position, color: Color) -> bool {
        let incs = vec![(1, 1), (1, -1), (-1, 1), (-1, -1), (1, 0), (0, 1)];

        for inc in incs {
            if let Some(to) = position.inc(inc.0, inc.1) {
                if let Some(King(king_color)) = self.get(to) {
                    if *king_color == color {
                        return true;
                    }
                }
            }
        }

        false
    }

    pub fn to_string(&self) -> String {

        let mut lines = vec![];

        let mut columns = " ".to_string();

        for column in FIRST_COLUMN..=LAST_COLUMN {
            columns = format!("{} {} ", columns, column);
        }

        for row in FIRST_ROW..=LAST_ROW {
            let mut line = format!("{}", row);
            for column in FIRST_COLUMN..=LAST_COLUMN {
                if self.pieces.contains_key(&row) 
                    && self.pieces.get(&row).unwrap().contains_key(&column) {
                        let piece = self.pieces.get(&row).unwrap().get(&column).unwrap();
                        line = format!("{} {}", line, piece.to_string());
                } else {
                    line = format!("{} {}", line, "  ");
                }
            }

            lines.push(line);
        }

        let mut result = format!("{}\r\n", columns);

        for line in lines.iter().rev() {
            result = format!("{}{}\r\n", result, line);
        }

        result

        
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_board_correctly() {
        let board = Board::new();

        assert_eq!(White, board.current);
        assert_eq!(Some(&Rook(White)), board.get(Position::new(FIRST_ROW, 'a')));
        assert_eq!(Some(&Knight(White)), board.get(Position::new(FIRST_ROW, 'b')));
        assert_eq!(Some(&Bishop(White)), board.get(Position::new(FIRST_ROW, 'c')));
        assert_eq!(Some(&Queen(White)), board.get(Position::new(FIRST_ROW, 'd')));
        assert_eq!(Some(&King(White)), board.get(Position::new(FIRST_ROW, 'e')));
        assert_eq!(Some(&Bishop(White)), board.get(Position::new(FIRST_ROW, 'f')));
        assert_eq!(Some(&Knight(White)), board.get(Position::new(FIRST_ROW, 'g')));
        assert_eq!(Some(&Rook(White)), board.get(Position::new(FIRST_ROW, 'h')));

        for column in FIRST_COLUMN..=LAST_COLUMN {
            assert_eq!(Some(&Pawn(White)), board.get(Position::new(FIRST_ROW + 1, column)));
        }

        assert_eq!(Some(&Rook(Black)), board.get(Position::new(LAST_ROW, 'a')));
        assert_eq!(Some(&Knight(Black)), board.get(Position::new(LAST_ROW, 'b')));
        assert_eq!(Some(&Bishop(Black)), board.get(Position::new(LAST_ROW, 'c')));
        assert_eq!(Some(&Queen(Black)), board.get(Position::new(LAST_ROW, 'd')));
        assert_eq!(Some(&King(Black)), board.get(Position::new(LAST_ROW, 'e')));
        assert_eq!(Some(&Bishop(Black)), board.get(Position::new(LAST_ROW, 'f')));
        assert_eq!(Some(&Knight(Black)), board.get(Position::new(LAST_ROW, 'g')));
        assert_eq!(Some(&Rook(Black)), board.get(Position::new(LAST_ROW, 'h')));

        for column in FIRST_COLUMN..=LAST_COLUMN {
            assert_eq!(Some(&Pawn(Black)), board.get(Position::new(LAST_ROW - 1, column)));
        }

        for row in FIRST_ROW + 2..=LAST_ROW - 2 {
            for column in FIRST_COLUMN..=LAST_COLUMN {
                assert!(board.is_empty(Position::new(row, column)));
            }
        }        

        assert_eq!(None, board.en_passant);
    }

    #[test]
    fn get_should_return_piece_from_its_position() {
        // set up
        let white_king_position = Position::new(1, 'e');
        
        // SUT
        let board = Board::new();

        let maybe_white_king = board.get(white_king_position);

        // assert
        assert_eq!(&King(White), maybe_white_king.unwrap());
    }

    #[test] 
    fn get_should_return_none_if_position_is_empty() {
        // set up
        let empty_position = Position::new(3, 'e');
        
        // SUT
        let board = Board::new();

        let none = board.get(empty_position);

        // assert
        assert_eq!(None, none);

    }

    #[test]
    fn is_empty_should_return_true_for_empty_position() {
        // set up
        let empty_position = Position::new(3, 'e');

        // SUT
        let board = Board::new();
        let is_empty = board.is_empty(empty_position);

        // assert
        assert!(is_empty);
    }

    #[test]
    fn is_empty_should_return_false_for_nonempty_position() {
        // set up
        let empty_position = Position::new(1, 'e');

        // SUT
        let board = Board::new();
        let is_empty = board.is_empty(empty_position);

        // assert
        assert!(!is_empty);
    }

    #[test]
    fn get_current_king_position_should_return_current_king_position() {
        // SUT
        let board = Board::new();
        let current_king_position = board.get_current_king_position();

        // assert
        assert_eq!(Position::new(1, 'e'), current_king_position);

    }
}

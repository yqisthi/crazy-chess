use crate::model::{game::Board, piece::Color};

pub enum MoveResult {
    Empty,
    Enemy,
    Friendly,
}

pub fn get_tile_by_coordinate(col: i32, row: i32) -> usize {
    return ((row * 8) + col) as usize;
}

pub fn check_target(board: &Board, target: usize, color: Color) -> MoveResult {
    match board[target] {
        None => MoveResult::Empty,
        Some(piece) => {
            if piece.color != color {
                MoveResult::Enemy
            } else {
                MoveResult::Friendly
            }
        }
    }
}

pub fn is_valid_tile(col: i32, row: i32) -> bool {
  return col >= 0 && col < 8 && row >= 0 && row < 8;
}
use crate::model::{board::Board, piece::Color};

pub fn pawn_moves(board: &Board, from: usize, color: Color) -> Vec<usize> {
    let mut moves = Vec::new();

    match color {
        Color::White => {
            let target = from + 8;
            if board[target].is_none() {
                moves.push(target);
            }
        }
        Color::Black => {
            let target = from - 8;
            if board[target].is_none() {
                moves.push(target);
            }
        }
    }

    return moves;
}

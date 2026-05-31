use crate::model::{
    game::Board,
    moves::helpers::{MoveResult, check_target},
    piece::Color,
};

pub fn pawn_moves(board: &Board, from: usize, color: Color) -> Vec<usize> {
    let mut moves = Vec::new();

    match color {
        Color::White => {
            let target = from + 8;
            get_pawn_move(board, &mut moves, target, color)
        }
        Color::Black => {
            let target = from - 8;
            get_pawn_move(board, &mut moves, target, color)
        }
    }

    return moves;
}

fn get_pawn_move(board: &Board, moves: &mut Vec<usize>, target: usize, color: Color) {
    match check_target(board, target, color) {
        MoveResult::Empty => moves.push(target),
        MoveResult::Enemy | MoveResult::Friendly => {},
    }
}

use crate::model::{
    game::Board,
    moves::helpers::{MoveResult, check_target, get_coordinate},
    piece::Color,
};

pub fn rook_moves(board: &Board, from: usize, color: Color) -> Vec<usize> {
    let mut moves = Vec::new();
    let (col, row) = get_coordinate(from);

    // down
    for i in (0..row).rev() {
        let target = ((i * 8) + col) as usize;
        let is_blocked = get_rook_move(board, &mut moves, target, color);
        if is_blocked {
            break;
        }
    }

    // up
    for i in (row + 1)..8 {
        let target = ((i * 8) + col) as usize;
        let is_blocked = get_rook_move(board, &mut moves, target, color);
        if is_blocked {
            break;
        }
    }

    // left
    for i in (0..col).rev() {
        let target = ((8 * row) + i) as usize;
        let is_blocked: bool = get_rook_move(board, &mut moves, target, color);
        if is_blocked {
            break;
        }
    }

    // right
    for i in (col + 1)..8 {
        let target = ((8 * row) + i) as usize;
        let is_blocked = get_rook_move(board, &mut moves, target, color);
        if is_blocked {
            break;
        }
    }

    return moves;
}

fn get_rook_move(board: &Board, moves: &mut Vec<usize>, target: usize, color: Color) -> bool {
    match check_target(board, target, color) {
        MoveResult::Empty => {
            moves.push(target);
            false
        }
        MoveResult::Enemy => {
            moves.push(target);
            true
        }
        MoveResult::Friendly => true,
    }
}

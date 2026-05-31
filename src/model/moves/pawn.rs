use crate::model::{
    game::Board,
    moves::helpers::{
        MoveResult, check_target, get_coordinate, get_tile_by_coordinate, is_valid_tile,
    },
    piece::Color,
};

pub fn pawn_moves(board: &Board, from: usize, color: Color) -> Vec<usize> {
    let mut moves = Vec::new();
    let (col, row) = get_coordinate(from);

    match color {
        Color::White => {
            get_pawn_move(board, &mut moves, from, color);

            let capture_directions = [(col + 1, row + 1), (col + -1, row + 1)];
            get_pawn_capture(board, &mut moves, capture_directions, color)
        }
        Color::Black => {
            get_pawn_move(board, &mut moves, from, color);

            let capture_directions = [(col + 1, row - 1), (col - 1, row - 1)];
            get_pawn_capture(board, &mut moves, capture_directions, color)
        }
    }

    return moves;
}

fn pawn_move_forward(board: &Board, moves: &mut Vec<usize>, target: usize, color: Color) {
    match check_target(board, target, color) {
        MoveResult::Empty => moves.push(target),
        MoveResult::Enemy | MoveResult::Friendly => {}
    }
}

fn get_pawn_move(board: &Board, moves: &mut Vec<usize>, from: usize, color: Color) {
    match color {
        Color::White => {
            let target = from + 8;
            pawn_move_forward(board, moves, target, color);
            if from < 16 {
                pawn_move_forward(board, moves, target + 8, color);
            }
        }
        Color::Black => {
            let target = from - 8;
            pawn_move_forward(board, moves, target, color);
            if from >= 48 {
                pawn_move_forward(board, moves, target - 8, color);
            }
        }
    }
}

fn get_pawn_capture(
    board: &Board,
    moves: &mut Vec<usize>,
    capture_directions: [(i32, i32); 2],
    color: Color,
) {
    for (col, row) in capture_directions {
        if is_valid_tile(col, row) {
            let target = get_tile_by_coordinate(col, row);
            match check_target(board, target, color) {
                MoveResult::Enemy => moves.push(target),
                MoveResult::Empty | MoveResult::Friendly => {}
            }
        }
    }
}

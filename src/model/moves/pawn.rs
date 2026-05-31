use crate::model::{
    game::Board,
    moves::helpers::{MoveResult, check_target, get_tile_by_coordinate, is_valid_tile},
    piece::Color,
};

pub fn pawn_moves(board: &Board, from: usize, color: Color) -> Vec<usize> {
    let mut moves = Vec::new();
    let col = (from % 8) as i32;
    let row = (from / 8) as i32;

    match color {
        Color::White => {
            let target = from + 8;
            get_pawn_move(board, &mut moves, target, color);

            let capture_directions = [(col+1, row+1), (col+-1,row+1)];
            get_pawn_capture(board, &mut moves, capture_directions, color)
        }
        Color::Black => {
            let target = from - 8;
            get_pawn_move(board, &mut moves, target, color);

            let capture_directions = [(col+1, row-1), (col-1, row-1)];
            get_pawn_capture(board, &mut moves, capture_directions, color)
        }
    }

    return moves;
}

fn get_pawn_move(board: &Board, moves: &mut Vec<usize>, target: usize, color: Color) {
    match check_target(board, target, color) {
        MoveResult::Empty => moves.push(target),
        MoveResult::Enemy | MoveResult::Friendly => {}
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

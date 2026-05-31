use crate::model::{
    game::Board,
    moves::helper::{MoveResult, check_target, get_tile_by_coordinate},
    piece::Color,
};

pub fn knight_moves(board: &Board, from: usize, color: Color) -> Vec<usize> {
    let mut moves = Vec::new();
    let col = (from % 8) as i32;
    let row = (from / 8) as i32;

    // top-left
    if is_valid_knight_tile(col - 1, row + 2) {
        let target = get_tile_by_coordinate(col - 1, row + 2);
        get_knight_move(board, &mut moves, target, color);
    }

    // left-top
    if is_valid_knight_tile(col - 2, row + 1) {
        let target = get_tile_by_coordinate(col - 2, row + 1);
        get_knight_move(board, &mut moves, target, color);
    }

    // left-bottom
    if is_valid_knight_tile(col - 2, row - 1) {
        let target = get_tile_by_coordinate(col - 2, row - 1);
        get_knight_move(board, &mut moves, target, color);
    }

    // bottom-left
    if is_valid_knight_tile(col - 1, row - 2) {
        let target = get_tile_by_coordinate(col - 1, row - 2);
        get_knight_move(board, &mut moves, target, color);
    }

    // top-right
    if is_valid_knight_tile(col + 1, row + 2) {
        let target = get_tile_by_coordinate(col + 1, row + 2);
        get_knight_move(board, &mut moves, target, color);
    }

    // right-top
    if is_valid_knight_tile(col + 2, row + 1) {
        let target = get_tile_by_coordinate(col + 2, row + 1);
        get_knight_move(board, &mut moves, target, color);
    }

    // right-bottom
    if is_valid_knight_tile(col + 2, row - 1) {
        let target = get_tile_by_coordinate(col + 2, row - 1);
        get_knight_move(board, &mut moves, target, color);
    }

    // bottom-right
    if is_valid_knight_tile(col + 1, row - 2) {
        let target = get_tile_by_coordinate(col + 1, row - 2);
        get_knight_move(board, &mut moves, target, color);
    }

    return moves;
}

fn is_valid_knight_tile(col: i32, row: i32) -> bool {
    if col < 0 || row < 0 || col > 8 || row > 8 {
        return false;
    }
    return true;
}

fn get_knight_move(board: &Board, moves: &mut Vec<usize>, target: usize, color: Color) {
    match check_target(board, target, color) {
        MoveResult::Empty | MoveResult::Enemy => moves.push(target),
        MoveResult::Friendly => {}
    }
}

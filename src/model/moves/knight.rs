use crate::model::{
    game::Board,
    moves::helpers::{MoveResult, check_target, get_tile_by_coordinate, is_valid_tile},
    piece::Color,
};

pub fn knight_moves(board: &Board, from: usize, color: Color) -> Vec<usize> {
    let mut moves = Vec::new();
    let col = (from % 8) as i32;
    let row = (from / 8) as i32;

    let directions = [
        (1, 2),   // up-right
        (2, 1),   // right-up
        (2, -1),  //right-down
        (1, -2),  //down-right
        (-1, -2), //left-down
        (-2, -1), //down-left
        (-2, 1),  // left-up
        (-1, 2),  // up-left
    ];

    for (dc, dr) in directions {
        let target_col = col + dc;
        let target_row = row + dr;

        if is_valid_tile(target_col, target_row) {
            let target = get_tile_by_coordinate(target_col, target_row);
            get_knight_move(board, &mut moves, target, color);
        }
    }

    return moves;
}


fn get_knight_move(board: &Board, moves: &mut Vec<usize>, target: usize, color: Color) {
    match check_target(board, target, color) {
        MoveResult::Empty | MoveResult::Enemy => moves.push(target),
        MoveResult::Friendly => {}
    }
}

use crate::model::{
    game::Board,
    moves::helpers::{MoveResult, check_target, get_tile_by_coordinate, is_valid_tile},
    piece::Color,
};

pub fn king_moves(board: &Board, from: usize, color: Color) -> Vec<usize> {
    let mut moves = Vec::new();
    let col = (from % 8) as i32;
    let row = (from / 8) as i32;

    let directions = [
        (0, 1), // up
        (1, 1),
        (1, 0), // right
        (1, -1),
        (0, -1), // down
        (-1, -1),
        (-1, 0), // left
        (-1, 1),
    ];

    for (dc, dr) in directions {
        let target_col = col + dc;
        let target_row = row + dr;

        if is_valid_tile(target_col, target_row) {
            let target = get_tile_by_coordinate(target_col, target_row);
            match check_target(board, target, color) {
                MoveResult::Empty | MoveResult::Enemy => moves.push(target),
                MoveResult::Friendly => {}
            }
        }
    }

    return moves;
}

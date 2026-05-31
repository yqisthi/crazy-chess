use crate::model::{
    game::Board,
    moves::helpers::{MoveResult, check_target, get_coordinate, get_tile_by_coordinate, is_valid_tile},
    piece::Color,
};

pub fn bishop_moves(board: &Board, from: usize, color: Color) -> Vec<usize> {
    let mut moves = Vec::new();
    let (col, row) = get_coordinate(from);

    let directions = [(1, 1), (1, -1), (-1, 1), (-1, -1)];

    for (dc, dr) in directions {
        let mut target_col = col + dc;
        let mut target_row = row + dr;

        while is_valid_tile(target_col, target_row) {
            let target: usize = get_tile_by_coordinate(target_col, target_row);
            let is_blocked = get_bishop_move(board, &mut moves, target, color);
            if is_blocked {
              break;
            }
            target_col += dc;
            target_row += dr;
        }
    }

    return moves;
}

fn get_bishop_move(board: &Board, moves: &mut Vec<usize>, target: usize, color: Color) -> bool {
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

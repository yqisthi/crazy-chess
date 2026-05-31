use crate::model::{
    game::Board,
    moves::helper::{MoveResult, check_target, get_tile_by_coordinate},
    piece::Color,
};

pub fn bishop_moves(board: &Board, from: usize, color: Color) -> Vec<usize> {
    let mut moves = Vec::new();
    let col = (from % 8) as i32;
    let row = (from / 8) as i32;

    let directions = [(1, 1), (1, -1), (-1, 1), (-1, -1)];

    for (dc, dr) in directions {
        let mut c = col + dc;
        let mut r = row + dr;

        while c >= 0 && c < 8 && r >= 0 && r < 8 {
            let target: usize = get_tile_by_coordinate(c, r);
            let is_blocked = get_bishop_move(board, &mut moves, target, color);
            if is_blocked {
              break;
            }
            c += dc;
            r += dr;
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

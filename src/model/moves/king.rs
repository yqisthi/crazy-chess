use crate::model::{board::Board, moves::helper::get_tile_by_coordinate};

pub fn king_moves(board: &Board, from: usize) -> Vec<usize> {
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
        let c = col + dc;
        let r = row + dr;

        if c >= 0 && c < 8 && r >= 0 && r < 8 {
            let target = get_tile_by_coordinate(c, r);
            if board[target].is_none() {
                moves.push(target);
            }
        }
    }

    return moves;
}

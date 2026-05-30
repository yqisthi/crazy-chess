use crate::model::{board::Board, moves::helper::get_tile_by_coordinate};

pub fn bishop_moves(board: &Board, from: usize) -> Vec<usize> {
    let mut moves = Vec::new();
    let col = (from % 8) as i32;
    let row = (from / 8) as i32;

    let directions = [(1, 1), (1, -1), (-1, 1), (-1, -1)];

    for (dc, dr) in directions {
        let mut c = col + dc;
        let mut r = row + dr;

        while c >= 0 && c < 8 && r >= 0 && r < 8 {
            let target: usize = get_tile_by_coordinate(c, r);
            if board[target].is_none() {
                moves.push(target);
            } else {
                break;
            }
            c += dc;
            r += dr;
        }
    }

    return moves;
}

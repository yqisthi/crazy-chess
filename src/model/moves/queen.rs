use crate::model::{
    board::Board,
    moves::{bishop::bishop_moves, rook::rook_moves},
};

pub fn queen_moves(board: &Board, from: usize) -> Vec<usize> {
    let mut moves = Vec::new();

    moves.append(&mut rook_moves(board, from));
    moves.append(&mut bishop_moves(board, from));

    return moves;
}

use crate::model::{
    moves::{bishop::bishop_moves, knight::knight_moves, pawn::pawn_moves, rook::rook_moves},
    piece::{Color, Piece, PieceKind},
};

pub type Board = [Option<Piece>; 64];

pub fn initial_board() -> Board {
    let mut board: Board = [None; 64];

    // White Pieces
    for i in 8..16 {
        board[i] = Some(Piece {
            kind: PieceKind::Pawn,
            color: Color::White,
        });
    }

    board[0] = Some(Piece {
        kind: PieceKind::Rook,
        color: Color::White,
    });
    board[1] = Some(Piece {
        kind: PieceKind::Knight,
        color: Color::White,
    });
    board[2] = Some(Piece {
        kind: PieceKind::Bishop,
        color: Color::White,
    });
    board[3] = Some(Piece {
        kind: PieceKind::Queen,
        color: Color::White,
    });
    board[4] = Some(Piece {
        kind: PieceKind::King,
        color: Color::White,
    });
    board[5] = Some(Piece {
        kind: PieceKind::Bishop,
        color: Color::White,
    });
    board[6] = Some(Piece {
        kind: PieceKind::Knight,
        color: Color::White,
    });
    board[7] = Some(Piece {
        kind: PieceKind::Rook,
        color: Color::White,
    });

    // Black Pieces
    board[56] = Some(Piece {
        kind: PieceKind::Rook,
        color: Color::Black,
    });
    board[57] = Some(Piece {
        kind: PieceKind::Knight,
        color: Color::Black,
    });
    board[58] = Some(Piece {
        kind: PieceKind::Bishop,
        color: Color::Black,
    });
    board[59] = Some(Piece {
        kind: PieceKind::Queen,
        color: Color::Black,
    });
    board[60] = Some(Piece {
        kind: PieceKind::King,
        color: Color::Black,
    });
    board[61] = Some(Piece {
        kind: PieceKind::Knight,
        color: Color::Black,
    });
    board[62] = Some(Piece {
        kind: PieceKind::Bishop,
        color: Color::Black,
    });
    board[63] = Some(Piece {
        kind: PieceKind::Rook,
        color: Color::Black,
    });

    for i in 48..56 {
        board[i] = Some(Piece {
            kind: PieceKind::Pawn,
            color: Color::Black,
        });
    }

    return board;
}

pub fn legal_moves(board: &Board, from: usize) -> Vec<usize> {
    match board[from] {
        None => vec![],
        Some(piece) => match piece.kind {
            PieceKind::Pawn => pawn_moves(board, from, piece.color),
            PieceKind::Rook => rook_moves(board, from),
            PieceKind::Knight => knight_moves(board, from),
            PieceKind::Bishop => bishop_moves(board, from),
            PieceKind::Queen => todo!(),
            PieceKind::King => todo!(),
        },
    }
}

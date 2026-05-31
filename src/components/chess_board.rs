use leptos::prelude::*;

use crate::model::{
    game::{execute_move, init_game, legal_moves, update_turn},
    piece::Piece,
};

#[component]
pub fn ChessBoard() -> impl IntoView {
    let (initial_board, initial_turn) = init_game();
    let board = RwSignal::new(initial_board);
    let turn = RwSignal::new(initial_turn);
    let selected = RwSignal::new(None::<usize>);

    let handle_select_piece = move |tile_index: usize, piece: Option<Piece>| {
        let Some(piece) = piece else {
            return;
        };
        if piece.color != turn.get() {
            return;
        }
        selected.set(Some(tile_index))
    };

    let handle_move_piece = move |from: usize, target: usize| {
        if legal_moves(&board.get(), from).contains(&target) {
            if let Some(from) = selected.get() {
                board.update(|b| execute_move(b, from, target));
                selected.set(None);
                turn.update(|c| update_turn(c));
            }
        }
    };

    view! {
        <div class="grid h-80 w-80 grid-cols-8 grid-rows-8 auto-rows-fr sm:h-[500px] sm:w-[500px] border-4 border-black">
            {
                (0..64).rev()
                    .map(|i| {
                        let row = i / 8;
                        let col = i % 8;
                        let is_black = (row + col) % 2 == 0;
                        let bg = if is_black { "bg-amber-800" } else { "bg-amber-100" };
                        let piece = move || board.get()[i];
                        let piece_symbol = move || piece().map(|p| p.unicode());
                        let piece_color = move || piece().map(|p| p.color);

                        view! {
                          <div
                            class=format!("w-full h-full flex items-center justify-center text-3xl {}", bg)
                            on:click=move |_| {
                              match selected.get() {
                                None => handle_select_piece(i, board.get()[i]),
                                Some(from) => {
                                    if from == i {
                                        selected.set(None);
                                    } else if board.get()[i].is_some() && piece_color() == Some(turn.get()){
                                        handle_select_piece(i, board.get()[i]);
                                    }
                                    else {
                                        handle_move_piece(from,  i)
                                    }
                                },
                              }
                            }
                            >
                            <span class=move || {
                                let shadow = if selected.get() == Some(i) {"bg-slate-300"} else {""};
                                format!("flex w-full h-full items-center justify-center cursor-pointer {}", shadow)
                              }
                            >
                              {piece_symbol}
                            </span>
                            <div class=move || {
                              if let Some(from) = selected.get() {
                                  if legal_moves(&board.get(), from).contains(&i) {
                                      return "absolute w-3 h-3 rounded-full bg-black opacity-30".to_string();
                                  }
                              }
                              "hidden".to_string()
                            }/>
                          </div>
                        }
                    })
                    .collect_view()
            }
        </div>
    }
}

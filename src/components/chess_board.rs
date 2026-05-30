use leptos::prelude::*;

use crate::model::{
    board::{initial_board, legal_moves},
    piece::Piece,
};

#[component]
pub fn ChessBoard() -> impl IntoView {
    let board = RwSignal::new(initial_board());
    let selected = RwSignal::new(None::<usize>);

    let handle_select_piece = move |tile_index: usize, piece: Option<Piece>| {
        if piece.is_none() {
            return;
        }
        selected.set(Some(tile_index))
    };

    let handle_move_piece = move |from: usize, target: usize| {
        if legal_moves(&board.get(), from).contains(&target) {
            if let Some(from) = selected.get() {
                board.update(|b| {
                    b[target] = b[from];
                    b[from] = None;
                });
                selected.set(None);
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
                        let piece_symbol = move || board.get()[i].map(|p| p.unicode());

                        view! {
                          <div
                            class=format!("w-full h-full flex items-center justify-center text-3xl {}", bg)
                            on:click=move |_| {
                              match selected.get() {
                                None => handle_select_piece(i, board.get()[i]),
                                Some(from) => {
                                    if from == i {
                                        selected.set(None);
                                    } else if board.get()[i].is_some() {
                                        handle_select_piece(i, board.get()[i]);
                                    }
                                    else {
                                        handle_move_piece(from, i)
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

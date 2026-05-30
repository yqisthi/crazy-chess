use leptos::{attr::selected, prelude::*};

use crate::model::{board::initial_board, piece::Piece};

#[component]
pub fn ChessBoard() -> impl IntoView {
    let board = RwSignal::new(initial_board());
    let selected = RwSignal::new(None::<usize>);

    let handle_select_piece = move |tile_index: usize, option: Option<Piece>| {
        if option.is_none() {
            return;
        } else {
            if selected.get() == Some(tile_index) {
                selected.set(None)
            } else {
                selected.set(Some(tile_index))
            }
        }
    };

    view! {
        <div class="grid h-80 w-80 grid-cols-8 grid-rows-8 auto-rows-fr sm:h-[500px] sm:w-[500px] border-4 border-black">
            {
                (0..64)
                    .map(|i| {
                        let row = i / 8;
                        let col = i % 8;
                        let is_black = (row + col) % 2 == 0;
                        let bg = if is_black { "bg-amber-800" } else { "bg-amber-100" };
                        let piece_symbol = board.get()[i].map(|p| p.unicode());

                        view! {
                          <div
                            class=format!("w-full h-full flex items-center justify-center text-3xl {}", bg)
                            on:click=move |_| handle_select_piece(i, board.get()[i])
                            >
                            <span class=move || {
                                let shadow = if selected.get() == Some(i) {"bg-slate-100"} else {""};
                                format!("flex w-full h-full items-center justify-center cursor-pointer {}", shadow)
                              }
                            >
                              {piece_symbol}
                            </span>
                          </div>
                        }
                    })
                    .collect_view()
            }
        </div>
    }
}

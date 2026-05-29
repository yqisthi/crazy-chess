use leptos::prelude::*;

use crate::model::board::initial_board;

#[component]
pub fn ChessBoard() -> impl IntoView {
    let pieces = initial_board();
    view! {
        <div class="grid h-80 w-80 grid-cols-8 grid-rows-8 auto-rows-fr sm:h-[500px] sm:w-[500px] border-4 border-black">
            {
                (0..64)
                    .map(|i| {
                        let row = i / 8;
                        let col = i % 8;
                        let is_black = (row + col) % 2 == 0;
                        let bg = if is_black { "bg-amber-800" } else { "bg-amber-100" };
                        let piece_symbol = pieces[i].map(|p| p.unicode());

                        view! {
                          <div class=format!("w-full h-full flex items-center justify-center text-3xl {}", bg)>
                            <span class="flex w-full h-full items-center justify-center">
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

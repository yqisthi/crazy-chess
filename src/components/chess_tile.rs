use leptos::prelude::*;

use crate::{
    components::{
        highlight_layer::{LastMoveLayer, SelectedLayer},
        valid_move_dot::ValidMoveDot,
    },
    model::piece::Piece,
};

#[component]
pub fn ChessTile(
    is_black: bool,
    piece: impl Fn() -> Option<Piece> + Send + Sync + 'static,
    is_selected: impl Fn() -> bool + Send + Sync + 'static,
    is_last_move: impl Fn() -> bool + Send + Sync + 'static,
    is_valid_move: impl Fn() -> bool + Send + Sync + 'static,
) -> impl IntoView {
    let bg = if is_black {
        "bg-amber-800"
    } else {
        "bg-amber-100"
    };
    let piece_symbol = move || piece().map(|p| p.unicode());

    view! {
      <div class=format!("relative w-full h-full flex items-center justify-center text-3xl cursor-pointer {}", bg)>
          <LastMoveLayer is_show=is_last_move/>
          <SelectedLayer is_show=is_selected/>
          <span class="relative z-10 flex w-full h-full items-center justify-center">
            {piece_symbol}
          </span>
          <ValidMoveDot is_valid=is_valid_move/>
      </div>
    }
}

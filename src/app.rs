use leptos::prelude::*;

use crate::components::chess_board::ChessBoard;

#[component]
pub fn App() -> impl IntoView {
    view! {
    <main class="flex items-center justify-center min-h-screen">
        <ChessBoard/>
    </main>
    }
}

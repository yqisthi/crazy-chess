use leptos::prelude::mount_to_body;
use leptos::prelude::*;

mod components;

use components::chess_board::ChessBoard;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    view! {
    <main class="flex items-center justify-center min-h-screen">
        <ChessBoard/>
    </main>
    }
}

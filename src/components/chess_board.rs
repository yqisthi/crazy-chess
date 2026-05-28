use leptos::prelude::*;

#[component]
pub fn ChessBoard() -> impl IntoView {
    view! {
        <div class="grid h-80 w-80 grid-cols-8 grid-rows-8 sm:h-[500px] sm:w-[500px]">
            {
                (0..64)
                    .map(|i| {
                        let row = i / 8;
                        let col = i % 8;

                        let is_black = (row + col) % 2 == 0;

                        let bg = if is_black {
                            "bg-black"
                        } else {
                            "bg-white"
                        };

                        view! {
                            <div class=format!("w-full h-full {}", bg)></div>
                        }
                    })
                    .collect_view()
            }
        </div>
    }
}

use leptos::prelude::*;

#[component]
pub fn ValidMoveDot(is_valid: impl Fn() -> bool + Send + Sync + 'static) -> impl IntoView {
    view! {
        <Show when=is_valid>
            <div class="absolute w-3 h-3 rounded-full bg-black opacity-30 z-20"/>
        </Show>
    }
}

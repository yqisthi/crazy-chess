use leptos::prelude::mount_to_body;
use leptos::prelude::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = signal(0);

    view! {
        <button on:click=move |_| set_count.set(count.get()+1)>"Click Me!"</button>
        <p>
        "Count: "
        {move || count}
        </p>
    }
}

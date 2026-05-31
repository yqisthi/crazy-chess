use leptos::{IntoView, attr::global::ClassAttribute, component, control_flow::Show, view};

#[component]
fn HighlightLayer(is_show: impl Fn() -> bool + Send + Sync + 'static, class: &'static str) -> impl IntoView {
    view! {
      <Show when=is_show>
        <div class=move || format!("absolute inset-0 {}", class)/>
      </Show>
    }
}

#[component]
pub fn SelectedLayer(is_show: impl Fn() -> bool + Send + Sync + 'static) -> impl IntoView {
    view! {
        <HighlightLayer is_show=is_show class="bg-yellow-400 opacity-60"/>
    }
}

#[component]
pub fn LastMoveLayer(is_show: impl Fn() -> bool + Send + Sync + 'static) -> impl IntoView {
    view! {
        <HighlightLayer is_show=is_show class="bg-yellow-400 opacity-40"/>
    }
}

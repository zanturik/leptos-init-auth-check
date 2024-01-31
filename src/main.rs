use leptos::*;
use leptos_router::*;

fn main() {
    leptos::mount_to_body(|| leptos::view! { <App /> })
}

#[component]
pub fn App() -> impl IntoView {
    view! {"test"}
}

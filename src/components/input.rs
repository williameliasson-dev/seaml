use leptos::*;

#[component]
pub fn Input(#[prop(optional)] placeholder: String) -> impl IntoView {
    view! { <input placeholder=placeholder class="p-3 rounded-lg"/> }
}

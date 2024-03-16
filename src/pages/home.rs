use crate::components::button::*;
use leptos::*;

#[component]
pub fn Home() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <h2 class="text-4xl">"Welcome to Leptos with Tailwind"</h2>
        <Button on:click=on_click>"Click Me: " {count}</Button>
    }
}

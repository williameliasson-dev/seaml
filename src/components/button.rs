use leptos::*;

#[component]
pub fn Button(children: ChildrenFn) -> impl IntoView {
    view! {
        <button class="bg-secondary rounded-lg p-3 flex justify-center items-center text-gun_metal">
            {children}
        </button>
    }
}

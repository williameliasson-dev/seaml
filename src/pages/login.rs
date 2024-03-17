use crate::{api::auth::login, components::button::*};
use leptos::*;

#[component]
pub fn Login() -> impl IntoView {
    view! {
        <div class="flex flex-col w-full justify-center items-center h-full">
            <div class="p-6">
                <h2 class="text-xl">Hello.</h2>
            </div>
            <Button on:click=move |_| spawn_local(async {
                match login().await {
                    Ok(_) => {}
                    Err(err) => logging::log!("{err}"),
                }
            })>

                <span class="flex gap-2 justify-center items-center">
                    <span>Sign-in with GitHub</span>
                    <img src="assets/github-mark/github-mark.png" class="w-6 h-6"/>
                </span>
            </Button>
        </div>
    }
}

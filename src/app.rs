use crate::pages::home::*;
use crate::pages::login::*;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <head>
            <Stylesheet id="leptos" href="/pkg/seaml.css"/>
            <link rel="preconnect" href="https://fonts.googleapis.com"/>
            <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin/>
            <link
                href="https://fonts.googleapis.com/css2?family=Outfit:wght@100..900&display=swap"
                rel="stylesheet"
            />
            <Title text="Welcome to Leptos"/>
        </head>

        // content for this welcome page
        <Router>
            <main class="bg-primary absolute inset-0">
                <Routes>
                    <Route
                        path="/"
                        view=move || {
                            view! {
                                <Show when=|| false fallback=|| view! { <Login/> }>
                                    <Outlet/>
                                </Show>
                            }
                        }
                    >

                        <Route path="/" view=Home/>
                    </Route>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! { <h1>"Not Found"</h1> }
}

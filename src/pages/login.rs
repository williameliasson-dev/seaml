use crate::components::button::*;

use leptos::*;




#[server(Login)]
async fn login() -> Result<(), ServerFnError>{
    use dotenv::dotenv;
    use oauth2::{
        AuthorizationCode,
        AuthUrl,
        ClientId,
        ClientSecret,
        CsrfToken,
        PkceCodeChallenge,
        RedirectUrl,
        Scope,
        TokenResponse,
        TokenUrl
    };
    use oauth2::basic::BasicClient;
    use oauth2::reqwest::async_http_client;
    dotenv().ok();



    let client_id = std::env::var("GITHUB_CLIENT_ID").expect("GITHUB_CLIENT_ID must be set");
    let secret_key = std::env::var("GITHUB_SECRET").expect("GITHUB_SECRET must be set");


    let client = BasicClient::new(
        ClientId::new(client_id.to_string()),
        Some(ClientSecret::new(secret_key.to_string())),
        AuthUrl::new("https://github.com/login/oauth/authorize".to_string())?,
        Some(TokenUrl::new("https://github.com/login/oauth/access_token".to_string())?)
    ).set_redirect_uri(RedirectUrl::new("http://127.0.0.1:3000/".to_string())?);

    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    let (auth_url, csrf_token) = client
    .authorize_url(CsrfToken::new_random)
    // Set the desired scopes.
    .add_scope(Scope::new("read".to_string()))
    .add_scope(Scope::new("write".to_string()))
    // Set the PKCE code challenge.
    .set_pkce_challenge(pkce_challenge)
    .url();

    println!("Browse to: {}", auth_url);

    let token_result = client
    .exchange_code(AuthorizationCode::new("some authorization code".to_string()))
    // Set the PKCE code verifier.
    .set_pkce_verifier(pkce_verifier)
    .request_async(async_http_client)
    .await?;

    Ok(())

}


#[component]
pub fn Login() -> impl IntoView {
    view! {
        <div class="flex flex-col w-full justify-center items-center h-full">
            <div class="p-6">
                <h2 class="text-xl">Hello.</h2>
            </div>
            <Button on:click=move |_| spawn_local(async {
                logging::log!("clicked");
                login().await;
            })>

                <span class="flex gap-2 justify-center items-center">
                    <span>Sign-in with GitHub</span>
                    <img src="assets/github-mark/github-mark.png" class="w-6 h-6"/>
                </span>
            </Button>
        </div>
    }
}

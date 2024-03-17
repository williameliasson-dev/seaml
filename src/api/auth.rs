
use leptos::{*};


#[server(Login)]
pub async fn login() -> Result<(), ServerFnError>{
    use dotenv::dotenv;
    use oauth2::{
        AuthUrl,
        ClientId,
        ClientSecret,
        CsrfToken,
        RedirectUrl,
        Scope,
        PkceCodeChallenge,
        TokenUrl,
    
    };
    use oauth2::basic::BasicClient;

    dotenv().ok();

    let client_id = std::env::var("GITHUB_CLIENT_ID").expect("GITHUB_CLIENT_ID must be set");
    let secret_key = std::env::var("GITHUB_SECRET").expect("GITHUB_SECRET must be set");
    let env = std::env::var("ENV").expect("ENV must be set");

    let client = BasicClient::new(
        ClientId::new(client_id.to_string()),
        Some(ClientSecret::new(secret_key.to_string())),
        AuthUrl::new("https://github.com/login/oauth/authorize".to_string())?,
        Some(TokenUrl::new("https://github.com/login/oauth/access_token".to_string())?)
    ).set_redirect_uri(RedirectUrl::new("http://127.0.0.1:3000/".to_string())?);

    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
    let state = CsrfToken::new_random;

    let (auth_url, csrf_token) = client
    .authorize_url(state)
    .add_scope(Scope::new("read".to_string()))
    .set_pkce_challenge(pkce_challenge)
    .url();

    // TODO: Save state & pkce_verifier in DB so we can index with state & then get token (:

    leptos_actix::redirect(&auth_url.to_string());
   


    Ok(())
}

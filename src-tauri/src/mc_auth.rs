use oauth2::basic::{BasicClient, BasicTokenType};
use oauth2::reqwest::async_http_client;
use oauth2::{
    AuthType, AuthUrl, AuthorizationCode, ClientId, CsrfToken, EmptyExtraTokenFields, PkceCodeChallenge, RedirectUrl, Scope, StandardTokenResponse, TokenUrl
};
use reqwest::Url;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use tokio::time;

type MSATokenResponse = StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>; // alias for the Microsoft auth token response

const LOCAL_REDIRECT_URI: &str = "http://127.0.0.1:8114/redirect";

#[allow(dead_code)]
#[tokio::main]
async fn msa_auth(client_id:&str) -> Result<MSATokenResponse, Box<dyn std::error::Error>> {

    let client = BasicClient::new(
        ClientId::new(client_id.to_string()),
        None,
        AuthUrl::new("https://login.microsoftonline.com/consumers/oauth2/v2.0/authorize".to_string())?,
        Some(TokenUrl::new(
            "https://login.microsoftonline.com/consumers/oauth2/v2.0/token".to_string(),
        )?),
    )
    // Microsoft requires client_id in URL rather than using Basic authentication.
    .set_auth_type(AuthType::RequestBody)
    // This example will be running its own server at 127.0.0.1:8114.
    // See below for the server implementation.
    .set_redirect_uri(
        RedirectUrl::new(LOCAL_REDIRECT_URI.to_string())
            .expect("Invalid redirect URL"),
    );

    // Microsoft supports Proof Key for Code Exchange (PKCE - https://oauth.net/2/pkce/).
    // Create a PKCE code verifier and SHA-256 encode it as a code challenge.
    let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();

    // Generate the authorization URL to which we'll redirect the user.
    let (authorize_url, _) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("XboxLive.signin offline_access".to_string()))
        .set_pkce_challenge(pkce_code_challenge)
        .url();

    open::that(authorize_url.to_string())?;

    // A very naive implementation of the redirect server.
    let listener = TcpListener::bind("127.0.0.1:8114").await?;

    let res = time::timeout(time::Duration::from_secs(300), async{
        let token;
        loop {
            let (stream, _) = listener.accept().await?;
            stream.readable().await?;
            let mut stream = BufReader::new(stream);
    
            let code;
            {
                let mut request_line = String::new();
                stream.read_line(&mut request_line).await?;
    
                let redirect_url = request_line.split_whitespace().nth(1).unwrap();
                let url = Url::parse(&("http://localhost".to_string() + redirect_url))?;
    
                let (_key, value) = url.query_pairs().find(|(key, _value)| key == "code").unwrap();
                code = AuthorizationCode::new(value.into_owned());
    
            }
    
            let message = "Go back to your terminal :)";
            let response = format!(
                "HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}",
                message.len(),
                message
            );
            stream.write_all(response.as_bytes()).await?;
    
            // Exchange the code with a token.
            token = client
                .exchange_code(code)
                // Send the PKCE code verifier in the token request
                .set_pkce_verifier(pkce_code_verifier)
                .request_async(async_http_client).await?;
        
            break; // stop http server which need to take the response from the browser
        }
    
        Ok::<_, Box<dyn std::error::Error>>(token)
    });

    return res.await?;
}


#[cfg(test)]
mod tests {
    use super::msa_auth;

    #[test]
    fn msa_test() {
        msa_auth(env!("MICROSOFT_CLIENT_ID")).unwrap();
    }
}
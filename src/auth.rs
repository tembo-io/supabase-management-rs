use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AccessTokenResponse {
    pub expires_in: u64,
    pub token_type: String,
    pub access_token: String,
    pub refresh_token: Option<String>,
}

/// Generate a new access token using an existing refresh token.
///
/// Requires the client ID and secret, which are shown when creating the Supabase OAuth2 app.
pub async fn generate_access_token(
    client_id: &str,
    client_secret: &str,
    refresh_token: &str,
) -> Result<AccessTokenResponse, reqwest::Error> {
    let url = "https://api.supabase.com/v1/oauth/token";

    let params = [
        ("grant_type", "refresh_token"),
        ("client_id", client_id),
        ("client_secret", client_secret),
        ("refresh_token", refresh_token),
    ];

    let response = crate::CLIENT
        .post(url)
        .form(&params)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    Ok(response)
}

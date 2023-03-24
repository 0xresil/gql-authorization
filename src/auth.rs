use axum::{
    async_trait,
    extract::{FromRequest, RequestParts},
};

#[derive(Debug)]
pub struct AuthInfo {
    pub token: String,
}

#[async_trait]
impl<B> FromRequest<B> for AuthInfo
where
    B: Send + 'static,
{
    type Rejection = String;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let token = req
            .headers()
            .and_then(|headers| headers.get("Authorization"))
            .and_then(|header_value| header_value.to_str().ok())
            .and_then(|value| value.strip_prefix("Bearer "))
            .map(str::to_owned);

        match token {
            Some(token) => Ok(AuthInfo { token }),
            None => Err("No valid token found in the request headers".to_string()),
        }
    }
}

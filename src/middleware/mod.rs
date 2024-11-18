use axum::{
    body::Body,
    extract::State,
    http::{self, Request},
    middleware::Next,
    response::IntoResponse,
};
use axum_extra::headers::{authorization::Bearer, Authorization, Header};
use jsonwebtoken::errors::ErrorKind;

use crate::{
    api::users::service::IUserService, shared::error::Error, state::AppState,
    utils::jwt::validate_token,
};

pub async fn auth(
    State(state): State<AppState>,
    mut req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, Error> {
    let mut headers = req
        .headers_mut()
        .iter()
        .filter_map(|(header_name, header_value)| {
            if header_name == http::header::AUTHORIZATION {
                return Some(header_value);
            }
            None
        });

    let header: Authorization<Bearer> =
        Authorization::decode(&mut headers).map_err(|_| Error::MissingCredentials)?;
    let token = header.token();
    match validate_token(token) {
        Ok(token_data) => {
            let user = state.user_service.get_user_by_id(token_data.sub).await;
            match user {
                Ok(user) => {
                    req.extensions_mut().insert(user);
                    Ok(next.run(req).await)
                }
                Err(e) => Err(e)?,
            }
        }
        Err(err) => {
            return match err.kind() {
                ErrorKind::ExpiredSignature => Err(Error::ExpiredToken)?,
                ErrorKind::InvalidToken => Err(Error::InvalidToken)?,
                _ => Err(Error::InvalidToken)?,
            };
        }
    }
}

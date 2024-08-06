use axum::{
    body::Body,
    extract::{Json, Request},
    http::{header, HeaderMap, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
};

use crate::{
    handler::MyError,
    models::{Claims, TokenResponse},
};
use axum_extra::{
    headers::{
        authorization::{Basic, Bearer},
        Authorization,
    },
    TypedHeader,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

pub async fn encode_jwt(email: String) -> Result<String, MyError> {
    let claims = Claims {
        username: email,
        exp: (chrono::Utc::now() + chrono::Duration::hours(1)).timestamp() as usize,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("secret".as_ref()),
    )?;

    // let token = match encode(
    //     &Header::default(),
    //     &claims,
    //     &EncodingKey::from_secret("secret".as_ref()),
    // ) {
    //     Ok(tok) => Ok(tok),
    //     Err(e) => {
    //         eprint!("error in generating token {}", e);
    //         return Err(StatusCode::INTERNAL_SERVER_ERROR);
    //     }
    // };
    // let claims = Claims {
    //     username: users.username.clone(),
    //     exp: (chrono::Utc::now() + chrono::Duration::hours(1)).timestamp() as usize,
    // };

    // let token = match encode(
    //     &Header::default(),
    //     &claims,
    //     &EncodingKey::from_secret("secret".as_ref()),
    // ) {
    //     Ok(tok) => tok,
    //     Err(e) => {
    //         eprint!("error in generating token {}", e);
    //         return Err(StatusCode::INTERNAL_SERVER_ERROR);
    //     }
    // };

    // // this could be written as match condition could be eliminate by just using ?
    // let token = encode(&Header::default(), &claims, &EncodingKey::from_secret("secret".as_ref()))?;

    Ok(token)
}

pub fn decode_jwt(jwt: String) -> bool {
    let secret = "secret".to_string();

    let token = decode::<Claims>(
        &jwt,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    );
    match token {
        Ok(_) => true,
        Err(_) => false,
    }
}

// #[axum_macros::debug_handler]
// pub async fn my_middleware(mut req: Request, next: Next) -> Result<Response<Body>, StatusCode> {
//     let auth_header = req.headers_mut().get(header::AUTHORIZATION);
//     let auth_header = match auth_header {
//         Some(header) => header.to_str().map_err(|_| AuthError {
//             message: "Empty header is not allowed".to_string(),
//             status_code: StatusCode::FORBIDDEN,
//         })?,
//         None => {
//             return Err(AuthError {
//                 message: "Please add the JWT token to the header".to_string(),
//                 status_code: StatusCode::FORBIDDEN,
//             })
//         }
//     };

//     let mut header = auth_header.split_whitespace();
//     let (bearer, token) = (header.next(), header.next());
//     let jwt = decode_jwt(token.to_string());

//     // // Check if the Authorization header is present

//     match jwt {
//         Ok(_) => Ok(next.run(request).await),
//         Err(e) => Err(StatusCode::UNAUTHORIZED),
//     }
// }

// #[axum_macros::debug_handler]
// pub async fn my_middleware(
//     TypedHeader(token): TypedHeader<Authorization<Bearer>>,
//     mut request: Request<Body>,
// ) -> Result<Response<Body>, StatusCode> {
//     let bearer_token = token.token().to_owned();
//     let jwt = decode_jwt(bearer_token);

//     match jwt {
//         Ok(_) => Ok(Response::builder().body(Body::from("JWT validation successful")).unwrap()),
//         Err(e) => Err(StatusCode::UNAUTHORIZED),
//     }
// }

// use axum::{
//     middleware::Next,
//     response::Response,
//     http::{Request, StatusCode},
//     extract::TypedHeader,
//     headers::Authorization,
//     headers::authorization::Bearer,
// };
// use axum::body::Body;

pub async fn my_middleware(headers: HeaderMap, request: Request, next: Next) -> Response {
    let token = get_token(&headers).unwrap_or_default().to_string();

    if decode_jwt(token.clone()) {
        return next.run(request).await;
    }
    (StatusCode::UNAUTHORIZED, "Something went wrong...").into_response()
}

fn get_token(headers: &HeaderMap) -> Option<&str> {
    headers
        .get("Authorization")
        .and_then(|value| value.to_str().ok())
        .and_then(|auth_header| auth_header.strip_prefix("Bearer "))
}

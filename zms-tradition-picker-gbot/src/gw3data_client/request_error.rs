use axum::extract::rejection::JsonRejection;
use axum::extract::{FromRequest, Request};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{async_trait, Json};
use thiserror::Error as thisError;
use tracing::info;

use zms_tradition_proto_grpc_types::generated::PickerBotRequest;

#[derive(Debug, thisError)]
pub enum Error {
    #[error(transparent)]
    Validation(#[from] validator::ValidationErrors),

    #[error(transparent)]
    RequestJsonRejection(#[from] JsonRejection),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        info!("into response self is {:?}", self);
        match self {
            Error::Validation(_) => {
                let message = format!("Input validation error: [{self}]").replace('\n', ", ");
                (StatusCode::BAD_REQUEST, message)
            }

            Error::RequestJsonRejection(err) => {
                let message = format!("Axum Json Rejection error: {:?}", err);
                (StatusCode::BAD_REQUEST, message)
            }
        }
        .into_response()
    }
}

#[derive(Debug)]
pub struct ValidatePickerBotRequest(pub PickerBotRequest);

#[async_trait]
impl<S> FromRequest<S> for ValidatePickerBotRequest
where
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = req.into(); //Json::<PickerBotRequest>::from_request(req, state).await?;

        //  todo value.validate()?;
        todo!()
        // Ok(ValidatePickerBotRequest(value))
    }
}

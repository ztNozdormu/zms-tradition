use thiserror::Error;
use tonic::{Code, Status};
use tonic_types::{BadRequest, Help, LocalizedMessage, StatusExt};

pub fn bad_request_errors(new_strategy_picker_symbols: &[String]) -> Result<(), Status> {
    let mut bad_request = BadRequest::new(vec![]);
    if new_strategy_picker_symbols.is_empty() {
        bad_request.add_violation("new_picker_symbols", "name_banks cannot be empty");
    } else if new_strategy_picker_symbols.len() > 1 {
        bad_request.add_violation(
            "new_strategy_picker_symbols",
            "too many banks provided;once request must be less than upper limit of 1",
        );
    }

    if !bad_request.is_empty() {
        let help = Help::with_link("check your symbols list", "https://w3dataserver.com");
        let localized_message =
            LocalizedMessage::new("en-US", "overall validate your symbols list");
        let status = Status::with_error_details_vec(
            Code::InvalidArgument,
            "request contains invalid arguments",
            vec![bad_request.into(), help.into(), localized_message.into()],
        );
        return Err(status);
    }
    Ok(())
}
// TODO business custom error to Status

#[derive(Default, Debug, Error)]
pub enum QueryHaltError {
    #[default]
    #[error("Internal error All Calculations could not proceed")]
    Internal,
    #[error("Join error all calculations could not proceed: {0}")]
    Join(#[from] tokio::task::JoinError),
    #[error("Query data is true but could not serialize events for sending as desired: {0}")]
    QueyrSourceJsonSerializationError(#[from] serde_json::Error),
}

pub struct QueryHaltErrorWrapper(pub QueryHaltError);
impl From<QueryHaltErrorWrapper> for Status {
    fn from(wrapper: QueryHaltErrorWrapper) -> Self {
        match wrapper.0 {
            QueryHaltError::Internal => {
                Status::internal("All Calculations could not proceed")
            }
            QueryHaltError::Join(e) => Status::internal(format!(
                "Join error all calculations could not proceed: {}",
                e
            )),
            QueryHaltError::QueyrSourceJsonSerializationError(e) => {
                Status::internal(format!(
                    "Drive Deposits SEND_CAL_EVENTS is true but could not serialize events for sending as desired: {}",
                    e
                ))
            }
        }
    }
}

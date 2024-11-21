use axum::{async_trait, Json};
use mockall::automock;
use std::env::var;
use tracing::{debug, debug_span, error, info};

use app_error::Error as AppError;
use zms_tradition_proto_grpc_types::generated::{
    w3data_service_client::W3dataServiceClient,
    GateWayPickerRequest as GrpcGateWayPickerRequest,
    GateWayPickerResponse as GrpcGateWayPickerResponse,
};
use zms_tradition_rest_types::rest_types::{
    GateWayPickerRequest as RestGateWayPickerRequest,
    GateWayPickerResponse as RestGateWayPickerResponse,
};
use request_error::ValidateGateWayPickerRequest;
use tonic::transport::Channel;

mod app_error;
mod request_error;

#[automock]
#[async_trait]
pub trait W3dataClient {
    async fn query_picker_symbols_request(
        mut self,
        request: tonic::Request<GrpcGateWayPickerRequest>,
    ) -> Result<tonic::Response<GrpcGateWayPickerResponse>, tonic::Status>;
}

#[async_trait]
impl W3dataClient for W3dataServiceClient<Channel> {
    //  Differentiating the trait method name from the method being called on self helps avoid confusion and potential infinite recursion
    async fn query_picker_symbols_request(
        mut self,
        request: tonic::Request<GrpcGateWayPickerRequest>,
    ) -> Result<tonic::Response<GrpcGateWayPickerResponse>, tonic::Status> {
        // Call the actual gRPC method instead of recursively calling itself: using associated style
        // W3dataServiceClient::query_piker_symbols(&mut self, request).await
        self.query_picker_symbols(request).await
    }
}

// #[debug_handler]
pub async fn query_picker_symbols(
    ValidateGateWayPickerRequest(rest_delta_request): ValidateGateWayPickerRequest,
) -> Result<Json<RestGateWayPickerResponse>, AppError> {
    let span = debug_span!("query_piker_symbols");
    span.in_scope(|| {
        debug!(
            "query_piker_symbols request incoming is : {:#?}",
            rest_delta_request
        )
    });

    // for docker compose dns GRPC_SERVER_ADDRESS=http://zms-tradition-grpc-server:50052
    let grpc_server_address =
        var("GRPC_SERVER_ADDRESS").unwrap_or_else(|_| "http://[::]:50052".to_string());
    info!("grpc_server_address is: {}", grpc_server_address);
    let client = W3dataServiceClient::connect(grpc_server_address)
        .await
        .inspect_err(|err| {
            span.in_scope(|| error!("grpc client connection error: {:?}", err));
        })?;
    query_piker_symbols_with_client(rest_delta_request, client).await
}

pub async fn query_piker_symbols_with_client(
    rest_delta_request: RestGateWayPickerRequest,
    client: impl W3dataClient,
) -> Result<Json<RestGateWayPickerResponse>, AppError> {
    let span = debug_span!("query_piker_symbols_with_client");
    span.in_scope(|| {
        debug!(
            "query_piker_symbols request incoming is : {:#?}",
            rest_delta_request
        )
    });

    let grpc_delta_request = span.in_scope(|| rest_delta_request.into());

    let grpc_request = tonic::Request::new(grpc_delta_request);
    let grpc_response = client.query_picker_symbols_request(grpc_request).await?;

    span.in_scope(|| {
        info!(
            "grpc response in client for rest server acting as gateway is : {:#?}",
            grpc_response
        );
        let grpc_delta_response = grpc_response.into_inner();
        let rest_response = grpc_delta_response.into();
        info!(
            "rest response in client for rest server acting as gateway is : {:#?}",
            rest_response
        );
        Ok(Json(rest_response))
    })
}

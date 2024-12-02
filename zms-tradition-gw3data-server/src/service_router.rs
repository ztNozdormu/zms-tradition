use tonic::{transport::server::Router, transport::Server};
use tonic_reflection::server::Builder;
use tracing::{info, info_span, instrument};

use zms_tradition_proto_grpc_types::generated::{
    w3data_service_server::W3dataServiceServer, FILE_DESCRIPTOR_SET,
};

use crate::picker::PickerBus;

#[instrument]
pub async fn app() -> Result<Router, Box<dyn std::error::Error>> {
    let server_reflection = Builder::configure()
        .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
        .build_v1alpha()?;
    info!("reflection built using v1alpha since Postman gRPC still uses it as of tonic 0.12.2");

    let delta: PickerBus = PickerBus;

    let builder = Server::builder()
        .trace_fn(|_| info_span!("gw3data_server"))
        .add_service(server_reflection)
        .add_service(W3dataServiceServer::new(delta));
    Ok(builder)
}

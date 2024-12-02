use tonic::{async_trait, Request, Response, Status};
use tracing::{debug, error, info_span};

use zms_tradition_proto_grpc_types::generated::{
    w3data_service_server::W3dataService, GateWayPickerRequest, GateWayPickerResponse,
    PickerBotRequest, PickerBotResponse,
};

mod grpc_status_handler;
mod query_service;

pub struct PickerBus;

#[async_trait]
impl W3dataService for PickerBus {
    async fn query_picker_symbols(
        &self,
        request: Request<GateWayPickerRequest>,
    ) -> Result<Response<GateWayPickerResponse>, Status> {
        info_span!("grpc_picker_symbols");
        debug!("grpc_picker_symbols request incoming is : {:#?}", request);

        let picker_symbols_request = request.into_inner();

        let response = query_service::query_picker_symbols(picker_symbols_request)
            .await
            .inspect_err(|err| error!("building response errors : {:?}", err))?;

        Ok(Response::new(response))
    }

    async fn save_picker_bot_symbols(
        &self,
        request: tonic::Request<PickerBotRequest>,
    ) -> Result<Response<PickerBotResponse>, Status> {
        info_span!("grpc_picker_symbols");
        debug!("query picker symbols request incoming is : {:#?}", request);
        todo!()
    }
}

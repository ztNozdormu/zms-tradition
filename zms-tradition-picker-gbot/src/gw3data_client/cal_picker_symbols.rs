use tonic::Status;
use tracing::{debug, error, info};
use zms_tradition_proto_grpc_types::generated::{
    w3data_service_client::W3dataServiceClient, 
    PickerBotRequest as GrpcPickerBotRequest,
    PickerBotResponse as GrpcPickerBotResponse,
};

// TODO picker symbols logic impl picker_bot_request Replace config

pub async fn picker_symbols_calculate_by_factory(
    picker_bot_request: GrpcPickerBotRequest,
) -> Result<GrpcPickerBotResponse, Status> {
    info!("picker symbols push is : {picker_bot_request:?}");

    // convert response fom calculator GrpcPickerBotResponse to grpc GrpcPickerBotResponse
    // let grpc_resp: GrpcPickerBotResponse = cal_resp.into();
    // debug!("grpc response: {:?}", grpc_resp);

    // Ok(grpc_resp)
    todo!()
}
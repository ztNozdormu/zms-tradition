use tracing::{info, info_span};
// grpc data to rest data

use zms_tradition_rest_types::rest_types::{
    GateWayPickerResponse as RestGateWayPickerResponse, PickerSymbol as RestPickerSymbol,
    ProcessingError as RestProcessingError,
};

use crate::generated::{
    GateWayPickerResponse as GrpcGateWayPickerResponse, PickerSymbol as GrpcPickerSymbol,
    ProcessingError as GrpcProcessingError,
};

impl From<GrpcProcessingError> for RestProcessingError {
    fn from(grpc: GrpcProcessingError) -> Self {
        Self {
            uuid: grpc.uuid,
            message: grpc.message,
        }
    }
}

impl From<GrpcPickerSymbol> for RestPickerSymbol {
    fn from(grpc: GrpcPickerSymbol) -> Self {
        Self {
            symbol: grpc.symbol,
        }
    }
}

impl From<GrpcGateWayPickerResponse> for RestGateWayPickerResponse {
    fn from(grpc: GrpcGateWayPickerResponse) -> Self {
        let rest: RestGateWayPickerResponse = Self {
            strategy_type: grpc.strategy_type,
            picker_symbols: grpc.picker_symbols.into_iter().map(|x| x.into()).collect(),
        };
        info_span!("grpc_rest_response::From::grpc").in_scope(|| {
            info!(
                "In From: grpc response converted to rest response: {:?}",
                rest
            )
        });
        rest
    }
}

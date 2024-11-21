use heck::ToShoutySnakeCase;
use tracing::{debug, info, info_span};
// rest data to grpc data

use zms_tradition_rest_types::rest_types::{
    GateWayPickerRequest as RestGateWayPickerRequest, PickerSymbol as RestPickerSymbol,
};

use crate::generated::{
    PickerSymbol as GrpcPickerSymbol, GateWayPickerRequest as GrpcGateWayPickerRequest,
};

// RestPickerSymbol to GrpcPickerSymbol
impl From<RestPickerSymbol> for GrpcPickerSymbol {
    fn from(rest: RestPickerSymbol) -> Self {
        debug!(
            "rest.symbol.to_shouty_snake_case() is {}",
             rest.symbol.to_shouty_snake_case()
        );
        GrpcPickerSymbol {
            symbol: rest.symbol,
        }
    }
}
// RestGateWayPickerRequest to  GrpcGateWayPickerRequest
impl From<RestGateWayPickerRequest> for GrpcGateWayPickerRequest {
    fn from(rest: RestGateWayPickerRequest) -> Self {
        let grpc: GrpcGateWayPickerRequest = Self {
            strategy_type: rest.strategy_type,
        };
        info_span!("rest_grpc_request::From::rest")
            .in_scope(|| info!("rest request converted to grpc request: {:?}", grpc));
        grpc
    }
}

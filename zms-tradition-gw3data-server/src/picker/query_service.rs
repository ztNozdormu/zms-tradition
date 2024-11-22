use tonic::Status;
use tracing::{debug, error, info};

use zms_tradition_proto_grpc_types::generated::{
    GateWayPickerRequest as GrpcGateWayPickerRequest,
    GateWayPickerResponse as GrpcGateWayPickerResponse, PickerSymbol as GrpcPickerSymbol,
};

use super::grpc_status_handler;

pub async fn query_picker_symbols(
    pikcer_symbols_request: GrpcGateWayPickerRequest,
) -> Result<GrpcGateWayPickerResponse, Status> {
    let strategy = pikcer_symbols_request.strategy_type;

    info!(
        "query strategy picker symbols incoming is : {:?}",
        &strategy
    );
    grpc_status_handler::bad_request_errors(&[strategy.clone()])
        .inspect_err(|err| error!("new_banks checking at the grpc level errors : {:?}", err))?;

    let mock_data = build_mock_picker_symbols();
    // convert response fom calculator CalculatePortfolioResponse to grpc CalculatePortfolioResponse
    let response = GrpcGateWayPickerResponse {
        strategy_type: strategy,
        picker_symbols: mock_data,
    };
    debug!("grpc response: {:?}", response);

    Ok(response)
}

// test mock data
pub fn build_mock_picker_symbols() -> Vec<GrpcPickerSymbol> {
    let mut mock_picker_symbols = Vec::with_capacity(5);
    mock_picker_symbols.push(GrpcPickerSymbol {
        symbol: "btcusdt".to_string(),
    });
    mock_picker_symbols.push(GrpcPickerSymbol {
        symbol: "solusdt".to_string(),
    });
    mock_picker_symbols.push(GrpcPickerSymbol {
        symbol: "ethusdt".to_string(),
    });
    mock_picker_symbols.push(GrpcPickerSymbol {
        symbol: "pepeusdt".to_string(),
    });
    mock_picker_symbols.push(GrpcPickerSymbol {
        symbol: "wifusdt".to_string(),
    });
    info!("mock_picker_symbols is : {:?}", mock_picker_symbols);
    mock_picker_symbols
}

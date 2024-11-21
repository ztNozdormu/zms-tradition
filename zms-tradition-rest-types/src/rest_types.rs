use serde::{Deserialize, Serialize};

// Request sections
#[derive(Default, Debug, Deserialize)]
pub struct GateWayPickerRequest {
    pub strategy_type: String,
}

// picker symbols data
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct PickerSymbol {
    pub symbol: String,
}



// Response sections

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct GateWayPickerResponse {
    pub strategy_type: String,
    pub picker_symbols: Vec<PickerSymbol>,
}


#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct ProcessingError {
    pub uuid: String,
    pub message: String,
}

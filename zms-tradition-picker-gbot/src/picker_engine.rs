use tracing::info;

use crate::gw3data_client;

#[derive(Debug)]
pub struct PickerSymbolsConfig {
    // TODO define
    // config
    // other data
}

// TODO picker symbols logic impl picker_bot_request Replace config

pub fn picker_symbols_calculate_by_factory() {
    // TODO gw3data_client::load_config();
    let cal_config = PickerSymbolsConfig {};
    info!("picker symbols cal config is : {cal_config:?}");
    // call gw3data_client::save_piker_symbols_with_client()
    // todo!()
    info!("picker symbols cal logic todo impl da da da ");
}

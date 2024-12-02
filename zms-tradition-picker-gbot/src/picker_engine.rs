use barter_xchange::exchange::binance::{
    api::Binance,
    futures::{general::FuturesGeneral, market::FuturesMarket},
    model::{KlineSummaries, Symbol},
};
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

// get symbols list
async fn get_symbols() -> Vec<Symbol> {
    let general: FuturesGeneral = Binance::new(None, None);
    let symbols = match general.get_symbol_infos().await {
        Ok(symbols) => symbols,
        Err(e) => {
            info!("get_symbols() Error: {}", e);
            Vec::new()
        }
    };
    symbols
}
// get symnols candles
async fn market_data_feed(symbol: String) {
    let market: FuturesMarket = Binance::new(None, None);

    match market.get_klines(symbol, "5m", None, None, None).await {
        Ok(KlineSummaries::AllKlineSummaries(answer)) => println!(
            "First kline: {:?} kline count : {}",
            answer[0],
            answer.len()
        ),
        Err(e) => println!("Error: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_symbols() {
        let symbols = get_symbols().await;
        println!("symbols{:?}", symbols.get(0))
    }
    #[tokio::test]
    async fn test_market_data_feed() {
        market_data_feed("btcusdt".to_string()).await;
    }
}

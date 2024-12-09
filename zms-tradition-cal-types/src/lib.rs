mod macros_polars;
use barter_xchange::exchange::binance::{
    api::Binance,
    futures::{general::FuturesGeneral, market::FuturesMarket},
    model::{KlineSummaries, KlineSummary, Symbol},
};
use polars_talib::prelude::NamedFrom;
use polars_talib::{frame::DataFrame, series::Series};
use tracing::info;

pub mod cal_types;
pub mod calculation_error;
pub mod trend_algorithm;

// get symbols list
pub async fn get_symbols() -> Vec<Symbol> {
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

// get symnol candles
pub async fn market_data_feed(symbol: &str) -> Vec<KlineSummary> {
    let market: FuturesMarket = Binance::new(None, None);

    match market.get_klines(symbol, "5m", None, None, None).await {
        Ok(KlineSummaries::AllKlineSummaries(kline_summaries)) => kline_summaries,
        Err(e) => {
            info!("get_symbols() Error: {}", e);
            Vec::new()
        }
    }
}

// candle to dataframe
pub async fn candle_to_dataframe(klines: &Vec<KlineSummary>) -> DataFrame {
    struct_vec_to_dataframe!(klines, open, high, low, close, volume, close_time)
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
        market_data_feed("btcusdt").await;
    }

    #[tokio::test]
    async fn test_candle_to_dataframe() {
        let candles = market_data_feed("btcusdt").await;
        let dataframe = candle_to_dataframe(&candles).await;
        println!("dataframe {:?}", dataframe);
    }
}

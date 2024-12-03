use barter_xchange::exchange::binance::model::KlineSummary;
use polars_talib::*;
use polars_talib::overlap::ema;
use polars_talib::TimePeriodKwargs;
// use polars_ta_extension::*;
use crate::{calculation_error::StiffnessError, candle_to_dataframe};

/// calculate indicators
pub async fn calculate_indicators(klines: &Vec<KlineSummary>) -> Result<DataFrame,StiffnessError> {

     let df = candle_to_dataframe(klines).await;

      // 计算 ATR（14 周期）
      df = df.ta_atr("high", "low", "close", 14, "atr_real")?;

      // 计算 ATR 除以 (close + open) / 2
      let atr_scaled = df
          .lazy()
          .with_column(
              (col("atr_real") / ((col("close") + col("open")) / lit(2.0))).alias("atr"),
          )
          .collect()?;
    
      // 计算 ATR 上下界
      let n_factor = 3.25;
      let atr_bounds = atr_scaled
          .lazy()
          .with_column((col("close") + lit(n_factor) * col("atr_real")).alias("atr_up"))
          .with_column((col("close") - lit(n_factor) * col("atr_real")).alias("atr_down"))
          .collect()?;
  
      // 计算 +DI 和 -DI（14 周期）
      let atr_di = atr_bounds
          .ta_plus_di("high", "low", "close", 14, "plus_DI")?
          .ta_minus_di("high", "low", "close", 14, "minus_DI")?;
  
      // 计算 ADX（14 周期）
      let atr_adx = atr_di.ta_adx("high", "low", "close", 14, "adx")?;
  
      // 计算 ADX 趋势
      let adx_trend = atr_adx
          .lazy()
          .with_column(
              (col("adx").rolling_mean(3, None, None)
                  / col("adx").rolling_mean(14, None, None))
              .alias("adx_trend"),
          )
          .collect()?;
  
      // 计算动量指标
      let momentum_df = adx_trend
          .lazy()
          .with_column(((col("close") / col("close").shift(21) - lit(1.0)).alias("momentum_21")))
          .with_column(((col("close") / col("close").shift(58) - lit(1.0)).alias("momentum_58")))
          .with_column(((col("close") / col("close").shift(144) - lit(1.0)).alias("momentum_144")))
          .collect()?;
  
      // 打印最终结果
      println!("{:?}", momentum_df);
  
      Ok(df)
  }
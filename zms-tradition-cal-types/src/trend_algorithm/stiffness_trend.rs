use barter_xchange::exchange::binance::model::KlineSummary;
use error::PolarsResult;
use frame::DataFrame;
use polars_talib::*;
use polars_talib::volatility::*;
use polars_talib::datatypes::DataType;
use prelude::{apply_multiple, col, lit, map_multiple, GetOutput, IntoLazy};
use time::ClosedWindow;

use crate::{calculation_error::StiffnessError, candle_to_dataframe};

// calculate indicators
pub async fn calculate_indicators(klines: &Vec<KlineSummary>) -> Result<DataFrame,StiffnessError> {

     let df = candle_to_dataframe(klines).await;
     
      // 计算 ATR（14 周期）
      df = calculate_atr(&df,14).expect("atr计算失败");//df.ta_atr("high", "low", "close", 14, "atr_real")?;

      // 计算 ATR 除以 (close + open) / 2
      let atr_scaled = df
          .lazy()
          .with_column(
              (col("atr_real") / ((col("close") + col("open")) / lit(2))).alias("atr"),
          )
          .collect().expect("计算 ATR 除以 (close + open) / 2发生错误");
    
      // 计算 ATR 上下界
      let n_factor = 3.25;
      let atr_bounds = atr_scaled
          .lazy()
          .with_column((col("close") + lit(n_factor) * col("atr_real")).alias("atr_up"))
          .with_column((col("close") - lit(n_factor) * col("atr_real")).alias("atr_down"))
          .collect().expect("计算 ATR 上下界发生错误");
  
      // 计算 +DI 和 -DI（14 周期）
    //   let atr_di = atr_bounds
    //       .ta_plus_di("high", "low", "close", 14, "plus_DI")?
    //       .ta_minus_di("high", "low", "close", 14, "minus_DI")?;
  
      // 计算 ADX（14 周期）
    //   let atr_adx = atr_di.ta_adx("high", "low", "close", 14, "adx")?;
  
      // 计算 ADX 趋势
    //   let adx_trend = atr_adx
    //       .lazy()
    //       .with_column(
    //           (col("adx").rolling(3, RollingOptions {
    //             min_periods: Some(3), // 窗口大小必须达到 3
    //             window_size: 3,       // 窗口大小
    //             weights: None,        // 不使用权重
    //             center: false,        // 不居中
    //             by: None,             // 没有分组键
    //             closed_window: ClosedWindow::Both, // 窗口两端都包括
    //         })
    //               / col("adx").rolling_mean(14, None, None))
    //           .alias("adx_trend"),
    //       )
    //       .collect().expect("计算 ADX 趋势发生错误");
  
      // 计算动量指标
      let momentum_df = adx_trend
          .lazy()
          .with_column(((col("close") / col("close").shift(polars_talib::prelude::Expr::Nth(21)) - lit(1.0)).alias("momentum_21")))
          .with_column(((col("close") / col("close").shift(polars_talib::prelude::Expr::Nth(58)) - lit(1.0)).alias("momentum_58")))
          .with_column(((col("close") / col("close").shift(polars_talib::prelude::Expr::Nth(144)) - lit(1.0)).alias("momentum_144")))
          .collect().expect("计算动量指标发生错误");
  
      // 打印最终结果
      println!("{:?}", momentum_df);
  
      Ok(df)
  }

// 替换的 ATR 计算函数，整合到 Polars 表达式中

fn calculate_atr(df: &DataFrame, high: &str, low: &str, close: &str, period: i32) -> PolarsResult<DataFrame> {
    let kwargs = ATRKwargs {timeperiod: period };

    let result_df = df
        .lazy()
        .with_column(
            map_multiple(
                vec![col(close), col(high), col(low)],
                move |cols| {
                    // `cols` 是一个 Vec<Series>
                    let atr_series = atr(&cols, kwargs.clone()).expect("atr计算发生错误");
                    Ok(Some(atr_series))
                },
                GetOutput::from_type(DataType::Float64),
            )
            .alias("atr_real"),
        )
        .collect()?;

    Ok(result_df)
}
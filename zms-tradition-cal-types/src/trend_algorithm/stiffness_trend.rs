
use barter_xchange::exchange::binance::model::KlineSummary;
use error::PolarsResult;
use lazy::dsl::*;
use momentum::{adx, minus_di, plus_di};
use polars_talib::volatility::*;
use polars_talib::*;
use prelude::{col, lit, Column, DataFrame, GetOutput, IntoLazy};
use series::Series;

use crate::{calculation_error::StiffnessError, candle_to_dataframe};

// calculate indicators
pub async fn calculate_indicators(klines: &Vec<KlineSummary>) -> Result<DataFrame, StiffnessError> {
    
    let df = candle_to_dataframe(klines).await;

    // 计算 ATR（14 周期）
    // df = calculate_atr(&df, 14).expect("atr计算失败"); //df.ta_atr("high", "low", "close", 14, "atr_real")?;
    let atr_real= calculate_atr(&df, 14).expect("atr计算失败"); 

    // 计算 ATR 除以 (close + open) / 2
    let atr_scaled = atr_real
        .lazy()
        .with_column((col("atr_real") / ((col("close") + col("open")) / lit(2))).alias("atr"))
        .collect()
        .expect("计算 ATR 除以 (close + open) / 2发生错误");

    // 计算 ATR 上下界
    let n_factor = 3.25;
    let atr_bounds = atr_scaled
        .lazy()
        .with_column((col("close") + lit(n_factor) * col("atr_real")).alias("atr_up"))
        .with_column((col("close") - lit(n_factor) * col("atr_real")).alias("atr_down"))
        .collect()
        .expect("计算 ATR 上下界发生错误");
   // dataframe['adx_trend'] = dataframe['adx'].rolling(3).mean() / dataframe['adx'].rolling(14).mean()
    // 计算 +DI 和 -DI（14 周期）
    // let atr_di = atr_bounds
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
    // let momentum_df = adx_trend
    //     .lazy()
    //     .with_column(
    //         ((col("close") / col("close").shift(polars_talib::prelude::Expr::Nth(21)) - lit(1.0))
    //             .alias("momentum_21")),
    //     )
    //     .with_column(
    //         ((col("close") / col("close").shift(polars_talib::prelude::Expr::Nth(58)) - lit(1.0))
    //             .alias("momentum_58")),
    //     )
    //     .with_column(
    //         ((col("close") / col("close").shift(polars_talib::prelude::Expr::Nth(144)) - lit(1.0))
    //             .alias("momentum_144")),
    //     )
    //     .collect()
    //     .expect("计算动量指标发生错误");

    // 打印最终结果
    // println!("{:?}", momentum_df);
    todo!()
    // Ok(atr_real)
}

// 替换的 ATR 计算函数，整合到 Polars 表达式中

// fn calculate_atr(df: &DataFrame, period: i32) -> PolarsResult<DataFrame> {
//     let kwargs = ATRKwargs { timeperiod: period };

//     let close = df.column("close")?;
//     let high = df.column("high")?;
//     let low = df.column("low")?;
//     // 将 Column 转换为 Series
//     let close_series: Series = close.as_materialized_series().clone();
//     let high_series: Series = high.as_materialized_series().clone();
//     let low_series: Series = low.as_materialized_series().clone();
//     let mut atr_real_series = atr(&[close_series, high_series, low_series], kwargs)?;
    
//     atr_real_series.rename("atr_real".into());

//     df.clone().with_column(atr_real_series)?;

//     Ok(df.clone())
// }


/**
 * Helper function to extract the fields from the struct series
 * use example:
 *  let (close, high, low) = extract_struct_fields(series)?;
 *     atr(&[high, low, close], kwargs)
 */ 
fn extract_struct_fields(series: &Column) -> PolarsResult<(Series, Series, Series)> {
    let struct_series = series.struct_()?;
    
    let close = struct_series.field_by_name("close")?.clone();
    let high = struct_series.field_by_name("high")?.clone();
    let low = struct_series.field_by_name("low")?.clone();

    Ok((close, high, low))
}

/**
 * calculate atr_real indicator
 */
fn calculate_atr(df: &DataFrame, period: i32) -> PolarsResult<DataFrame> {

   let kwargs = ATRKwargs { timeperiod: period };
   // 将指定的列转换为 Struct 并使用自定义函数计算
   let result = df.clone()
   .lazy()
   .with_column(
       // 将 col1 和 col2 转换为 Struct
       as_struct((&[col("close"), col("high"), col("low")]).to_vec())
           .alias("chl_struct")
   )
   .with_column(
       // 对 Struct 应用自定义函数 计算 atr_real
       col("chl_struct")
           .apply(move |series| {
               let (close, high, low) = extract_struct_fields(&series)?;
               let res: Series = atr(&[close,high,low],kwargs.clone())?;
               Ok(Some(polars_talib::prelude::Column::Series(res)))
           }, GetOutput::same_type())
           .alias("atr_real")
   )
   .collect()?;

    Ok(result)
}
/**
 * calculate adx_trend indicator
 */
fn calculate_adx_trend(df: &DataFrame, period: i32) -> PolarsResult<DataFrame> {

 
    // 将指定的列转换为 Struct 并使用自定义函数计算
    let result = df.clone()
    .lazy()
    .with_column(
        // 将 col1 和 col2 转换为 Struct
        as_struct((&[col("close"), col("high"), col("low")]).to_vec())
            .alias("chl_struct")
    )
    .with_column(
        // 计算plus_DI
        col("chl_struct")
            .apply(move |series| {
                let (close, high, low) = extract_struct_fields(&series)?;
                let res: Series = plus_di(&[high,low,close],TimePeriodKwargs { timeperiod: period })?;
                Ok(Some(polars_talib::prelude::Column::Series(res)))
            }, GetOutput::same_type())
            .alias("plus_DI")
    ).with_column(
        // 计算 minus_DI
        col("chl_struct")
            .apply(move |series| {
                let (close, high, low) = extract_struct_fields(&series)?;
                let res: Series = minus_di(&[high,low,close],TimePeriodKwargs { timeperiod: period })?;
                Ok(Some(polars_talib::prelude::Column::Series(res)))
            }, GetOutput::same_type())
            .alias("minus_DI")
    ).with_column(
        // 计算 adx
        col("chl_struct")
            .apply(move |series| {
                let (close, high, low) = extract_struct_fields(&series)?;
                let res: Series = adx(&[high,low,close],TimePeriodKwargs { timeperiod: period })?;
                Ok(Some(polars_talib::prelude::Column::Series(res)))
            }, GetOutput::same_type())
            .alias("adx")
    )
    .collect()?;
 
     Ok(result)
 }
// /// 自定义的 ATR 计算函数，接收 Expr 参数，生成新的列


#[cfg(test)]
mod tests {
    use crate::{candle_to_dataframe, market_data_feed, trend_algorithm::stiffness_trend::*};


  

    #[tokio::test]
    async fn test_dataframe_atr() {
        let candles = market_data_feed("btcusdt").await;
        let df = candle_to_dataframe(&candles).await;
        let df_art_real = calculate_atr(&df,14);
        println!("df_atr_real_1 {:?}", df_art_real);
    }

    #[tokio::test]
    async fn test_dataframe_adx() {
        let candles = market_data_feed("btcusdt").await;
        let df = candle_to_dataframe(&candles).await;
        let df_art_real = calculate_atr(&df,14);
        println!("df_atr_real {:?}", df_art_real);
    }
}
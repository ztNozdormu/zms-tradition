// 将结构体的向量转换为polars DataFrame
#[macro_export]
macro_rules! struct_vec_to_dataframe {
    ($data:expr, $($field:ident),*) => {{
        let mut columns = Vec::new();

        $(
            let series = Series::new(stringify!($field).into(), $data.iter().map(|s| s.$field.clone()).collect::<Vec<_>>());
            columns.push(polars_talib::prelude::Column::Series(series));
        )*

        DataFrame::new(columns).expect("Failed to create DataFrame")
    }};
}

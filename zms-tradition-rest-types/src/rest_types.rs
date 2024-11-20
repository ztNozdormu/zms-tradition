use std::str::FromStr;

use chrono::NaiveDate;
use chrono_tz::Tz;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use strum_macros::EnumString;
use validator::Validate;
use validator::ValidationError;

// Request sections
#[derive(Default, Debug, Deserialize, Validate)]
pub struct CalculatePortfolioRequest {
    #[validate(length(min = 1), nested)]
    pub new_banks: Vec<PickerSymbol>,
}

#[derive(Default, Debug, Deserialize, Validate)]
pub struct PickerSymbol {
    #[validate(length(min = 1), custom(function = "validate_positive_decimal"))]
    pub period: String,
    #[validate(custom(function = "validate_period_unit"))]
    pub period_unit: String,
}


fn validate_period_unit(period_unit: &str) -> Result<(), ValidationError> {
    // DeltaPeriodUnit::from_str(period_unit).map_err(|_| {
    //     let mut error = ValidationError::new("invalid_period_unit");
    //     error.message = Some(
    //         format!(
    //             "Incorrect period_unit: {}. Invalid period unit. Must be Day, Week, Month, or Year.\n",
    //             period_unit
    //         )
    //         .into(),
    //     );
    //     error
    // })?;

    // uses FromStr implementation to parse the string into the enum variant: fn from_str(s: &str) -> Result<Self, Self::Err>;
    // converting from strum parse error to Validation error without thisError crate since no custom error defined here
    period_unit.parse::<DeltaPeriodUnit>().map_err(|_| {
        let mut error = ValidationError::new("invalid_period_unit");
        error.message = Some(
            format!(
                "Incorrect period_unit: {}. Must be Day, Week, Month, or Year.\n",
                period_unit
            )
            .into(),
        );
        error
    })?;
    Ok(())
}



// Response sections

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct CalculatePortfolioResponse {
    pub uuid: String,
    pub banks: Vec<Bank>,
    pub outcome: Option<Outcome>,
    pub created_at: String,
}


#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct ProcessingError {
    pub uuid: String,
    pub message: String,
}

use chrono::NaiveDate;
use chrono_tz::Tz;
use rust_decimal::Decimal;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ProcessingError {
    pub uuid: Uuid,
    pub message: String,
}

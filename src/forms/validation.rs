use chrono::{FixedOffset, TimeDelta};
use palette::Hsv;

use crate::models::{
    Bristol, ConsumableUnit, ConsumptionType, MaybeDateTime, MaybeF64, MaybeString,
};

use super::{FieldValue, errors::ValidationError};

pub fn validate_field_value<T: FieldValue>(str: &str) -> Result<T, ValidationError> {
    T::from_string(str).map_err(|_| ValidationError("Invalid value".to_string()))
}

pub fn validate_name(str: &str) -> Result<String, ValidationError> {
    validate_field_value(str)
}

pub fn validate_brand(str: &str) -> Result<MaybeString, ValidationError> {
    validate_field_value(str)
}

pub fn validate_barcode(str: &str) -> Result<MaybeString, ValidationError> {
    validate_field_value(str)
}

pub fn validate_username(str: &str) -> Result<String, ValidationError> {
    validate_field_value(str)
}

pub fn validate_full_name(str: &str) -> Result<String, ValidationError> {
    validate_field_value(str)
}

pub fn validate_email(str: &str) -> Result<String, ValidationError> {
    if !str.contains('@') {
        return Err(ValidationError("Email should contain @".to_string()));
    }
    validate_field_value(str)
}

pub fn validate_password(str: &str) -> Result<String, ValidationError> {
    validate_field_value(str)
}

pub fn validate_1st_password(str: &str) -> Result<String, ValidationError> {
    if str.is_empty() {
        return Err(ValidationError("Password cannot be empty".to_string()));
    }
    if str == "password" {
        return Err(ValidationError("Password cannot be 'password'".to_string()));
    }
    validate_field_value(str)
}

pub fn validate_2nd_password(str: &str, str2: &str) -> Result<String, ValidationError> {
    if str != str2 {
        return Err(ValidationError("Passwords do not match".to_string()));
    }
    validate_field_value(str)
}

pub fn validate_comments(str: &str) -> Result<MaybeString, ValidationError> {
    validate_field_value(str)
}

// pub fn validate_utc_date_time(str: &str) -> Result<chrono::DateTime<Utc>, ValidationError> {
//     validate_field_value(str)
// }

pub fn validate_fixed_offset_date_time(
    str: &str,
) -> Result<chrono::DateTime<FixedOffset>, ValidationError> {
    validate_field_value(str)
}

pub fn validate_maybe_date_time(str: &str) -> Result<MaybeDateTime, ValidationError> {
    validate_field_value(str)
}

pub fn validate_duration(str: &str) -> Result<TimeDelta, ValidationError> {
    validate_field_value(str)
}

pub fn validate_millilitres(str: &str) -> Result<i32, ValidationError> {
    validate_field_value(str)
}

pub fn validate_consumable_quantity(str: &str) -> Result<MaybeF64, ValidationError> {
    validate_field_value(str)
}

pub fn validate_consumable_millilitres(str: &str) -> Result<MaybeF64, ValidationError> {
    validate_field_value(str)
}

pub fn validate_consumable_unit(str: &str) -> Result<ConsumableUnit, ValidationError> {
    validate_field_value(str)
}

pub fn validate_consumption_type(str: &str) -> Result<ConsumptionType, ValidationError> {
    validate_field_value(str)
}

pub fn validate_bristol(str: &str) -> Result<Bristol, ValidationError> {
    validate_field_value(str)
}

pub fn validate_colour_hue(str: &str) -> Result<f32, ValidationError> {
    match str.parse() {
        Ok(hue) if (-180.0..=360.0).contains(&hue) => Ok(hue),
        Ok(_) => Err(ValidationError("Invalid hue".to_string())),
        Err(err) => Err(ValidationError(format!("Invalid hue: {err}"))),
    }
}

pub fn validate_colour_saturation(str: &str) -> Result<f32, ValidationError> {
    match str.parse() {
        Ok(saturation) if (0.0..=1.0).contains(&saturation) => Ok(saturation),
        Ok(_) => Err(ValidationError("Invalid saturation".to_string())),
        Err(err) => Err(ValidationError(format!("Invalid saturation: {err}"))),
    }
}

pub fn validate_colour_value(str: &str) -> Result<f32, ValidationError> {
    match str.parse() {
        Ok(value) if (0.0..=1.0).contains(&value) => Ok(value),
        Ok(_) => Err(ValidationError("Invalid value".to_string())),
        Err(err) => Err(ValidationError(format!("Invalid value: {err}"))),
    }
}

pub fn validate_colour(
    (hue, saturation, value): (String, String, String),
) -> Result<Hsv, ValidationError> {
    let hue = validate_colour_hue(str::trim(&hue))?;
    let saturation = validate_colour_saturation(str::trim(&saturation))?;
    let value = validate_colour_value(str::trim(&value))?;
    Ok(Hsv::new(hue, saturation, value))
}

pub fn validate_urgency(str: &str) -> Result<i32, ValidationError> {
    match str.parse() {
        Ok(urgency) if (0..=5).contains(&urgency) => Ok(urgency),
        Ok(_) => Err(ValidationError(
            "Urgency must be between 0 and 5".to_string(),
        )),
        Err(err) => Err(ValidationError(format!("Invalid integer: {err}"))),
    }
}

pub fn validate_poo_quantity(str: &str) -> Result<i32, ValidationError> {
    match str.parse() {
        Ok(quantity) if (0..=10).contains(&quantity) => Ok(quantity),
        Ok(_) => Err(ValidationError(
            "Quantity must be between 0 and 10".to_string(),
        )),
        Err(err) => Err(ValidationError(format!("Invalid integer: {err}"))),
    }
}

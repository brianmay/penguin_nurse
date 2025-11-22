use bigdecimal::BigDecimal;
use chrono::{DateTime, FixedOffset, TimeDelta, Utc};
use palette::Hsv;
use tap::Pipe;

use crate::models::{Bristol, ConsumableUnit, ConsumptionType, ExerciseRpe, ExerciseType, Urgency};

use super::{FieldValue, errors::ValidationError};

pub fn validate_field_value<T: FieldValue<RawValue = String, DerefValue = str>>(
    str: &str,
) -> Result<T, ValidationError> {
    T::from_raw(str).map_err(|err| ValidationError(err.to_string()))
}

pub fn validate_name(str: &str) -> Result<String, ValidationError> {
    validate_field_value(str)
}

pub fn validate_brand(str: &str) -> Result<Option<String>, ValidationError> {
    validate_field_value(str)
}

pub fn validate_barcode(str: &str) -> Result<Option<String>, ValidationError> {
    validate_field_value(str)
}

pub fn validate_username(str: &str) -> Result<String, ValidationError> {
    validate_field_value(str)
}

pub fn validate_full_name(str: &str) -> Result<String, ValidationError> {
    validate_field_value(str)
}

pub fn validate_email(str: &str) -> Result<String, ValidationError> {
    let str = validate_field_value::<String>(str)?;
    if !str.contains('@') {
        return Err(ValidationError("Email should contain @".to_string()));
    }
    Ok(str)
}

pub fn validate_password(str: &str) -> Result<String, ValidationError> {
    validate_field_value(str)
}

pub fn validate_1st_password(str: &str) -> Result<String, ValidationError> {
    let str = validate_field_value::<String>(str)?;

    if str.is_empty() {
        return Err(ValidationError("Password cannot be empty".to_string()));
    }
    if str == "password" {
        return Err(ValidationError("Password cannot be 'password'".to_string()));
    }
    Ok(str)
}

pub fn validate_2nd_password(
    password_1: &Result<String, ValidationError>,
    password_2: &str,
) -> Result<String, ValidationError> {
    let password_2 = validate_field_value::<String>(password_2)?;
    let password_1 = password_1
        .as_ref()
        .map_err(|_err| ValidationError("Passwords do not match".to_string()))?;
    if *password_1 != password_2 {
        return Err(ValidationError("Passwords do not match".to_string()));
    }
    Ok(password_2)
}

pub fn validate_comments(str: &str) -> Result<Option<String>, ValidationError> {
    validate_field_value(str)
}

pub fn validate_location(str: &str) -> Result<Option<String>, ValidationError> {
    validate_field_value(str)
}

pub fn validate_distance(str: &str) -> Result<Option<bigdecimal::BigDecimal>, ValidationError> {
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

pub fn validate_maybe_date_time(str: &str) -> Result<Option<DateTime<Utc>>, ValidationError> {
    validate_field_value(str)
}

pub fn validate_duration(str: &str) -> Result<TimeDelta, ValidationError> {
    validate_field_value(str)
}

pub fn validate_millilitres(str: &str) -> Result<i32, ValidationError> {
    validate_field_value(str)
}

pub fn validate_consumable_quantity(str: &str) -> Result<Option<f64>, ValidationError> {
    validate_field_value(str)
}

pub fn validate_consumable_millilitres(str: &str) -> Result<Option<f64>, ValidationError> {
    validate_field_value(str)
}

pub fn validate_consumable_unit(
    unit: Option<ConsumableUnit>,
) -> Result<ConsumableUnit, ValidationError> {
    unit.ok_or_else(|| ValidationError("Unit is required".to_string()))
}

pub fn validate_consumption_type(
    consumption_type: Option<ConsumptionType>,
) -> Result<ConsumptionType, ValidationError> {
    consumption_type.ok_or_else(|| ValidationError("Consumption type is required".to_string()))
}

pub fn validate_exercise_type(
    exercise_type: Option<ExerciseType>,
) -> Result<ExerciseType, ValidationError> {
    exercise_type.ok_or_else(|| ValidationError("Exercise type is required".to_string()))
}

pub fn validate_bristol(bristol_type: Option<Bristol>) -> Result<Bristol, ValidationError> {
    bristol_type.ok_or_else(|| ValidationError("Bristol type is required".to_string()))
}

pub fn validate_colour_hue(str: &str) -> Result<f32, ValidationError> {
    validate_in_range(str, -180.0, 360.0)
}

pub fn validate_colour_saturation(str: &str) -> Result<f32, ValidationError> {
    validate_in_range(str, 0.0, 1.0)
}

pub fn validate_colour_value(str: &str) -> Result<f32, ValidationError> {
    validate_in_range(str, 0.0, 1.0)
}

pub fn validate_colour(
    (hue, saturation, value): (String, String, String),
) -> Result<Hsv, ValidationError> {
    let hue = validate_colour_hue(str::trim(&hue));
    let saturation = validate_colour_saturation(str::trim(&saturation));
    let value = validate_colour_value(str::trim(&value));

    match (hue, saturation, value) {
        (Ok(hue), Ok(saturation), Ok(value)) => Ok(Hsv::new(hue, saturation, value)),
        (Err(_err), _, _) | (_, Err(_err), _) | (_, _, Err(_err)) => {
            Err(ValidationError("Invalid colour".to_string()))
        }
    }
}

pub fn validate_urgency(urgency: Option<Urgency>) -> Result<Urgency, ValidationError> {
    urgency.ok_or_else(|| ValidationError("Urgency is required".to_string()))
}

pub fn validate_poo_quantity(str: &str) -> Result<i32, ValidationError> {
    validate_in_range(str, 0, 10)
}

pub fn validate_in_range<T>(str: &str, min: T, max: T) -> Result<T, ValidationError>
where
    T: FieldValue<RawValue = String, DerefValue = str> + PartialOrd + std::fmt::Display,
{
    validate_field_value::<T>(str)?.pipe(|v: T| {
        if (min <= v) && (v <= max) {
            Ok(v)
        } else {
            Err(ValidationError(format!(
                "Value must be between {} and {}",
                min, max
            )))
        }
    })
}

pub fn validate_in_range_maybe<T>(str: &str, min: T, max: T) -> Result<Option<T>, ValidationError>
where
    T: FieldValue<RawValue = String, DerefValue = str> + PartialOrd + std::fmt::Display,
{
    validate_field_value::<Option<T>>(str)?
        .map(|v: T| {
            if (min <= v) && (v <= max) {
                Ok(v)
            } else {
                Err(ValidationError(format!(
                    "Value must be between {} and {}",
                    min, max
                )))
            }
        })
        .transpose()
}

pub fn validate_pulse(str: &str) -> Result<Option<i32>, ValidationError> {
    validate_in_range_maybe(str, 30, 220)
}

pub fn validate_blood_glucose(
    str: &str,
) -> Result<Option<bigdecimal::BigDecimal>, ValidationError> {
    validate_in_range_maybe(str, BigDecimal::from(0), BigDecimal::from(50))
}

pub fn validate_systolic_bp(str: &str) -> Result<Option<i32>, ValidationError> {
    validate_in_range_maybe(str, 50, 300)
}

pub fn validate_diastolic_bp(str: &str) -> Result<Option<i32>, ValidationError> {
    validate_in_range_maybe(str, 30, 200)
}

pub fn validate_weight(str: &str) -> Result<Option<bigdecimal::BigDecimal>, ValidationError> {
    validate_in_range_maybe(str, BigDecimal::from(0), BigDecimal::from(500))
}

pub fn validate_height(str: &str) -> Result<Option<i32>, ValidationError> {
    validate_in_range_maybe(str, 30, 300)
}

pub fn validate_waist_circumference(
    str: &str,
) -> Result<Option<bigdecimal::BigDecimal>, ValidationError> {
    validate_in_range_maybe(str, BigDecimal::from(30), BigDecimal::from(300))
}

pub fn validate_exercise_calories(str: &str) -> Result<Option<i32>, ValidationError> {
    validate_in_range_maybe(str, 0, 10_000)
}

pub fn validate_exercise_rpe(
    rpe: &Option<ExerciseRpe>,
) -> Result<Option<ExerciseRpe>, ValidationError> {
    Ok(*rpe)
}

pub fn validate_symptom_intensity(str: &str) -> Result<i32, ValidationError> {
    validate_in_range(str, 0, 10)
}

pub fn validate_symptom_extra_details(
    symptom_intensity: &Result<i32, ValidationError>,
    extra_details: &str,
) -> Result<Option<String>, ValidationError> {
    let extra_details = validate_field_value::<Option<String>>(extra_details)?;
    let symptom_intensity = *symptom_intensity
        .as_ref()
        .map_err(|_rr| ValidationError("Fix symptom intensity first".to_string()))?;

    match (symptom_intensity, &extra_details) {
        (0, Some(_)) => Err(ValidationError(
            "Extra details must be empty if symptom intensity is 0".to_string(),
        )),
        (x, None) if x > 0 => Err(ValidationError(
            "Extra details must be set if symptom intensity is greater than 0".to_string(),
        )),
        _ => Ok(extra_details),
    }
}

use chrono::{TimeDelta, Utc};
use palette::Hsv;

use super::errors::ValidationError;

// pub fn validate_order(str: &str) -> Result<i32, ValidationError> {
//     if str.is_empty() {
//         return Err(ValidationError("Order cannot be empty".to_string()));
//     }

//     str.parse()
//         .map_err(|err| ValidationError(format!("Invalid integer: {err}")))
// }

// pub fn validate_regexp(str: &str) -> Result<String, ValidationError> {
//     if str.is_empty() {
//         return Err(ValidationError("Regexp cannot be empty".to_string()));
//     }
//     regex::Regex::new(str).map_err(|err| ValidationError(format!("Invalid regexp: {err}")))?;
//     Ok(str.to_string())
// }

pub fn validate_username(str: &str) -> Result<String, ValidationError> {
    if str.is_empty() {
        return Err(ValidationError("Username cannot be empty".to_string()));
    }
    Ok(str.to_string())
}

pub fn validate_full_name(str: &str) -> Result<String, ValidationError> {
    if str.is_empty() {
        return Err(ValidationError("Name cannot be empty".to_string()));
    }
    Ok(str.to_string())
}

pub fn validate_email(str: &str) -> Result<String, ValidationError> {
    if str.is_empty() {
        return Err(ValidationError("Email cannot be empty".to_string()));
    }
    if !str.contains('@') {
        return Err(ValidationError("Email should contain @".to_string()));
    }
    Ok(str.to_string())
}

pub fn validate_password(str: &str) -> Result<String, ValidationError> {
    if str.is_empty() {
        return Err(ValidationError("Password cannot be empty".to_string()));
    }
    Ok(str.to_string())
}

pub fn validate_1st_password(str: &str) -> Result<String, ValidationError> {
    if str.is_empty() {
        return Err(ValidationError("Password cannot be empty".to_string()));
    }
    if str == "password" {
        return Err(ValidationError("Password cannot be 'password'".to_string()));
    }
    // if str.len() < 16 {
    //     return Err(ValidationError(
    //         "Password must be at least 16 characters".to_string(),
    //     ));
    // }
    Ok(str.to_string())
}

pub fn validate_2nd_password(str: &str, str2: &str) -> Result<String, ValidationError> {
    if str != str2 {
        return Err(ValidationError("Passwords do not match".to_string()));
    }
    Ok(str.to_string())
}

// pub fn validate_phone_number(str: &str) -> Result<String, ValidationError> {
//     if str.is_empty() {
//         return Err(ValidationError("Phone number cannot be empty".to_string()));
//     }
//     Ok(str.to_string())
// }

// pub fn validate_action(str: &str) -> Result<Action, ValidationError> {
//     Action::try_from(str).map_err(|err| ValidationError(format!("Invalid action: {err}")))
// }

// pub fn validate_comments(str: &str) -> Result<Option<String>, ValidationError> {
//     if str.is_empty() {
//         Ok(None)
//     } else {
//         Ok(Some(str.to_string()))
//     }
// }

pub fn validate_time(str: &str) -> Result<chrono::DateTime<Utc>, ValidationError> {
    match chrono::DateTime::parse_from_rfc3339(str) {
        Ok(time) => Ok(time.with_timezone(&Utc)),
        Err(err) => Err(ValidationError(format!("Invalid time: {err}"))),
    }
}

pub fn validate_duration(str: &str) -> Result<TimeDelta, ValidationError> {
    match str.parse() {
        Ok(duration) if duration >= 0 => Ok(TimeDelta::seconds(duration)),
        Ok(_) => Err(ValidationError(
            "Duration must be greater than or 0".to_string(),
        )),
        Err(err) => Err(ValidationError(format!("Invalid integer: {err}"))),
    }
}

pub fn validate_mls(str: &str) -> Result<i32, ValidationError> {
    match str.parse() {
        Ok(mls) => Ok(mls),
        Err(err) => Err(ValidationError(format!("Invalid integer: {err}"))),
    }
}

pub fn validate_bristol(str: &str) -> Result<i32, ValidationError> {
    match str.parse() {
        Ok(bristol) if (0..=7).contains(&bristol) => Ok(bristol),
        Ok(_) => Err(ValidationError(
            "Bristol must be between 0 and 7".to_string(),
        )),
        Err(err) => Err(ValidationError(format!("Invalid integer: {err}"))),
    }
}

pub fn validate_colour_hue(str: &str) -> Result<f32, ValidationError> {
    match str.parse() {
        Ok(hue) if (0.0..=360.0).contains(&hue) => Ok(hue),
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

pub fn validate_comments(str: &str) -> Result<Option<String>, ValidationError> {
    if str.is_empty() {
        Ok(None)
    } else {
        Ok(Some(str.to_string()))
    }
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
        Ok(quantity) if (0..=5).contains(&quantity) => Ok(quantity),
        Ok(_) => Err(ValidationError(
            "Quantity must be between 0 and 5".to_string(),
        )),
        Err(err) => Err(ValidationError(format!("Invalid integer: {err}"))),
    }
}

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
        return Err(ValidationError("Name cannot be empty".to_string()));
    }
    Ok(str.to_string())
}

pub fn validate_password(str: &str) -> Result<String, ValidationError> {
    if str.is_empty() {
        return Err(ValidationError("Password cannot be empty".to_string()));
    }
    Ok(str.to_string())
}

pub fn validate_new_password(str: &str) -> Result<String, ValidationError> {
    if str.is_empty() {
        return Err(ValidationError("Password cannot be empty".to_string()));
    }
    if str == "password" {
        return Err(ValidationError("Password cannot be 'password'".to_string()));
    }
    if str.len() < 16 {
        return Err(ValidationError(
            "Password must be at least 16 characters".to_string(),
        ));
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

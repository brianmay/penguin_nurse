mod buttons;
mod errors;
mod fields;
mod forms;
mod validation;

pub use buttons::SubmitButton;
pub use errors::{EditError, ValidationError};
pub use fields::{InputBoolean, InputPassword, InputString};
pub use forms::MyForm;
pub use forms::Saving;
pub use validation::{
    validate_1st_password, validate_2nd_password, validate_email, validate_full_name,
    validate_password, validate_username,
};

mod error;
mod traits;
pub mod validation;
pub use error::Error;
pub use traits::*;
pub use validation::{
    validate_array_length, validate_array_uniqueness, validate_generic_enumerated_values,
    validate_numeric_multiples, validate_numeric_range, validate_object_size,
    validate_string_length, validate_string_regular_expressions, FieldName, Limit,
};

pub fn from_value<T, V>(value: V) -> Result<T, self::Error<V::Error>>
where
    T: serde::de::DeserializeOwned,
    V: DeserializeWithValidation<T>,
    V::Error: std::fmt::Debug + std::fmt::Display + std::error::Error,
{
    value.deserialize_with_validation()
}

pub fn from_str<T, V>(str: &str) -> Result<T, self::Error<V::Error>>
where
    T: serde::de::DeserializeOwned,
    V: DeserializeWithValidationFromStr<T>,
    V::Error: std::fmt::Debug + std::fmt::Display + std::error::Error,
{
    V::deserialize_with_validation_from_str(str)
}

pub trait Validate {
    fn validate(&self) -> Result<(), self::validation::Errors>;
}

#[cfg(feature = "derive")]
pub use serde_valid_derive::Validate;

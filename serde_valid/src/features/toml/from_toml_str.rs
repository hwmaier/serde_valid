pub trait FromTomlStr<'de>
where
    Self: Sized,
{
    /// Convert from toml str.
    ///
    /// ```rust
    /// use serde::Deserialize;
    /// use serde_valid::Validate;
    /// use serde_valid::toml::FromTomlStr;
    ///
    /// #[derive(Debug, Validate, Deserialize)]
    /// struct TestStruct<'a> {
    ///     #[validate(min_length = 1)]
    ///     val: &'a str,
    /// }
    ///
    /// let s = TestStruct::from_toml_str(r#"val = "abcde""#);
    ///
    /// assert!(s.is_ok())
    /// ```
    fn from_toml_str(str: &'de str) -> Result<Self, crate::Error<serde_toml::de::Error>>;
}

impl<'de, T> FromTomlStr<'de> for T
where
    T: serde::de::Deserialize<'de> + crate::Validate,
{
    fn from_toml_str(str: &'de str) -> Result<Self, crate::Error<serde_toml::de::Error>> {
        let model: T = serde_toml::from_str(str)?;
        model.validate().map_err(crate::Error::ValidationError)?;
        Ok(model)
    }
}

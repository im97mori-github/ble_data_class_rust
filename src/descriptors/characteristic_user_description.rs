//! Characteristic User Description (Attribute Type: 0x2901) module.

use crate::Uuid16bit;

/// Characteristic User Description.
#[derive(Debug, PartialEq, Clone)]
pub struct CharacteristicUserDescription {
    /// Characteristic User Description
    pub description: String,
}

impl CharacteristicUserDescription {
    /// Create [`CharacteristicUserDescription`] from [`String`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::{
    ///     descriptors::characteristic_user_description::CharacteristicUserDescription, Uuid16bit,
    /// };
    ///
    /// let description = "description".to_string();
    /// let result = CharacteristicUserDescription::new(description.to_string());
    /// assert_eq!(description, result.description);
    /// ```
    pub fn new(description: String) -> Self {
        Self { description }
    }
}

impl TryFrom<&Vec<u8>> for CharacteristicUserDescription {
    type Error = String;
    /// Create [`CharacteristicUserDescription`] from [`Vec<u8>`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::{
    ///     descriptors::characteristic_user_description::CharacteristicUserDescription, Uuid16bit,
    /// };
    ///
    /// let description = "description".to_string();
    /// let result = CharacteristicUserDescription::try_from(description.to_string().into_bytes());
    /// assert!(result.is_ok());
    /// assert_eq!(description, result.unwrap().description);
    /// ```
    fn try_from(value: &Vec<u8>) -> Result<Self, String> {
        Ok(Self {
            description: String::from_utf8(value.to_vec()).unwrap(),
        })
    }
}

impl Into<Vec<u8>> for CharacteristicUserDescription {
    /// Create [`Vec<u8>`] from [`CharacteristicUserDescription`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::{
    ///     descriptors::characteristic_user_description::CharacteristicUserDescription, Uuid16bit,
    /// };
    ///
    /// let description = "description".to_string();
    /// let result = CharacteristicUserDescription::new(description.to_string());
    /// let into_data: Vec<u8> = result.into();
    /// assert_eq!(description.to_string().into_bytes(), into_data);
    /// ```
    fn into(self) -> Vec<u8> {
        self.description.clone().into_bytes()
    }
}

impl Uuid16bit for CharacteristicUserDescription {
    /// return `0x2901`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::{
    ///     descriptors::characteristic_user_description::CharacteristicUserDescription, Uuid16bit,
    /// };
    ///
    /// assert_eq!(0x2901, CharacteristicUserDescription::uuid_16bit());
    /// ```
    fn uuid_16bit() -> u16 {
        0x2901
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        descriptors::characteristic_user_description::CharacteristicUserDescription, Uuid16bit,
    };

    #[test]
    fn test_new() {
        let description = "description".to_string();
        let result = CharacteristicUserDescription::new(description.to_string());
        assert_eq!(description, result.description);
    }

    #[test]
    fn test_try_from() {
        let description = "description".to_string();
        let result = CharacteristicUserDescription::try_from(&description.to_string().into_bytes());
        assert!(result.is_ok());
        assert_eq!(description, result.unwrap().description);
    }

    #[test]
    fn test_into() {
        let description = "description".to_string();
        let result = CharacteristicUserDescription::new(description.to_string());
        let into_data: Vec<u8> = result.into();
        assert_eq!(description.to_string().into_bytes(), into_data);
    }

    #[test]
    fn test_uuid_16bit() {
        assert_eq!(0x2901, CharacteristicUserDescription::uuid_16bit());
    }
}

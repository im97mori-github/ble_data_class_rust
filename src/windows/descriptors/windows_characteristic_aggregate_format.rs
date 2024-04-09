//! Characteristic Aggregate Format (Attribute Type: 0x2905) module for windows.

use crate::Uuid16bit;

/// Characteristic Aggregate Format.
#[derive(Debug, PartialEq, Clone)]
pub struct CharacteristicAggregateFormat {
    /// List of Attribute Handles
    pub list_of_attribute_handles: Vec<u16>,
}

impl CharacteristicAggregateFormat {
    /// Create [`CharacteristicAggregateFormat`] from [`String`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::{
    ///     descriptors::characteristic_aggregate_format::CharacteristicAggregateFormat, Uuid16bit,
    /// };
    ///
    /// let list_of_attribute_handles: Vec<u16> = [0x0201, 0x0403].to_vec();
    /// let result = CharacteristicAggregateFormat::new(&list_of_attribute_handles.clone());
    /// assert_eq!(list_of_attribute_handles, result.list_of_attribute_handles);
    /// ```
    pub fn new(list_of_attribute_handles: &Vec<u16>) -> Self {
        Self {
            list_of_attribute_handles: list_of_attribute_handles.clone(),
        }
    }
}

impl TryFrom<&Vec<u8>> for CharacteristicAggregateFormat {
    type Error = String;
    /// Create [`CharacteristicAggregateFormat`] from [`Vec<u8>`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::{
    ///     descriptors::characteristic_aggregate_format::CharacteristicAggregateFormat, Uuid16bit,
    /// };
    ///
    /// let list_of_attribute_handles: Vec<u16> = [0x0201, 0x0403].to_vec();
    /// let data: Vec<u8> = list_of_attribute_handles
    ///     .clone()
    ///     .iter()
    ///     .flat_map(|f| f.to_le_bytes())
    ///     .collect();
    /// 
    /// let result = CharacteristicAggregateFormat::try_from(&data);
    /// assert!(result.is_ok());
    /// let descriptor = result.unwrap();
    /// assert_eq!(
    ///     list_of_attribute_handles,
    ///     descriptor.list_of_attribute_handles
    /// );
    /// 
    /// let result = CharacteristicAggregateFormat::try_from(&Vec::new());
    /// assert!(!result.is_ok());
    /// 
    /// let result = CharacteristicAggregateFormat::try_from(&vec![0, 1, 2]);
    /// assert!(!result.is_ok());
    /// ```
    fn try_from(value: &Vec<u8>) -> Result<Self, String> {
        let len = value.len();
        if len < 2 {
            return Err(format!("Invalid data size :{}", len).to_string());
        }
        if len % 2 == 1 {
            return Err(format!("Invalid data size :{}", len).to_string());
        }
        Ok(Self {
            list_of_attribute_handles: value
                .windows(2)
                .step_by(2)
                .map(|w| u16::from_le_bytes(w[0..2].try_into().unwrap()))
                .collect(),
        })
    }
}

impl Into<Vec<u8>> for CharacteristicAggregateFormat {
    /// Create [`Vec<u8>`] from [`CharacteristicAggregateFormat`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::{
    ///     descriptors::characteristic_aggregate_format::CharacteristicAggregateFormat, Uuid16bit,
    /// };
    ///
    /// let list_of_attribute_handles: Vec<u16> = [0x0201, 0x0403].to_vec();
    /// let result = CharacteristicAggregateFormat::new(&list_of_attribute_handles.clone());
    /// 
    /// let data: Vec<u8> = list_of_attribute_handles
    ///     .clone()
    ///     .iter()
    ///     .flat_map(|f| f.to_le_bytes())
    ///     .collect();
    /// let into_data: Vec<u8> = result.into();
    /// assert_eq!(data, into_data);
    /// ```
    fn into(self) -> Vec<u8> {
        return self
            .list_of_attribute_handles
            .clone()
            .iter()
            .flat_map(|f| f.to_le_bytes())
            .collect();
    }
}

impl Uuid16bit for CharacteristicAggregateFormat {
    /// return `0x2905`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::{
    ///     descriptors::characteristic_aggregate_format::CharacteristicAggregateFormat, Uuid16bit,
    /// };
    ///
    /// assert_eq!(0x2905, CharacteristicAggregateFormat::uuid_16bit());
    /// ```
    fn uuid_16bit() -> u16 {
        0x2905
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        descriptors::characteristic_aggregate_format::CharacteristicAggregateFormat, Uuid16bit,
    };

    #[test]
    fn test_new() {
        let list_of_attribute_handles: Vec<u16> = [0x0201, 0x0403].to_vec();
        let result = CharacteristicAggregateFormat::new(&list_of_attribute_handles.clone());
        assert_eq!(list_of_attribute_handles, result.list_of_attribute_handles);
    }

    #[test]
    fn test_try_from() {
        let list_of_attribute_handles: Vec<u16> = [0x0201, 0x0403].to_vec();
        let data: Vec<u8> = list_of_attribute_handles
            .clone()
            .iter()
            .flat_map(|f| f.to_le_bytes())
            .collect();

        let result = CharacteristicAggregateFormat::try_from(&data);
        assert!(result.is_ok());
        let descriptor = result.unwrap();
        assert_eq!(
            list_of_attribute_handles,
            descriptor.list_of_attribute_handles
        );

        let result = CharacteristicAggregateFormat::try_from(&Vec::new());
        assert!(!result.is_ok());

        let result = CharacteristicAggregateFormat::try_from(&vec![0, 1, 2]);
        assert!(!result.is_ok());
    }

    #[test]
    fn test_into() {
        let list_of_attribute_handles: Vec<u16> = [0x0201, 0x0403].to_vec();
        let result = CharacteristicAggregateFormat::new(&list_of_attribute_handles.clone());

        let data: Vec<u8> = list_of_attribute_handles
            .clone()
            .iter()
            .flat_map(|f| f.to_le_bytes())
            .collect();
        let into_data: Vec<u8> = result.into();
        assert_eq!(data, into_data);
    }

    #[test]
    fn test_uuid_16bit() {
        assert_eq!(0x2905, CharacteristicAggregateFormat::uuid_16bit());
    }
}

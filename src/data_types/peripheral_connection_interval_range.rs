//! Peripheral Connection Interval Range (Data Type Value: 0x1a) module.

use crate::data_types::data_type::DataType;

/// Peripheral Connection Interval Range.

#[derive(Debug)]
pub struct PeripheralConnectionIntervalRange {
    /// data length
    pub length: u8,

    /// Minimum connection interval
    pub minimum_value: u16,

    /// Maximum connection interval
    pub maximum_value: u16,
}

impl PeripheralConnectionIntervalRange {
    /// Create [`PeripheralConnectionIntervalRange`] from `Peripheral Connection Interval Range`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::peripheral_connection_interval_range::PeripheralConnectionIntervalRange;
    ///
    /// let minimum_value = 0x0006u16;
    /// let maximum_value = 0x0C80u16;
    /// let result = PeripheralConnectionIntervalRange::new(minimum_value, maximum_value);
    /// assert_eq!(5, result.length);
    /// assert_eq!(minimum_value, result.minimum_value);
    /// assert_eq!(maximum_value, result.maximum_value);
    /// ```
    pub fn new(minimum_value: u16, maximum_value: u16) -> Self {
        Self {
            length: 5,
            minimum_value,
            maximum_value,
        }
    }

    /// Get Minimum connection interval(millis).
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{
    ///    peripheral_connection_interval_range::{PeripheralConnectionIntervalRange, CONNECTION_INTERVAL_RANGE},
    ///    data_type::DataType,
    /// };
    ///
    /// let minimum_value = 0x0006u16;
    /// let maximum_value = 0x0C80u16;
    /// let result = PeripheralConnectionIntervalRange::new(minimum_value, maximum_value);
    /// assert_eq!(
    ///     minimum_value as f32 * CONNECTION_INTERVAL_RANGE,
    ///     result.minimum_value_millis()
    /// )
    /// ```
    pub fn minimum_value_millis(&self) -> f32 {
        self.minimum_value as f32 * CONNECTION_INTERVAL_RANGE
    }

    /// Get Maximum connection interval(millis).
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{
    ///    peripheral_connection_interval_range::{PeripheralConnectionIntervalRange, CONNECTION_INTERVAL_RANGE},
    ///    data_type::DataType,
    /// };
    ///
    /// let minimum_value = 0x0006u16;
    /// let maximum_value = 0x0C80u16;
    /// let result = PeripheralConnectionIntervalRange::new(minimum_value, maximum_value);
    /// assert_eq!(
    ///     minimum_value as f32 * CONNECTION_INTERVAL_RANGE,
    ///     result.minimum_value_millis()
    /// )
    /// ```
    pub fn maximum_value_millis(&self) -> f32 {
        self.maximum_value as f32 * CONNECTION_INTERVAL_RANGE
    }

    /// check no specific minimum values
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{
    ///    peripheral_connection_interval_range::{PeripheralConnectionIntervalRange, CONNECTION_INTERVAL_NO_SPECIFIC_VALUE},
    ///    data_type::DataType,
    /// };
    ///
    /// let minimum_value = 0x0006u16;
    /// let maximum_value = 0x0C80u16;
    /// let result = PeripheralConnectionIntervalRange::new(minimum_value, maximum_value);
    /// assert!(!result.is_no_specific_minimum_value());
    ///
    /// let minimum_value = CONNECTION_INTERVAL_NO_SPECIFIC_VALUE;
    /// let maximum_value = 0x0C80u16;
    /// let result = PeripheralConnectionIntervalRange::new(minimum_value, maximum_value);
    /// assert!(result.is_no_specific_minimum_value());
    /// ```
    pub fn is_no_specific_minimum_value(&self) -> bool {
        self.minimum_value == CONNECTION_INTERVAL_NO_SPECIFIC_VALUE
    }

    /// check no specific minimum values
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{
    ///    peripheral_connection_interval_range::{PeripheralConnectionIntervalRange, CONNECTION_INTERVAL_NO_SPECIFIC_VALUE},
    ///    data_type::DataType,
    /// };
    ///
    /// let minimum_value = 0x0006u16;
    /// let maximum_value = 0x0C80u16;
    /// let result = PeripheralConnectionIntervalRange::new(minimum_value, maximum_value);
    /// assert!(!result.is_no_specific_maximum_value());
    ///
    /// let minimum_value = 0x0006u16;
    /// let maximum_value = CONNECTION_INTERVAL_NO_SPECIFIC_VALUE;
    /// let result = PeripheralConnectionIntervalRange::new(minimum_value, maximum_value);
    /// assert!(result.is_no_specific_maximum_value());
    /// ```
    pub fn is_no_specific_maximum_value(&self) -> bool {
        self.maximum_value == CONNECTION_INTERVAL_NO_SPECIFIC_VALUE
    }
}

/// Units: 1.25 ms
///
/// Connection Interval Range
pub const CONNECTION_INTERVAL_RANGE: f32 = 1.25;

/// no specific minimum / maximum values
pub const CONNECTION_INTERVAL_NO_SPECIFIC_VALUE: u16 = 0xffff;

impl TryFrom<&Vec<u8>> for PeripheralConnectionIntervalRange {
    type Error = String;
    /// Create [`PeripheralConnectionIntervalRange`] from `Vec<u8>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{peripheral_connection_interval_range::PeripheralConnectionIntervalRange, data_type::DataType};
    ///
    /// let minimum_value = 0x0006u16;
    /// let maximum_value = 0x0C80u16;
    /// let length = 5;
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(PeripheralConnectionIntervalRange::data_type());
    /// data.append(&mut minimum_value.to_le_bytes().to_vec());
    /// data.append(&mut maximum_value.to_le_bytes().to_vec());
    ///
    /// let result = PeripheralConnectionIntervalRange::try_from(&data);
    /// assert!(result.is_ok());
    /// let data_type = result.unwrap();
    /// assert_eq!(length, data_type.length);
    /// assert_eq!(minimum_value, data_type.minimum_value);
    /// assert_eq!(maximum_value, data_type.maximum_value);
    ///
    /// let data: Vec<u8> = Vec::new();
    /// let result = PeripheralConnectionIntervalRange::try_from(&data);
    /// assert!(result.is_err());
    /// assert_eq!(
    ///     format!("Invalid data size :{}", data.len()),
    ///     result.unwrap_err()
    /// );
    /// ```
    fn try_from(value: &Vec<u8>) -> Result<Self, String> {
        let len = value.len();
        if len < 6 {
            return Err(format!("Invalid data size :{}", len).to_string());
        }
        let length = value[0];
        Ok(Self {
            length,
            minimum_value: u16::from_le_bytes(value[2..4].try_into().unwrap()),
            maximum_value: u16::from_le_bytes(value[4..6].try_into().unwrap()),
        })
    }
}

impl Into<Vec<u8>> for PeripheralConnectionIntervalRange {
    /// Create `Vec<u8>` from [`PeripheralConnectionIntervalRange`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{peripheral_connection_interval_range::PeripheralConnectionIntervalRange, data_type::DataType};
    ///
    /// let minimum_value = 0x0006u16;
    /// let maximum_value = 0x0C80u16;
    /// let result1 = PeripheralConnectionIntervalRange::new(minimum_value, maximum_value);
    ///
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(5);
    /// data.push(PeripheralConnectionIntervalRange::data_type());
    /// data.append(&mut minimum_value.to_le_bytes().to_vec());
    /// data.append(&mut maximum_value.to_le_bytes().to_vec());
    ///
    /// let into_data: Vec<u8> = result1.into();
    /// assert_eq!(data, into_data);
    ///
    /// let result2 = PeripheralConnectionIntervalRange::try_from(&data);
    /// assert!(result2.is_ok());
    /// let data_type = result2.unwrap();
    /// let into_data: Vec<u8> = data_type.into();
    /// assert_eq!(data, into_data);
    /// ```
    fn into(self) -> Vec<u8> {
        let mut data: Vec<u8> = Vec::new();
        data.push(self.length);
        data.push(Self::data_type());
        data.append(&mut self.minimum_value.to_le_bytes().to_vec());
        data.append(&mut self.maximum_value.to_le_bytes().to_vec());
        return data;
    }
}

impl DataType for PeripheralConnectionIntervalRange {
    /// return `0x12`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{peripheral_connection_interval_range::PeripheralConnectionIntervalRange, data_type::DataType};
    ///
    /// assert_eq!(0x12, PeripheralConnectionIntervalRange::data_type());
    /// ```
    fn data_type() -> u8 {
        0x12
    }
}

/// check `Peripheral Connection Interval Range` data type.
///
/// # Examples
///
/// ```
/// use ble_data_struct::data_types::peripheral_connection_interval_range::*;
/// use ble_data_struct::data_types::data_type::DataType;
///
/// assert!(is_peripheral_connection_interval_range(0x12));
/// assert!(!is_peripheral_connection_interval_range(0x00));
/// ```
pub fn is_peripheral_connection_interval_range(data_type: u8) -> bool {
    PeripheralConnectionIntervalRange::data_type() == data_type
}

#[cfg(test)]
mod tests {
    use crate::data_types::{data_type::DataType, peripheral_connection_interval_range::*};

    #[test]
    fn test_new() {
        let minimum_value = 0x0006u16;
        let maximum_value = 0x0C80u16;
        let result = PeripheralConnectionIntervalRange::new(minimum_value, maximum_value);
        assert_eq!(5, result.length);
        assert_eq!(minimum_value, result.minimum_value);
        assert_eq!(maximum_value, result.maximum_value);
    }

    #[test]
    fn test_minimum_value_millis() {
        let minimum_value = 0x0006u16;
        let maximum_value = 0x0C80u16;
        let result = PeripheralConnectionIntervalRange::new(minimum_value, maximum_value);
        assert_eq!(
            minimum_value as f32 * CONNECTION_INTERVAL_RANGE,
            result.minimum_value_millis()
        )
    }

    #[test]
    fn test_maximum_value_millis() {
        let minimum_value = 0x0006u16;
        let maximum_value = 0x0C80u16;
        let result = PeripheralConnectionIntervalRange::new(minimum_value, maximum_value);
        assert_eq!(
            minimum_value as f32 * CONNECTION_INTERVAL_RANGE,
            result.minimum_value_millis()
        )
    }

    #[test]
    fn test_is_no_specific_minimum_value() {
        let minimum_value = 0x0006u16;
        let maximum_value = 0x0C80u16;
        let result = PeripheralConnectionIntervalRange::new(minimum_value, maximum_value);
        assert!(!result.is_no_specific_minimum_value());

        let minimum_value = CONNECTION_INTERVAL_NO_SPECIFIC_VALUE;
        let maximum_value = 0x0C80u16;
        let result = PeripheralConnectionIntervalRange::new(minimum_value, maximum_value);
        assert!(result.is_no_specific_minimum_value());
    }

    #[test]
    fn test_is_no_specific_maximum_value() {
        let minimum_value = 0x0006u16;
        let maximum_value = 0x0C80u16;
        let result = PeripheralConnectionIntervalRange::new(minimum_value, maximum_value);
        assert!(!result.is_no_specific_maximum_value());

        let minimum_value = 0x0006u16;
        let maximum_value = CONNECTION_INTERVAL_NO_SPECIFIC_VALUE;
        let result = PeripheralConnectionIntervalRange::new(minimum_value, maximum_value);
        assert!(result.is_no_specific_maximum_value());
    }

    #[test]
    fn test_try_from() {
        let minimum_value = 0x0006u16;
        let maximum_value = 0x0C80u16;
        let length = 5;
        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(PeripheralConnectionIntervalRange::data_type());
        data.append(&mut minimum_value.to_le_bytes().to_vec());
        data.append(&mut maximum_value.to_le_bytes().to_vec());

        let result = PeripheralConnectionIntervalRange::try_from(&data);
        assert!(result.is_ok());
        let data_type = result.unwrap();
        assert_eq!(length, data_type.length);
        assert_eq!(minimum_value, data_type.minimum_value);
        assert_eq!(maximum_value, data_type.maximum_value);

        let data: Vec<u8> = Vec::new();
        let result = PeripheralConnectionIntervalRange::try_from(&data);
        assert!(result.is_err());
        assert_eq!(
            format!("Invalid data size :{}", data.len()),
            result.unwrap_err()
        );
    }

    #[test]
    fn test_into() {
        let minimum_value = 0x0006u16;
        let maximum_value = 0x0C80u16;
        let result1 = PeripheralConnectionIntervalRange::new(minimum_value, maximum_value);

        let mut data: Vec<u8> = Vec::new();
        data.push(5);
        data.push(PeripheralConnectionIntervalRange::data_type());
        data.append(&mut minimum_value.to_le_bytes().to_vec());
        data.append(&mut maximum_value.to_le_bytes().to_vec());

        let into_data: Vec<u8> = result1.into();
        assert_eq!(data, into_data);

        let result2 = PeripheralConnectionIntervalRange::try_from(&data);
        assert!(result2.is_ok());
        let data_type = result2.unwrap();
        let into_data: Vec<u8> = data_type.into();
        assert_eq!(data, into_data);
    }

    #[test]
    fn test_data_type() {
        assert_eq!(0x12, PeripheralConnectionIntervalRange::data_type());
    }

    #[test]
    fn test_is_peripheral_connection_interval_range() {
        assert!(is_peripheral_connection_interval_range(0x12));
        assert!(!is_peripheral_connection_interval_range(0x00));
    }
}

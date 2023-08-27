//! Advertising Interval (Data Type Value: 0x1a) module.

use crate::data_types::data_type::DataType;

/// Advertising Interval.
pub struct AdvertisingInterval {
    /// data length
    pub length: u8,

    /// Advertising Interval
    pub advertising_interval: u16,
}

impl AdvertisingInterval {
    /// Create [AdvertisingInterval] from `Advertising Interval`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_class::data_types::advertising_interval::AdvertisingInterval;
    ///
    /// let advertising_interval = 0x01;
    /// let result = AdvertisingInterval::new(advertising_interval);
    /// assert_eq!(3, result.length);
    /// assert_eq!(advertising_interval, result.advertising_interval);
    /// ```
    pub fn new(advertising_interval: u16) -> Self {
        Self {
            length: 3,
            advertising_interval,
        }
    }

    /// Create [AdvertisingInterval] from `Vec<u8>` with offset.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_class::data_types::{advertising_interval::AdvertisingInterval, data_type::DataType};
    ///
    /// let advertising_interval: u16 = 0x01;
    /// let length = 3;
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(AdvertisingInterval::data_type());
    /// data.append(&mut advertising_interval.to_le_bytes().to_vec());
    ///
    /// let result = AdvertisingInterval::from_with_offset(&data, 0);
    /// assert_eq!(length, result.length);
    /// assert_eq!(advertising_interval, result.advertising_interval);
    ///
    /// data = Vec::new();
    /// data.push(0);
    /// data.push(length);
    /// data.push(AdvertisingInterval::data_type());
    /// data.append(&mut advertising_interval.to_le_bytes().to_vec());
    /// let result = AdvertisingInterval::from_with_offset(&data, 1);
    /// assert_eq!(length, result.length);
    /// assert_eq!(advertising_interval, result.advertising_interval);
    /// ```
    pub fn from_with_offset(data: &Vec<u8>, offset: usize) -> Self {
        let data = data[offset..].to_vec();
        let length = data[0];
        Self {
            length,
            advertising_interval: u16::from_le_bytes(data[2..4].try_into().unwrap()),
        }
    }

    /// Get Advertising Interval(millis).
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_class::data_types::{
    ///    advertising_interval::{AdvertisingInterval, ADVINTERVAL_VALUE},
    ///    data_type::DataType,
    /// };
    ///
    /// let advertising_interval: u16 = 0x01;
    /// let result = AdvertisingInterval::new(advertising_interval);
    /// assert_eq!(
    /// advertising_interval as f32 * ADVINTERVAL_VALUE,
    /// result.advertising_interval_millis()
    /// )
    /// ```
    pub fn advertising_interval_millis(self) -> f32 {
        self.advertising_interval as f32 * ADVINTERVAL_VALUE
    }
}

impl From<&Vec<u8>> for AdvertisingInterval {
    /// Create [AdvertisingInterval] from `Vec<u8>`.
    ///
    /// [`AdvertisingInterval::from_with_offset`]
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_class::data_types::{advertising_interval::AdvertisingInterval, data_type::DataType};
    ///
    /// let advertising_interval: u16 = 0x01;
    /// let length = 3;
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(AdvertisingInterval::data_type());
    /// data.append(&mut advertising_interval.to_le_bytes().to_vec());
    /// let result = AdvertisingInterval::from(&data);
    /// assert_eq!(length, result.length);
    /// assert_eq!(advertising_interval, result.advertising_interval);
    /// ```
    fn from(data: &Vec<u8>) -> Self {
        Self::from_with_offset(data, 0)
    }
}

/// Units: 0.625 ms
///
/// advInterval value
pub const ADVINTERVAL_VALUE: f32 = 0.625;

impl Into<Vec<u8>> for AdvertisingInterval {
    /// Create `Vec<u8>` from [AdvertisingInterval].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_class::data_types::{advertising_interval::AdvertisingInterval, data_type::DataType};
    ///
    /// let advertising_interval: u16 = 0x01;
    /// let result1 = AdvertisingInterval::new(advertising_interval);
    ///
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(3);
    /// data.push(AdvertisingInterval::data_type());
    /// data.append(&mut advertising_interval.to_le_bytes().to_vec());
    ///
    /// let into_data: Vec<u8> = result1.into();
    /// assert_eq!(data, into_data);
    ///
    /// let result2 = AdvertisingInterval::from(&data);
    /// let into_data: Vec<u8> = result2.into();
    /// assert_eq!(data, into_data);
    /// ```
    fn into(self) -> Vec<u8> {
        let mut data: Vec<u8> = Vec::new();
        data.push(self.length);
        data.push(Self::data_type());
        data.append(&mut self.advertising_interval.to_le_bytes().to_vec());
        return data;
    }
}

impl DataType for AdvertisingInterval {
    /// return `0x1a`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_class::data_types::{advertising_interval::AdvertisingInterval, data_type::DataType};
    ///
    /// assert_eq!(0x1a, AdvertisingInterval::data_type());
    /// ```
    fn data_type() -> u8 {
        0x1a
    }
}

/// check `Advertising Interval` data type.
///
/// # Examples
///
/// ```
/// use ble_data_class::data_types::advertising_interval::*;
/// use ble_data_class::data_types::data_type::DataType;
///
/// assert!(is_advertising_interval(0x1a));
/// assert!(!is_advertising_interval(0x00));
/// ```
pub fn is_advertising_interval(data_type: u8) -> bool {
    AdvertisingInterval::data_type() == data_type
}

#[cfg(test)]
mod tests {
    use crate::data_types::{advertising_interval::*, data_type::DataType};

    #[test]
    fn test_new() {
        let advertising_interval = 0x01;
        let result = AdvertisingInterval::new(advertising_interval);
        assert_eq!(3, result.length);
        assert_eq!(advertising_interval, result.advertising_interval);
    }

    #[test]
    fn test_from_with_offset() {
        let advertising_interval: u16 = 0x01;
        let length = 3;
        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(AdvertisingInterval::data_type());
        data.append(&mut advertising_interval.to_le_bytes().to_vec());

        let result = AdvertisingInterval::from_with_offset(&data, 0);
        assert_eq!(length, result.length);
        assert_eq!(advertising_interval, result.advertising_interval);

        data = Vec::new();
        data.push(0);
        data.push(length);
        data.push(AdvertisingInterval::data_type());
        data.append(&mut advertising_interval.to_le_bytes().to_vec());
        let result = AdvertisingInterval::from_with_offset(&data, 1);
        assert_eq!(length, result.length);
        assert_eq!(advertising_interval, result.advertising_interval);
    }

    #[test]
    fn test_advertising_interval_millis() {
        let advertising_interval: u16 = 0x01;
        let result = AdvertisingInterval::new(advertising_interval);
        assert_eq!(
            advertising_interval as f32 * ADVINTERVAL_VALUE,
            result.advertising_interval_millis()
        )
    }

    #[test]
    fn test_from() {
        let advertising_interval: u16 = 0x01;
        let length = 3;
        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(AdvertisingInterval::data_type());
        data.append(&mut advertising_interval.to_le_bytes().to_vec());
        let result = AdvertisingInterval::from(&data);
        assert_eq!(length, result.length);
        assert_eq!(advertising_interval, result.advertising_interval);
    }

    #[test]
    fn test_into() {
        let advertising_interval: u16 = 0x01;
        let result1 = AdvertisingInterval::new(advertising_interval);

        let mut data: Vec<u8> = Vec::new();
        data.push(3);
        data.push(AdvertisingInterval::data_type());
        data.append(&mut advertising_interval.to_le_bytes().to_vec());

        let into_data: Vec<u8> = result1.into();
        assert_eq!(data, into_data);

        let result2 = AdvertisingInterval::from(&data);
        let into_data: Vec<u8> = result2.into();
        assert_eq!(data, into_data);
    }

    #[test]
    fn test_data_type() {
        assert_eq!(0x1a, AdvertisingInterval::data_type());
    }

    #[test]
    fn test_is_advertising_interval() {
        assert!(is_advertising_interval(0x1a));
        assert!(!is_advertising_interval(0x00));
    }
}

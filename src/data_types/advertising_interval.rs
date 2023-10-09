//! Advertising Interval (Data Type Value: 0x1a) module.

use crate::data_types::data_type::DataType;

/// Advertising Interval.
#[derive(Debug, PartialEq, Clone)]
pub struct AdvertisingInterval {
    /// data length
    pub length: u8,

    /// Advertising Interval
    pub advertising_interval: u16,
}

impl AdvertisingInterval {
    /// Create [`AdvertisingInterval`] from `Advertising Interval`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::advertising_interval::AdvertisingInterval;
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

    /// Get Advertising Interval(millis).
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{
    ///    advertising_interval::{AdvertisingInterval, ADVINTERVAL_VALUE},
    ///    data_type::DataType,
    /// };
    ///
    /// let advertising_interval: u16 = 0x01;
    /// let result = AdvertisingInterval::new(advertising_interval);
    /// assert_eq!(
    ///     advertising_interval as f32 * ADVINTERVAL_VALUE,
    ///     result.advertising_interval_millis()
    /// )
    /// ```
    pub fn advertising_interval_millis(&self) -> f32 {
        self.advertising_interval as f32 * ADVINTERVAL_VALUE
    }
}

/// Units: 0.625 ms
///
/// advInterval value
pub const ADVINTERVAL_VALUE: f32 = 0.625;

impl TryFrom<&Vec<u8>> for AdvertisingInterval {
    type Error = String;
    /// Create [`AdvertisingInterval`] from [`Vec<u8>`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{advertising_interval::AdvertisingInterval, data_type::DataType};
    ///
    /// let advertising_interval: u16 = 0x01;
    /// let length = 3;
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(AdvertisingInterval::data_type());
    /// data.append(&mut advertising_interval.to_le_bytes().to_vec());
    ///
    /// let result = AdvertisingInterval::try_from(&data);
    /// assert!(result.is_ok());
    /// let data_type = result.unwrap();
    /// assert_eq!(length, data_type.length);
    /// assert_eq!(advertising_interval, data_type.advertising_interval);
    ///
    /// let data: Vec<u8> = Vec::new();
    /// let result = AdvertisingInterval::try_from(&data);
    /// assert!(result.is_err());
    /// assert_eq!(
    ///     format!("Invalid data size :{}", data.len()),
    ///     result.unwrap_err()
    /// );
    /// ```
    fn try_from(value: &Vec<u8>) -> Result<Self, String> {
        let len = value.len();
        if len < 4 {
            return Err(format!("Invalid data size :{}", len).to_string());
        }
        let length = value[0];
        Ok(Self {
            length,
            advertising_interval: u16::from_le_bytes(value[2..4].try_into().unwrap()),
        })
    }
}

impl Into<Vec<u8>> for AdvertisingInterval {
    /// Create [`Vec<u8>`] from [`AdvertisingInterval`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{advertising_interval::AdvertisingInterval, data_type::DataType};
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
    /// let result2 = AdvertisingInterval::try_from(&data);
    /// assert!(result2.is_ok());
    /// let data_type = result2.unwrap();
    /// let into_data: Vec<u8> = data_type.into();
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
    /// use ble_data_struct::data_types::{advertising_interval::AdvertisingInterval, data_type::DataType};
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
/// use ble_data_struct::data_types::advertising_interval::*;
/// use ble_data_struct::data_types::data_type::DataType;
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
    fn test_advertising_interval_millis() {
        let advertising_interval: u16 = 0x01;
        let result = AdvertisingInterval::new(advertising_interval);
        assert_eq!(
            advertising_interval as f32 * ADVINTERVAL_VALUE,
            result.advertising_interval_millis()
        )
    }

    #[test]
    fn test_try_from() {
        let advertising_interval: u16 = 0x01;
        let length = 3;
        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(AdvertisingInterval::data_type());
        data.append(&mut advertising_interval.to_le_bytes().to_vec());

        let result = AdvertisingInterval::try_from(&data);
        assert!(result.is_ok());
        let data_type = result.unwrap();
        assert_eq!(length, data_type.length);
        assert_eq!(advertising_interval, data_type.advertising_interval);

        let mut data: Vec<u8> = vec![0u8; 3];
        data[0] = data.len() as u8 - 1;
        let result = AdvertisingInterval::try_from(&data);
        assert!(result.is_err());
        assert_eq!(
            format!("Invalid data size :{}", data.len()),
            result.unwrap_err()
        );
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

        let result2 = AdvertisingInterval::try_from(&data);
        assert!(result2.is_ok());
        let data_type = result2.unwrap();
        let into_data: Vec<u8> = data_type.into();
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

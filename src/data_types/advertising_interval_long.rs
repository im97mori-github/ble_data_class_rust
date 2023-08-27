//! Advertising Interval - long (Data Type Value: 0x2f) module.

use crate::data_types::data_type::DataType;

/// Advertising Interval - long.
pub struct AdvertisingIntervalLong {
    /// data length
    pub length: u8,

    /// check uint24 or uint32.
    ///
    /// `true` is uint32, `false` is uint24
    pub is_u32: bool,

    /// Advertising Interval - long
    pub advertising_interval_long: u32,
}

impl AdvertisingIntervalLong {
    /// Create [AdvertisingIntervalLong] from `Advertising Interval - long`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_class::data_types::advertising_interval_long::AdvertisingIntervalLong;
    ///
    /// let advertising_interval_long: u32 = 0x01020304u32;
    /// let result = AdvertisingIntervalLong::new(true, advertising_interval_long);
    /// assert_eq!(5, result.length);
    /// assert_eq!(advertising_interval_long, result.advertising_interval_long);
    ///
    /// let result = AdvertisingIntervalLong::new(false, advertising_interval_long);
    /// assert_eq!(4, result.length);
    /// assert_eq!(
    ///     advertising_interval_long & 0x00ffffff,
    ///     result.advertising_interval_long
    /// );
    /// ```
    pub fn new(is_u32: bool, advertising_interval_long: u32) -> Self {
        Self {
            length: if is_u32 { 5 } else { 4 },
            is_u32,
            advertising_interval_long: if is_u32 {
                advertising_interval_long
            } else {
                advertising_interval_long & 0x00ffffff
            },
        }
    }

    /// Create [AdvertisingIntervalLong] from `Vec<u8>` with offset.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_class::data_types::{advertising_interval_long::AdvertisingIntervalLong, data_type::DataType};
    ///
    /// let advertising_interval: u32 = 0x01020304u32;
    /// let length = 5;
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(AdvertisingIntervalLong::data_type());
    /// data.append(&mut advertising_interval.to_le_bytes().to_vec());
    ///
    /// let result = AdvertisingIntervalLong::from_with_offset(&data, 0);
    /// assert_eq!(length, result.length);
    /// assert_eq!(advertising_interval, result.advertising_interval_long);
    ///
    /// data = Vec::new();
    /// data.push(0);
    /// data.push(length);
    /// data.push(AdvertisingIntervalLong::data_type());
    /// data.append(&mut advertising_interval.to_le_bytes().to_vec());
    /// let result = AdvertisingIntervalLong::from_with_offset(&data, 1);
    /// assert_eq!(length, result.length);
    /// assert_eq!(advertising_interval, result.advertising_interval_long);
    ///
    /// let length = 4;
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(AdvertisingIntervalLong::data_type());
    /// data.append(&mut advertising_interval.to_le_bytes()[..3].to_vec());
    ///
    /// let result = AdvertisingIntervalLong::from_with_offset(&data, 0);
    /// assert_eq!(length, result.length);
    /// assert_eq!(
    ///     advertising_interval & 0x00ffffff,
    ///     result.advertising_interval_long
    /// );
    ///
    /// data = Vec::new();
    /// data.push(0);
    /// data.push(length);
    /// data.push(AdvertisingIntervalLong::data_type());
    /// data.append(&mut advertising_interval.to_le_bytes()[..3].to_vec());
    /// let result = AdvertisingIntervalLong::from_with_offset(&data, 1);
    /// assert_eq!(length, result.length);
    /// assert_eq!(
    ///     advertising_interval & 0x00ffffff,
    ///     result.advertising_interval_long
    /// );
    /// ```
    pub fn from_with_offset(data: &Vec<u8>, offset: usize) -> Self {
        let mut data = data[offset..].to_vec();
        let length = data[0];
        let is_u32 = length == 5;
        if !is_u32 {
            data.push(0x00);
        };
        Self {
            length,
            is_u32,
            advertising_interval_long: u32::from_le_bytes(data[2..6].try_into().unwrap()),
        }
    }

    /// Get Advertising Interval - long(millis).
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_class::data_types::{
    ///    advertising_interval_long::{AdvertisingIntervalLong, ADVINTERVAL_VALUE},
    ///    data_type::DataType,
    /// };
    ///
    /// let advertising_interval_long: u32 = 0x01020304u32;
    /// let result = AdvertisingIntervalLong::new(true, advertising_interval_long);
    /// assert_eq!(
    ///     advertising_interval_long as f32 * ADVINTERVAL_VALUE,
    ///     result.advertising_interval_long_millis()
    /// );
    ///
    /// let result = AdvertisingIntervalLong::new(false, advertising_interval_long);
    /// assert_eq!(
    ///     (advertising_interval_long & 0x00ffffff) as f32 * ADVINTERVAL_VALUE,
    ///     result.advertising_interval_long_millis()
    /// );
    /// ```
    pub fn advertising_interval_long_millis(self) -> f32 {
        self.advertising_interval_long as f32 * ADVINTERVAL_VALUE
    }
}

impl From<&Vec<u8>> for AdvertisingIntervalLong {
    /// Create [AdvertisingIntervalLong] from `Vec<u8>`.
    ///
    /// [`AdvertisingIntervalLong::from_with_offset`]
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_class::data_types::{advertising_interval_long::AdvertisingIntervalLong, data_type::DataType};
    ///
    /// let advertising_interval_long: u32 = 0x01020304u32;
    /// let length = 5;
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(AdvertisingIntervalLong::data_type());
    /// data.append(&mut advertising_interval_long.to_le_bytes().to_vec());
    /// let result = AdvertisingIntervalLong::from(&data);
    /// assert_eq!(length, result.length);
    /// assert_eq!(advertising_interval_long, result.advertising_interval_long);
    ///
    /// let length = 4;
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(AdvertisingIntervalLong::data_type());
    /// data.append(&mut advertising_interval_long.to_le_bytes()[..3].to_vec());
    /// let result = AdvertisingIntervalLong::from(&data);
    /// assert_eq!(length, result.length);
    /// assert_eq!(
    ///     advertising_interval_long & 0x00ffffff,
    ///     result.advertising_interval_long
    /// );
    /// ```
    fn from(data: &Vec<u8>) -> Self {
        Self::from_with_offset(data, 0)
    }
}

/// Units: 0.625 ms
///
/// advInterval value
pub const ADVINTERVAL_VALUE: f32 = 0.625;

impl Into<Vec<u8>> for AdvertisingIntervalLong {
    /// Create `Vec<u8>` from [AdvertisingIntervalLong].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_class::data_types::{advertising_interval_long::AdvertisingIntervalLong, data_type::DataType};
    ///
    /// let advertising_interval_long: u32 = 0x01020304u32;
    /// let result1 = AdvertisingIntervalLong::new(true, advertising_interval_long);

    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(5);
    /// data.push(AdvertisingIntervalLong::data_type());
    /// data.append(&mut advertising_interval_long.to_le_bytes().to_vec());

    /// let into_data: Vec<u8> = result1.into();
    /// assert_eq!(data, into_data);

    /// let result2 = AdvertisingIntervalLong::from(&data);
    /// let into_data: Vec<u8> = result2.into();
    /// assert_eq!(data, into_data);

    /// let result1 = AdvertisingIntervalLong::new(false, advertising_interval_long);

    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(4);
    /// data.push(AdvertisingIntervalLong::data_type());
    /// data.append(&mut advertising_interval_long.to_le_bytes()[..3].to_vec());

    /// let into_data: Vec<u8> = result1.into();
    /// assert_eq!(data, into_data);

    /// let result2 = AdvertisingIntervalLong::from(&data);
    /// let into_data: Vec<u8> = result2.into();
    /// assert_eq!(data, into_data);
    /// ```
    fn into(self) -> Vec<u8> {
        let mut data: Vec<u8> = Vec::new();
        data.push(self.length);
        data.push(Self::data_type());
        if self.is_u32 {
            data.append(&mut self.advertising_interval_long.to_le_bytes().to_vec());
        } else {
            data.append(&mut self.advertising_interval_long.to_le_bytes()[..3].to_vec());
        }
        return data;
    }
}

impl DataType for AdvertisingIntervalLong {
    /// return `0x2f`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_class::data_types::{advertising_interval_long::AdvertisingIntervalLong, data_type::DataType};
    ///
    /// assert_eq!(0x2f, AdvertisingIntervalLong::data_type());
    /// ```
    fn data_type() -> u8 {
        0x2f
    }
}


/// check `Advertising Interval - long` data type.
///
/// # Examples
///
/// ```
/// use ble_data_class::data_types::advertising_interval_long::*;
/// use ble_data_class::data_types::data_type::DataType;
///
/// assert!(is_advertising_interval_long(0x2f));
/// assert!(!is_advertising_interval_long(0x00));
/// ```
pub fn is_advertising_interval_long(data_type: u8) -> bool {
    AdvertisingIntervalLong::data_type() == data_type
}

#[cfg(test)]
mod tests {
    use crate::data_types::{
        advertising_interval_long::{AdvertisingIntervalLong, ADVINTERVAL_VALUE, is_advertising_interval_long},
        data_type::DataType,
    };

    #[test]
    fn test_new() {
        let advertising_interval_long: u32 = 0x01020304u32;
        let result = AdvertisingIntervalLong::new(true, advertising_interval_long);
        assert_eq!(5, result.length);
        assert_eq!(advertising_interval_long, result.advertising_interval_long);

        let result = AdvertisingIntervalLong::new(false, advertising_interval_long);
        assert_eq!(4, result.length);
        assert_eq!(
            advertising_interval_long & 0x00ffffff,
            result.advertising_interval_long
        );
    }

    #[test]
    fn test_from_with_offset() {
        let advertising_interval: u32 = 0x01020304u32;
        let length = 5;
        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(AdvertisingIntervalLong::data_type());
        data.append(&mut advertising_interval.to_le_bytes().to_vec());

        let result = AdvertisingIntervalLong::from_with_offset(&data, 0);
        assert_eq!(length, result.length);
        assert_eq!(advertising_interval, result.advertising_interval_long);

        data = Vec::new();
        data.push(0);
        data.push(length);
        data.push(AdvertisingIntervalLong::data_type());
        data.append(&mut advertising_interval.to_le_bytes().to_vec());
        let result = AdvertisingIntervalLong::from_with_offset(&data, 1);
        assert_eq!(length, result.length);
        assert_eq!(advertising_interval, result.advertising_interval_long);

        let length = 4;
        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(AdvertisingIntervalLong::data_type());
        data.append(&mut advertising_interval.to_le_bytes()[..3].to_vec());

        let result = AdvertisingIntervalLong::from_with_offset(&data, 0);
        assert_eq!(length, result.length);
        assert_eq!(
            advertising_interval & 0x00ffffff,
            result.advertising_interval_long
        );

        data = Vec::new();
        data.push(0);
        data.push(length);
        data.push(AdvertisingIntervalLong::data_type());
        data.append(&mut advertising_interval.to_le_bytes()[..3].to_vec());
        let result = AdvertisingIntervalLong::from_with_offset(&data, 1);
        assert_eq!(length, result.length);
        assert_eq!(
            advertising_interval & 0x00ffffff,
            result.advertising_interval_long
        );
    }

    #[test]
    fn test_advertising_interval_millis() {
        let advertising_interval_long: u32 = 0x01020304u32;
        let result = AdvertisingIntervalLong::new(true, advertising_interval_long);
        assert_eq!(
            advertising_interval_long as f32 * ADVINTERVAL_VALUE,
            result.advertising_interval_long_millis()
        );

        let result = AdvertisingIntervalLong::new(false, advertising_interval_long);
        assert_eq!(
            (advertising_interval_long & 0x00ffffff) as f32 * ADVINTERVAL_VALUE,
            result.advertising_interval_long_millis()
        );
    }

    #[test]
    fn test_from() {
        let advertising_interval_long: u32 = 0x01020304u32;
        let length = 5;
        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(AdvertisingIntervalLong::data_type());
        data.append(&mut advertising_interval_long.to_le_bytes().to_vec());
        let result = AdvertisingIntervalLong::from(&data);
        assert_eq!(length, result.length);
        assert_eq!(advertising_interval_long, result.advertising_interval_long);

        let length = 4;
        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(AdvertisingIntervalLong::data_type());
        data.append(&mut advertising_interval_long.to_le_bytes()[..3].to_vec());
        let result = AdvertisingIntervalLong::from(&data);
        assert_eq!(length, result.length);
        assert_eq!(
            advertising_interval_long & 0x00ffffff,
            result.advertising_interval_long
        );
    }

    #[test]
    fn test_into() {
        let advertising_interval_long: u32 = 0x01020304u32;
        let result1 = AdvertisingIntervalLong::new(true, advertising_interval_long);

        let mut data: Vec<u8> = Vec::new();
        data.push(5);
        data.push(AdvertisingIntervalLong::data_type());
        data.append(&mut advertising_interval_long.to_le_bytes().to_vec());

        let into_data: Vec<u8> = result1.into();
        assert_eq!(data, into_data);

        let result2 = AdvertisingIntervalLong::from(&data);
        let into_data: Vec<u8> = result2.into();
        assert_eq!(data, into_data);

        let result1 = AdvertisingIntervalLong::new(false, advertising_interval_long);

        let mut data: Vec<u8> = Vec::new();
        data.push(4);
        data.push(AdvertisingIntervalLong::data_type());
        data.append(&mut advertising_interval_long.to_le_bytes()[..3].to_vec());

        let into_data: Vec<u8> = result1.into();
        assert_eq!(data, into_data);

        let result2 = AdvertisingIntervalLong::from(&data);
        let into_data: Vec<u8> = result2.into();
        assert_eq!(data, into_data);
    }

    #[test]
    fn test_data_type() {
        assert_eq!(0x2f, AdvertisingIntervalLong::data_type());
    }
    
    #[test]
    fn test_is_advertising_interval_long() {
        assert!(is_advertising_interval_long(0x2f));
        assert!(!is_advertising_interval_long(0x00));
    }

}

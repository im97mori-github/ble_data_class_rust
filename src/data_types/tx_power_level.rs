//! Tx Power Level (Data Type Value: 0x0a) module.

use crate::data_types::data_type::DataType;

/// Tx Power Level.
pub struct TxPowerLevel {
    /// data length
    pub length: u8,

    /// Tx Power Level
    pub tx_power_level: i8,
}

impl TxPowerLevel {
    /// Create [TxPowerLevel] from `Tx Power Level`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::tx_power_level::TxPowerLevel;
    ///
    /// let tx_power_level = -127;
    /// let result = TxPowerLevel::new(tx_power_level);
    /// assert_eq!(2, result.length);
    /// assert_eq!(tx_power_level, result.tx_power_level);
    /// 
    /// let tx_power_level = 127;
    /// let result = TxPowerLevel::new(tx_power_level);
    /// assert_eq!(2, result.length);
    /// assert_eq!(tx_power_level, result.tx_power_level);
    /// ```
    pub fn new(tx_power_level: i8) -> Self {
        Self {
            length: 2,
            tx_power_level,
        }
    }

    /// Create [TxPowerLevel] from `Vec<u8>` with offset.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{tx_power_level::TxPowerLevel, data_type::DataType};
    ///
    /// let tx_power_level = -127;
    /// let length = 2;
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(TxPowerLevel::data_type());
    /// data.push(tx_power_level as u8);
    /// 
    /// let result = TxPowerLevel::from_with_offset(&data, 0);
    /// assert_eq!(length, result.length);
    /// assert_eq!(tx_power_level, result.tx_power_level);
    /// 
    /// data = Vec::new();
    /// data.push(0);
    /// data.push(length);
    /// data.push(TxPowerLevel::data_type());
    /// data.push(tx_power_level as u8);
    /// let result = TxPowerLevel::from_with_offset(&data, 1);
    /// assert_eq!(length, result.length);
    /// assert_eq!(tx_power_level, result.tx_power_level);
    /// 
    /// let tx_power_level = 127;
    /// let length = 2;
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(TxPowerLevel::data_type());
    /// data.push(tx_power_level as u8);
    /// 
    /// let result = TxPowerLevel::from_with_offset(&data, 0);
    /// assert_eq!(length, result.length);
    /// assert_eq!(tx_power_level, result.tx_power_level);
    /// 
    /// data = Vec::new();
    /// data.push(0);
    /// data.push(length);
    /// data.push(TxPowerLevel::data_type());
    /// data.push(tx_power_level as u8);
    /// let result = TxPowerLevel::from_with_offset(&data, 1);
    /// assert_eq!(length, result.length);
    /// assert_eq!(tx_power_level, result.tx_power_level);
    pub fn from_with_offset(data: &Vec<u8>, offset: usize) -> Self {
        let data = data[offset..].to_vec();
        let length = data[0];
        Self {
            length,
            tx_power_level: data[2] as i8,
        }
    }
}

impl From<&Vec<u8>> for TxPowerLevel {
    /// Create [TxPowerLevel] from `Vec<u8>`.
    ///
    /// [`TxPowerLevel::from_with_offset`]
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{tx_power_level::TxPowerLevel, data_type::DataType};
    ///
    /// let tx_power_level = -127;
    /// let length = 2;
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(TxPowerLevel::data_type());
    /// data.push(tx_power_level as u8);
    /// 
    /// let result = TxPowerLevel::from(&data);
    /// assert_eq!(length, result.length);
    /// assert_eq!(tx_power_level, result.tx_power_level);
    /// 
    /// let tx_power_level = 127;
    /// let length = 2;
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(TxPowerLevel::data_type());
    /// data.push(tx_power_level as u8);
    /// 
    /// let result = TxPowerLevel::from(&data);
    /// assert_eq!(length, result.length);
    /// assert_eq!(tx_power_level, result.tx_power_level);
    /// ```
    fn from(data: &Vec<u8>) -> Self {
        Self::from_with_offset(data, 0)
    }
}

impl Into<Vec<u8>> for TxPowerLevel {
    /// Create `Vec<u8>` from [TxPowerLevel].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{tx_power_level::TxPowerLevel, data_type::DataType};
    ///
    /// let tx_power_level = -127;
    /// let result1 = TxPowerLevel::new(tx_power_level);
    /// 
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(2);
    /// data.push(TxPowerLevel::data_type());
    /// data.push(tx_power_level as u8);
    /// 
    /// let into_data: Vec<u8> = result1.into();
    /// assert_eq!(data, into_data);
    /// 
    /// let result2 = TxPowerLevel::from(&data);
    /// let into_data: Vec<u8> = result2.into();
    /// assert_eq!(data, into_data);
    /// 
    /// let tx_power_level = 127;
    /// let result1 = TxPowerLevel::new(tx_power_level);
    /// 
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(2);
    /// data.push(TxPowerLevel::data_type());
    /// data.push(tx_power_level as u8);
    /// 
    /// let into_data: Vec<u8> = result1.into();
    /// assert_eq!(data, into_data);
    /// 
    /// let result2 = TxPowerLevel::from(&data);
    /// let into_data: Vec<u8> = result2.into();
    /// assert_eq!(data, into_data);
    /// ```
    fn into(self) -> Vec<u8> {
        let mut data: Vec<u8> = Vec::new();
        data.push(self.length);
        data.push(Self::data_type());
        data.push(self.tx_power_level as u8);
        return data;
    }
}

impl DataType for TxPowerLevel {
    /// return `0x0a`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{tx_power_level::TxPowerLevel, data_type::DataType};
    ///
    /// assert_eq!(0x0a, TxPowerLevel::data_type());
    /// ```
    fn data_type() -> u8 {
        0x0a
    }
}

/// check `Tx Power Level` data type.
///
/// # Examples
///
/// ```
/// use ble_data_struct::data_types::tx_power_level::*;
/// use ble_data_struct::data_types::data_type::DataType;
///
/// assert!(is_tx_power_level(0x0a));
/// assert!(!is_tx_power_level(0x00));
/// ```
pub fn is_tx_power_level(data_type: u8) -> bool {
    TxPowerLevel::data_type() == data_type
}

#[cfg(test)]
mod tests {
    use crate::data_types::{data_type::DataType, tx_power_level::*};

    #[test]
    fn test_new() {
        let tx_power_level = -127;
        let result = TxPowerLevel::new(tx_power_level);
        assert_eq!(2, result.length);
        assert_eq!(tx_power_level, result.tx_power_level);

        let tx_power_level = 127;
        let result = TxPowerLevel::new(tx_power_level);
        assert_eq!(2, result.length);
        assert_eq!(tx_power_level, result.tx_power_level);
    }

    #[test]
    fn test_from_with_offset() {
        let tx_power_level = -127;
        let length = 2;
        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(TxPowerLevel::data_type());
        data.push(tx_power_level as u8);

        let result = TxPowerLevel::from_with_offset(&data, 0);
        assert_eq!(length, result.length);
        assert_eq!(tx_power_level, result.tx_power_level);

        data = Vec::new();
        data.push(0);
        data.push(length);
        data.push(TxPowerLevel::data_type());
        data.push(tx_power_level as u8);
        let result = TxPowerLevel::from_with_offset(&data, 1);
        assert_eq!(length, result.length);
        assert_eq!(tx_power_level, result.tx_power_level);

        let tx_power_level = 127;
        let length = 2;
        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(TxPowerLevel::data_type());
        data.push(tx_power_level as u8);

        let result = TxPowerLevel::from_with_offset(&data, 0);
        assert_eq!(length, result.length);
        assert_eq!(tx_power_level, result.tx_power_level);

        data = Vec::new();
        data.push(0);
        data.push(length);
        data.push(TxPowerLevel::data_type());
        data.push(tx_power_level as u8);
        let result = TxPowerLevel::from_with_offset(&data, 1);
        assert_eq!(length, result.length);
        assert_eq!(tx_power_level, result.tx_power_level);
    }

    #[test]
    fn test_from() {
        let tx_power_level = -127;
        let length = 2;
        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(TxPowerLevel::data_type());
        data.push(tx_power_level as u8);

        let result = TxPowerLevel::from(&data);
        assert_eq!(length, result.length);
        assert_eq!(tx_power_level, result.tx_power_level);

        let tx_power_level = 127;
        let length = 2;
        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(TxPowerLevel::data_type());
        data.push(tx_power_level as u8);

        let result = TxPowerLevel::from(&data);
        assert_eq!(length, result.length);
        assert_eq!(tx_power_level, result.tx_power_level);
    }

    #[test]
    fn test_into() {
        let tx_power_level = -127;
        let result1 = TxPowerLevel::new(tx_power_level);

        let mut data: Vec<u8> = Vec::new();
        data.push(2);
        data.push(TxPowerLevel::data_type());
        data.push(tx_power_level as u8);

        let into_data: Vec<u8> = result1.into();
        assert_eq!(data, into_data);

        let result2 = TxPowerLevel::from(&data);
        let into_data: Vec<u8> = result2.into();
        assert_eq!(data, into_data);

        let tx_power_level = 127;
        let result1 = TxPowerLevel::new(tx_power_level);

        let mut data: Vec<u8> = Vec::new();
        data.push(2);
        data.push(TxPowerLevel::data_type());
        data.push(tx_power_level as u8);

        let into_data: Vec<u8> = result1.into();
        assert_eq!(data, into_data);

        let result2 = TxPowerLevel::from(&data);
        let into_data: Vec<u8> = result2.into();
        assert_eq!(data, into_data);
    }

    #[test]
    fn test_data_type() {
        assert_eq!(0x0a, TxPowerLevel::data_type());
    }

    #[test]
    fn test_is_tx_power_level() {
        assert!(is_tx_power_level(0x0a));
        assert!(!is_tx_power_level(0x00));
    }
}

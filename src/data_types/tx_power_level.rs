//! Tx Power Level (Data Type Value: 0x0a) module.

use crate::data_types::data_type::DataType;

/// Tx Power Level.
#[derive(Debug)]
pub struct TxPowerLevel {
    /// data length
    pub length: u8,

    /// Tx Power Level
    pub tx_power_level: i8,
}

impl TxPowerLevel {
    /// Create [`TxPowerLevel`] from `Tx Power Level`.
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
}

impl TryFrom<&Vec<u8>> for TxPowerLevel {
    type Error = String;
    /// Create [`TxPowerLevel`] from `Vec<u8>`.
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
    /// let result = TxPowerLevel::try_from(&data);
    /// assert!(result.is_ok());
    /// let data_type = result.unwrap();
    /// assert_eq!(length, data_type.length);
    /// assert_eq!(tx_power_level, data_type.tx_power_level);
    ///
    /// let tx_power_level = 127;
    /// let length = 2;
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(TxPowerLevel::data_type());
    /// data.push(tx_power_level as u8);
    ///
    /// let result = TxPowerLevel::try_from(&data);
    /// assert!(result.is_ok());
    /// let data_type = result.unwrap();
    /// assert_eq!(length, data_type.length);
    /// assert_eq!(tx_power_level, data_type.tx_power_level);
    ///
    /// let data: Vec<u8> = Vec::new();
    /// let result = TxPowerLevel::try_from(&data);
    /// assert!(result.is_err());
    /// assert_eq!(
    ///     format!("Invalid data size :{}", data.len()),
    ///     result.unwrap_err()
    /// );
    /// ```
    fn try_from(value: &Vec<u8>) -> Result<Self, String> {
        let len = value.len();
        if len < 3 {
            return Err(format!("Invalid data size :{}", len).to_string());
        }
        let length = value[0];
        Ok(Self {
            length,
            tx_power_level: value[2] as i8,
        })
    }
}

impl Into<Vec<u8>> for TxPowerLevel {
    /// Create `Vec<u8>` from [`TxPowerLevel`].
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
    /// let result2 = TxPowerLevel::try_from(&data);
    /// assert!(result2.is_ok());
    /// let data_type = result2.unwrap();
    /// let into_data: Vec<u8> = data_type.into();
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
    /// let result2 = TxPowerLevel::try_from(&data);
    /// assert!(result2.is_ok());
    /// let data_type = result2.unwrap();
    /// let into_data: Vec<u8> = data_type.into();
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
    fn test_from() {
        let tx_power_level = -127;
        let length = 2;
        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(TxPowerLevel::data_type());
        data.push(tx_power_level as u8);

        let result = TxPowerLevel::try_from(&data);
        assert!(result.is_ok());
        let data_type = result.unwrap();
        assert_eq!(length, data_type.length);
        assert_eq!(tx_power_level, data_type.tx_power_level);

        let tx_power_level = 127;
        let length = 2;
        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(TxPowerLevel::data_type());
        data.push(tx_power_level as u8);

        let result = TxPowerLevel::try_from(&data);
        assert!(result.is_ok());
        let data_type = result.unwrap();
        assert_eq!(length, data_type.length);
        assert_eq!(tx_power_level, data_type.tx_power_level);

        let data: Vec<u8> = Vec::new();
        let result = TxPowerLevel::try_from(&data);
        assert!(result.is_err());
        assert_eq!(
            format!("Invalid data size :{}", data.len()),
            result.unwrap_err()
        );
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

        let result2 = TxPowerLevel::try_from(&data);
        assert!(result2.is_ok());
        let data_type = result2.unwrap();
        let into_data: Vec<u8> = data_type.into();
        assert_eq!(data, into_data);

        let tx_power_level = 127;
        let result1 = TxPowerLevel::new(tx_power_level);

        let mut data: Vec<u8> = Vec::new();
        data.push(2);
        data.push(TxPowerLevel::data_type());
        data.push(tx_power_level as u8);

        let into_data: Vec<u8> = result1.into();
        assert_eq!(data, into_data);

        let result2 = TxPowerLevel::try_from(&data);
        assert!(result2.is_ok());
        let data_type = result2.unwrap();
        let into_data: Vec<u8> = data_type.into();
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

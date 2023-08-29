//! LE Secure Connections Confirmation Value (Data Type Value: 0x22) module.

use crate::data_types::data_type::DataType;

/// LE Secure Connections Confirmation Value.
pub struct LeSecureConnectionsConfirmationValue {
    /// data length
    pub length: u8,

    /// LE Secure Connections Confirmation Value
    pub le_secure_connections_confirmation_value: u128,
}

impl LeSecureConnectionsConfirmationValue {
    /// Create [LeSecureConnectionsConfirmationValue] from `LE Secure Connections Confirmation Value`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_class::data_types::le_secure_connections_confirmation_value::LeSecureConnectionsConfirmationValue;
    ///
    /// let le_secure_connections_confirmation_value = 0x0102030405060708090a0b0c0d0e0f10u128;
    /// let result = LeSecureConnectionsConfirmationValue::new(le_secure_connections_confirmation_value);
    /// assert_eq!(17, result.length);
    /// assert_eq!(le_secure_connections_confirmation_value, result.le_secure_connections_confirmation_value);
    /// ```
    pub fn new(le_secure_connections_confirmation_value: u128) -> Self {
        Self {
            length: 17,
            le_secure_connections_confirmation_value,
        }
    }

    /// Create [LeSecureConnectionsConfirmationValue] from `Vec<u8>` with offset.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_class::{BASE_UUID, data_types::{le_secure_connections_confirmation_value::LeSecureConnectionsConfirmationValue, data_type::DataType}};
    ///
    /// let le_secure_connections_confirmation_value = 0x0102030405060708090a0b0c0d0e0f10u128;
    /// let length = 17;
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(LeSecureConnectionsConfirmationValue::data_type());
    /// data.append(&mut le_secure_connections_confirmation_value.to_le_bytes().to_vec());
    /// 
    /// let result = LeSecureConnectionsConfirmationValue::from_with_offset(&data, 0);
    /// assert_eq!(length, result.length);
    /// assert_eq!(le_secure_connections_confirmation_value, result.le_secure_connections_confirmation_value);
    /// 
    /// data = Vec::new();
    /// data.push(0);
    /// data.push(length);
    /// data.push(LeSecureConnectionsConfirmationValue::data_type());
    /// data.append(&mut le_secure_connections_confirmation_value.to_le_bytes().to_vec());
    /// let result = LeSecureConnectionsConfirmationValue::from_with_offset(&data, 1);
    /// assert_eq!(length, result.length);
    /// assert_eq!(le_secure_connections_confirmation_value, result.le_secure_connections_confirmation_value);
    /// ```
    pub fn from_with_offset(data: &Vec<u8>, offset: usize) -> Self {
        let data = data[offset..].to_vec();
        let length = data[0];
        Self {
            length,
            le_secure_connections_confirmation_value: u128::from_le_bytes(data[2..18].try_into().unwrap()),
        }
    }
}

impl From<&Vec<u8>> for LeSecureConnectionsConfirmationValue {
    /// Create [LeSecureConnectionsConfirmationValue] from `Vec<u8>`.
    ///
    /// [`LeSecureConnectionsConfirmationValue::from_with_offset`]
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_class::{BASE_UUID, data_types::{le_secure_connections_confirmation_value::LeSecureConnectionsConfirmationValue, data_type::DataType}};
    ///
    /// let le_secure_connections_confirmation_value = 0x0102030405060708090a0b0c0d0e0f10u128;
    /// let length = 17;
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(LeSecureConnectionsConfirmationValue::data_type());
    /// data.append(&mut le_secure_connections_confirmation_value.to_le_bytes().to_vec());
    /// 
    /// let result = LeSecureConnectionsConfirmationValue::from(&data);
    /// assert_eq!(length, result.length);
    /// assert_eq!(le_secure_connections_confirmation_value, result.le_secure_connections_confirmation_value);
    /// ```
    fn from(data: &Vec<u8>) -> Self {
        Self::from_with_offset(data, 0)
    }
}

impl Into<Vec<u8>> for LeSecureConnectionsConfirmationValue {
    /// Create `Vec<u8>` from [LeSecureConnectionsConfirmationValue].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_class::{BASE_UUID, data_types::{le_secure_connections_confirmation_value::LeSecureConnectionsConfirmationValue, data_type::DataType}};
    ///
    /// let le_secure_connections_confirmation_value = 0x0102030405060708090a0b0c0d0e0f10u128;
    /// let result1 = LeSecureConnectionsConfirmationValue::new(le_secure_connections_confirmation_value);
    /// 
    /// let length = 17;
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(LeSecureConnectionsConfirmationValue::data_type());
    /// data.append(&mut le_secure_connections_confirmation_value.to_le_bytes().to_vec());
    /// 
    /// let into_data: Vec<u8> = result1.into();
    /// assert_eq!(data, into_data);
    /// 
    /// let result2 = LeSecureConnectionsConfirmationValue::from(&data);
    /// let into_data: Vec<u8> = result2.into();
    /// assert_eq!(data, into_data);
    /// ```
    fn into(self) -> Vec<u8> {
        let mut data: Vec<u8> = Vec::new();
        data.push(self.length);
        data.push(Self::data_type());
        data.append(&mut self.le_secure_connections_confirmation_value.to_le_bytes().to_vec());
        return data;
    }
}

impl DataType for LeSecureConnectionsConfirmationValue {
    /// return `0x22`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_class::data_types::{le_secure_connections_confirmation_value::LeSecureConnectionsConfirmationValue, data_type::DataType};
    ///
    /// assert_eq!(0x22, LeSecureConnectionsConfirmationValue::data_type());
    /// ```
    fn data_type() -> u8 {
        0x22
    }
}

/// check `LE Secure Connections Confirmation Value.` data type.
///
/// # Examples
///
/// ```
/// use ble_data_class::data_types::le_secure_connections_confirmation_value::*;
/// use ble_data_class::data_types::data_type::DataType;
///
/// assert!(is_le_secure_connections_confirmation_value(0x22));
/// assert!(!is_le_secure_connections_confirmation_value(0x00));
/// ```
pub fn is_le_secure_connections_confirmation_value(data_type: u8) -> bool {
    LeSecureConnectionsConfirmationValue::data_type() == data_type
}

#[cfg(test)]
mod tests {
    use crate::data_types::{data_type::DataType, le_secure_connections_confirmation_value::*};

    #[test]
    fn test_new() {
        let le_secure_connections_confirmation_value = 0x0102030405060708090a0b0c0d0e0f10u128;
        let result = LeSecureConnectionsConfirmationValue::new(le_secure_connections_confirmation_value);
        assert_eq!(17, result.length);
        assert_eq!(le_secure_connections_confirmation_value, result.le_secure_connections_confirmation_value);
    }

    #[test]
    fn test_from_with_offset() {
        let le_secure_connections_confirmation_value = 0x0102030405060708090a0b0c0d0e0f10u128;
        let length = 17;
        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(LeSecureConnectionsConfirmationValue::data_type());
        data.append(&mut le_secure_connections_confirmation_value.to_le_bytes().to_vec());

        let result = LeSecureConnectionsConfirmationValue::from_with_offset(&data, 0);
        assert_eq!(length, result.length);
        assert_eq!(le_secure_connections_confirmation_value, result.le_secure_connections_confirmation_value);

        data = Vec::new();
        data.push(0);
        data.push(length);
        data.push(LeSecureConnectionsConfirmationValue::data_type());
        data.append(&mut le_secure_connections_confirmation_value.to_le_bytes().to_vec());
        let result = LeSecureConnectionsConfirmationValue::from_with_offset(&data, 1);
        assert_eq!(length, result.length);
        assert_eq!(le_secure_connections_confirmation_value, result.le_secure_connections_confirmation_value);
    }

    #[test]
    fn test_from() {
        let le_secure_connections_confirmation_value = 0x0102030405060708090a0b0c0d0e0f10u128;
        let length = 17;
        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(LeSecureConnectionsConfirmationValue::data_type());
        data.append(&mut le_secure_connections_confirmation_value.to_le_bytes().to_vec());

        let result = LeSecureConnectionsConfirmationValue::from(&data);
        assert_eq!(length, result.length);
        assert_eq!(le_secure_connections_confirmation_value, result.le_secure_connections_confirmation_value);
    }

    #[test]
    fn test_into() {
        let le_secure_connections_confirmation_value = 0x0102030405060708090a0b0c0d0e0f10u128;
        let result1 = LeSecureConnectionsConfirmationValue::new(le_secure_connections_confirmation_value);

        let length = 17;
        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(LeSecureConnectionsConfirmationValue::data_type());
        data.append(&mut le_secure_connections_confirmation_value.to_le_bytes().to_vec());

        let into_data: Vec<u8> = result1.into();
        assert_eq!(data, into_data);

        let result2 = LeSecureConnectionsConfirmationValue::from(&data);
        let into_data: Vec<u8> = result2.into();
        assert_eq!(data, into_data);
    }

    #[test]
    fn test_data_type() {
        assert_eq!(0x22, LeSecureConnectionsConfirmationValue::data_type());
    }

    #[test]
    fn test_is_le_secure_connections_confirmation_value() {
        assert!(is_le_secure_connections_confirmation_value(0x22));
        assert!(!is_le_secure_connections_confirmation_value(0x00));
    }
}

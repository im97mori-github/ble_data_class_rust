//! Security Manager TK Value (Data Type Value: 0x10) module.

use crate::data_types::data_type::DataType;

/// Security Manager TK Value.
pub struct SecurityManagerTkValue {
    /// data length
    pub length: u8,

    /// Security Manager TK Value
    pub security_manager_tk_value: u128,
}

impl SecurityManagerTkValue {
    /// Create [SecurityManagerTkValue] from Parameters.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::security_manager_tk_value::SecurityManagerTkValue;
    ///
    /// let security_manager_tk_value = 0x0102030405060708090a0b0c0d0e0f10u128;
    /// let result = SecurityManagerTkValue::new(security_manager_tk_value);
    /// assert_eq!(17, result.length);
    /// assert_eq!(security_manager_tk_value, result.security_manager_tk_value);
    /// ```
    pub fn new(security_manager_tk_value: u128) -> Self {
        Self {
            length: 17,
            security_manager_tk_value,
        }
    }

    /// Create [SecurityManagerTkValue] from `Vec<u8>` with offset.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::{BASE_UUID, data_types::{security_manager_tk_value::SecurityManagerTkValue, data_type::DataType}};
    ///
    /// let security_manager_tk_value = 0x0102030405060708090a0b0c0d0e0f10u128;
    /// let length = 17;
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(SecurityManagerTkValue::data_type());
    /// data.append(&mut security_manager_tk_value.to_le_bytes().to_vec());
    /// 
    /// let result = SecurityManagerTkValue::from_with_offset(&data, 0);
    /// assert_eq!(length, result.length);
    /// assert_eq!(security_manager_tk_value, result.security_manager_tk_value);
    /// 
    /// data = Vec::new();
    /// data.push(0);
    /// data.push(length);
    /// data.push(SecurityManagerTkValue::data_type());
    /// data.append(&mut security_manager_tk_value.to_le_bytes().to_vec());
    /// let result = SecurityManagerTkValue::from_with_offset(&data, 1);
    /// assert_eq!(length, result.length);
    /// assert_eq!(security_manager_tk_value, result.security_manager_tk_value);
    /// ```
    pub fn from_with_offset(data: &Vec<u8>, offset: usize) -> Self {
        let data = data[offset..].to_vec();
        let length = data[0];
        Self {
            length,
            security_manager_tk_value: u128::from_le_bytes(data[2..18].try_into().unwrap()),
        }
    }
}

impl From<&Vec<u8>> for SecurityManagerTkValue {
    /// Create [SecurityManagerTkValue] from `Vec<u8>`.
    ///
    /// [`SecurityManagerTkValue::from_with_offset`]
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::{BASE_UUID, data_types::{security_manager_tk_value::SecurityManagerTkValue, data_type::DataType}};
    ///
    /// let security_manager_tk_value = 0x0102030405060708090a0b0c0d0e0f10u128;
    /// let length = 17;
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(SecurityManagerTkValue::data_type());
    /// data.append(&mut security_manager_tk_value.to_le_bytes().to_vec());
    /// 
    /// let result = SecurityManagerTkValue::from(&data);
    /// assert_eq!(length, result.length);
    /// assert_eq!(security_manager_tk_value, result.security_manager_tk_value);
    /// ```
    fn from(data: &Vec<u8>) -> Self {
        Self::from_with_offset(data, 0)
    }
}

impl Into<Vec<u8>> for SecurityManagerTkValue {
    /// Create `Vec<u8>` from [SecurityManagerTkValue].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::{BASE_UUID, data_types::{security_manager_tk_value::SecurityManagerTkValue, data_type::DataType}};
    ///
    /// let security_manager_tk_value = 0x0102030405060708090a0b0c0d0e0f10u128;
    /// let result1 = SecurityManagerTkValue::new(security_manager_tk_value);
    /// 
    /// let length = 17;
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(SecurityManagerTkValue::data_type());
    /// data.append(&mut security_manager_tk_value.to_le_bytes().to_vec());
    /// 
    /// let into_data: Vec<u8> = result1.into();
    /// assert_eq!(data, into_data);
    /// 
    /// let result2 = SecurityManagerTkValue::from(&data);
    /// let into_data: Vec<u8> = result2.into();
    /// assert_eq!(data, into_data);
    /// ```
    fn into(self) -> Vec<u8> {
        let mut data: Vec<u8> = Vec::new();
        data.push(self.length);
        data.push(Self::data_type());
        data.append(&mut self.security_manager_tk_value.to_le_bytes().to_vec());
        return data;
    }
}

impl DataType for SecurityManagerTkValue {
    /// return `0x10`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{security_manager_tk_value::SecurityManagerTkValue, data_type::DataType};
    ///
    /// assert_eq!(0x10, SecurityManagerTkValue::data_type());
    /// ```
    fn data_type() -> u8 {
        0x10
    }
}

/// check `Security Manager TK Value.` data type.
///
/// # Examples
///
/// ```
/// use ble_data_struct::data_types::security_manager_tk_value::*;
/// use ble_data_struct::data_types::data_type::DataType;
///
/// assert!(is_security_manager_tk_value(0x10));
/// assert!(!is_security_manager_tk_value(0x00));
/// ```
pub fn is_security_manager_tk_value(data_type: u8) -> bool {
    SecurityManagerTkValue::data_type() == data_type
}

#[cfg(test)]
mod tests {
    use crate::data_types::{data_type::DataType, security_manager_tk_value::*};

    #[test]
    fn test_new() {
        let security_manager_tk_value = 0x0102030405060708090a0b0c0d0e0f10u128;
        let result = SecurityManagerTkValue::new(security_manager_tk_value);
        assert_eq!(17, result.length);
        assert_eq!(security_manager_tk_value, result.security_manager_tk_value);
    }

    #[test]
    fn test_from_with_offset() {
        let security_manager_tk_value = 0x0102030405060708090a0b0c0d0e0f10u128;
        let length = 17;
        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(SecurityManagerTkValue::data_type());
        data.append(&mut security_manager_tk_value.to_le_bytes().to_vec());

        let result = SecurityManagerTkValue::from_with_offset(&data, 0);
        assert_eq!(length, result.length);
        assert_eq!(security_manager_tk_value, result.security_manager_tk_value);

        data = Vec::new();
        data.push(0);
        data.push(length);
        data.push(SecurityManagerTkValue::data_type());
        data.append(&mut security_manager_tk_value.to_le_bytes().to_vec());
        let result = SecurityManagerTkValue::from_with_offset(&data, 1);
        assert_eq!(length, result.length);
        assert_eq!(security_manager_tk_value, result.security_manager_tk_value);
    }

    #[test]
    fn test_from() {
        let security_manager_tk_value = 0x0102030405060708090a0b0c0d0e0f10u128;
        let length = 17;
        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(SecurityManagerTkValue::data_type());
        data.append(&mut security_manager_tk_value.to_le_bytes().to_vec());

        let result = SecurityManagerTkValue::from(&data);
        assert_eq!(length, result.length);
        assert_eq!(security_manager_tk_value, result.security_manager_tk_value);
    }

    #[test]
    fn test_into() {
        let security_manager_tk_value = 0x0102030405060708090a0b0c0d0e0f10u128;
        let result1 = SecurityManagerTkValue::new(security_manager_tk_value);

        let length = 17;
        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(SecurityManagerTkValue::data_type());
        data.append(&mut security_manager_tk_value.to_le_bytes().to_vec());

        let into_data: Vec<u8> = result1.into();
        assert_eq!(data, into_data);

        let result2 = SecurityManagerTkValue::from(&data);
        let into_data: Vec<u8> = result2.into();
        assert_eq!(data, into_data);
    }

    #[test]
    fn test_data_type() {
        assert_eq!(0x10, SecurityManagerTkValue::data_type());
    }

    #[test]
    fn test_is_security_manager_tk_value() {
        assert!(is_security_manager_tk_value(0x10));
        assert!(!is_security_manager_tk_value(0x00));
    }
}

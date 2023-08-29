//! Secure Simple Pairing Hash C-192 (Data Type Value: 0x0e) module.

use crate::data_types::data_type::DataType;

/// Secure Simple Pairing Hash C-192.
pub struct SecureSimplePairingHashC192 {
    /// data length
    pub length: u8,

    /// Secure Simple Pairing Hash C-192
    pub secure_simple_pairing_hash_c192: u128,
}

impl SecureSimplePairingHashC192 {
    /// Create [SecureSimplePairingHashC192] from `Secure Simple Pairing Hash C-192`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_class::data_types::secure_simple_pairing_hash_c192::SecureSimplePairingHashC192;
    ///
    /// let secure_simple_pairing_hash_c192 = 0x0102030405060708090a0b0c0d0e0f10u128;
    /// let result = SecureSimplePairingHashC192::new(secure_simple_pairing_hash_c192);
    /// assert_eq!(17, result.length);
    /// assert_eq!(secure_simple_pairing_hash_c192, result.secure_simple_pairing_hash_c192);
    /// ```
    pub fn new(secure_simple_pairing_hash_c192: u128) -> Self {
        Self {
            length: 17,
            secure_simple_pairing_hash_c192,
        }
    }

    /// Create [SecureSimplePairingHashC192] from `Vec<u8>` with offset.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_class::{BASE_UUID, data_types::{secure_simple_pairing_hash_c192::SecureSimplePairingHashC192, data_type::DataType}};
    ///
    /// let secure_simple_pairing_hash_c192 = 0x0102030405060708090a0b0c0d0e0f10u128;
    /// let length = 17;
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(SecureSimplePairingHashC192::data_type());
    /// data.append(&mut secure_simple_pairing_hash_c192.to_le_bytes().to_vec());
    /// 
    /// let result = SecureSimplePairingHashC192::from_with_offset(&data, 0);
    /// assert_eq!(length, result.length);
    /// assert_eq!(secure_simple_pairing_hash_c192, result.secure_simple_pairing_hash_c192);
    /// 
    /// data = Vec::new();
    /// data.push(0);
    /// data.push(length);
    /// data.push(SecureSimplePairingHashC192::data_type());
    /// data.append(&mut secure_simple_pairing_hash_c192.to_le_bytes().to_vec());
    /// let result = SecureSimplePairingHashC192::from_with_offset(&data, 1);
    /// assert_eq!(length, result.length);
    /// assert_eq!(secure_simple_pairing_hash_c192, result.secure_simple_pairing_hash_c192);
    /// ```
    pub fn from_with_offset(data: &Vec<u8>, offset: usize) -> Self {
        let data = data[offset..].to_vec();
        let length = data[0];
        Self {
            length,
            secure_simple_pairing_hash_c192: u128::from_le_bytes(data[2..18].try_into().unwrap()),
        }
    }
}

impl From<&Vec<u8>> for SecureSimplePairingHashC192 {
    /// Create [SecureSimplePairingHashC192] from `Vec<u8>`.
    ///
    /// [`SecureSimplePairingHashC192::from_with_offset`]
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_class::{BASE_UUID, data_types::{secure_simple_pairing_hash_c192::SecureSimplePairingHashC192, data_type::DataType}};
    ///
    /// let secure_simple_pairing_hash_c192 = 0x0102030405060708090a0b0c0d0e0f10u128;
    /// let length = 17;
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(SecureSimplePairingHashC192::data_type());
    /// data.append(&mut secure_simple_pairing_hash_c192.to_le_bytes().to_vec());
    /// 
    /// let result = SecureSimplePairingHashC192::from(&data);
    /// assert_eq!(length, result.length);
    /// assert_eq!(secure_simple_pairing_hash_c192, result.secure_simple_pairing_hash_c192);
    /// ```
    fn from(data: &Vec<u8>) -> Self {
        Self::from_with_offset(data, 0)
    }
}

impl Into<Vec<u8>> for SecureSimplePairingHashC192 {
    /// Create `Vec<u8>` from [SecureSimplePairingHashC192].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_class::{BASE_UUID, data_types::{secure_simple_pairing_hash_c192::SecureSimplePairingHashC192, data_type::DataType}};
    ///
    /// let secure_simple_pairing_hash_c192 = 0x0102030405060708090a0b0c0d0e0f10u128;
    /// let result1 = SecureSimplePairingHashC192::new(secure_simple_pairing_hash_c192);
    /// 
    /// let length = 17;
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(SecureSimplePairingHashC192::data_type());
    /// data.append(&mut secure_simple_pairing_hash_c192.to_le_bytes().to_vec());
    /// 
    /// let into_data: Vec<u8> = result1.into();
    /// assert_eq!(data, into_data);
    /// 
    /// let result2 = SecureSimplePairingHashC192::from(&data);
    /// let into_data: Vec<u8> = result2.into();
    /// assert_eq!(data, into_data);
    /// ```
    fn into(self) -> Vec<u8> {
        let mut data: Vec<u8> = Vec::new();
        data.push(self.length);
        data.push(Self::data_type());
        data.append(&mut self.secure_simple_pairing_hash_c192.to_le_bytes().to_vec());
        return data;
    }
}

impl DataType for SecureSimplePairingHashC192 {
    /// return `0x0e`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_class::data_types::{secure_simple_pairing_hash_c192::SecureSimplePairingHashC192, data_type::DataType};
    ///
    /// assert_eq!(0x0e, SecureSimplePairingHashC192::data_type());
    /// ```
    fn data_type() -> u8 {
        0x0e
    }
}

/// check `Secure Simple Pairing Hash C-192.` data type.
///
/// # Examples
///
/// ```
/// use ble_data_class::data_types::secure_simple_pairing_hash_c192::*;
/// use ble_data_class::data_types::data_type::DataType;
///
/// assert!(is_secure_simple_pairing_hash_c192(0x0e));
/// assert!(!is_secure_simple_pairing_hash_c192(0x00));
/// ```
pub fn is_secure_simple_pairing_hash_c192(data_type: u8) -> bool {
    SecureSimplePairingHashC192::data_type() == data_type
}

#[cfg(test)]
mod tests {
    use crate::data_types::{data_type::DataType, secure_simple_pairing_hash_c192::*};

    #[test]
    fn test_new() {
        let secure_simple_pairing_hash_c192 = 0x0102030405060708090a0b0c0d0e0f10u128;
        let result = SecureSimplePairingHashC192::new(secure_simple_pairing_hash_c192);
        assert_eq!(17, result.length);
        assert_eq!(secure_simple_pairing_hash_c192, result.secure_simple_pairing_hash_c192);
    }

    #[test]
    fn test_from_with_offset() {
        let secure_simple_pairing_hash_c192 = 0x0102030405060708090a0b0c0d0e0f10u128;
        let length = 17;
        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(SecureSimplePairingHashC192::data_type());
        data.append(&mut secure_simple_pairing_hash_c192.to_le_bytes().to_vec());

        let result = SecureSimplePairingHashC192::from_with_offset(&data, 0);
        assert_eq!(length, result.length);
        assert_eq!(secure_simple_pairing_hash_c192, result.secure_simple_pairing_hash_c192);

        data = Vec::new();
        data.push(0);
        data.push(length);
        data.push(SecureSimplePairingHashC192::data_type());
        data.append(&mut secure_simple_pairing_hash_c192.to_le_bytes().to_vec());
        let result = SecureSimplePairingHashC192::from_with_offset(&data, 1);
        assert_eq!(length, result.length);
        assert_eq!(secure_simple_pairing_hash_c192, result.secure_simple_pairing_hash_c192);
    }

    #[test]
    fn test_from() {
        let secure_simple_pairing_hash_c192 = 0x0102030405060708090a0b0c0d0e0f10u128;
        let length = 17;
        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(SecureSimplePairingHashC192::data_type());
        data.append(&mut secure_simple_pairing_hash_c192.to_le_bytes().to_vec());

        let result = SecureSimplePairingHashC192::from(&data);
        assert_eq!(length, result.length);
        assert_eq!(secure_simple_pairing_hash_c192, result.secure_simple_pairing_hash_c192);
    }

    #[test]
    fn test_into() {
        let secure_simple_pairing_hash_c192 = 0x0102030405060708090a0b0c0d0e0f10u128;
        let result1 = SecureSimplePairingHashC192::new(secure_simple_pairing_hash_c192);

        let length = 17;
        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(SecureSimplePairingHashC192::data_type());
        data.append(&mut secure_simple_pairing_hash_c192.to_le_bytes().to_vec());

        let into_data: Vec<u8> = result1.into();
        assert_eq!(data, into_data);

        let result2 = SecureSimplePairingHashC192::from(&data);
        let into_data: Vec<u8> = result2.into();
        assert_eq!(data, into_data);
    }

    #[test]
    fn test_data_type() {
        assert_eq!(0x0e, SecureSimplePairingHashC192::data_type());
    }

    #[test]
    fn test_is_secure_simple_pairing_hash_c192() {
        assert!(is_secure_simple_pairing_hash_c192(0x0e));
        assert!(!is_secure_simple_pairing_hash_c192(0x00));
    }
}

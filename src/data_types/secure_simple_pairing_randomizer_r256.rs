//! Secure Simple Pairing Randomizer R-256 (Data Type Value: 0x1e) module.

use crate::data_types::data_type::DataType;

/// Secure Simple Pairing Randomizer R-256.
#[derive(Debug, PartialEq, Clone)]
pub struct SecureSimplePairingRandomizerR256 {
    /// data length
    pub length: u8,

    /// Secure Simple Pairing Randomizer R-256
    pub secure_simple_pairing_randomizer_r256: u128,
}

impl SecureSimplePairingRandomizerR256 {
    /// Create [`SecureSimplePairingRandomizerR256`] from `Secure Simple Pairing Randomizer R-256`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::secure_simple_pairing_randomizer_r256::SecureSimplePairingRandomizerR256;
    ///
    /// let secure_simple_pairing_randomizer_r256 = 0x0102030405060708090a0b0c0d0e0f10u128;
    /// let result = SecureSimplePairingRandomizerR256::new(secure_simple_pairing_randomizer_r256);
    /// assert_eq!(17, result.length);
    /// assert_eq!(secure_simple_pairing_randomizer_r256, result.secure_simple_pairing_randomizer_r256);
    /// ```
    pub fn new(secure_simple_pairing_randomizer_r256: u128) -> Self {
        Self {
            length: 17,
            secure_simple_pairing_randomizer_r256,
        }
    }
}

impl TryFrom<&Vec<u8>> for SecureSimplePairingRandomizerR256 {
    type Error = String;
    /// Create [`SecureSimplePairingRandomizerR256`] from [`Vec<u8>`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::{BASE_UUID, data_types::{secure_simple_pairing_randomizer_r256::SecureSimplePairingRandomizerR256, data_type::DataType}};
    ///
    /// let secure_simple_pairing_randomizer_r256 = 0x0102030405060708090a0b0c0d0e0f10u128;
    /// let length = 17;
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(SecureSimplePairingRandomizerR256::data_type());
    /// data.append(&mut secure_simple_pairing_randomizer_r256.to_le_bytes().to_vec());
    ///
    /// let result = SecureSimplePairingRandomizerR256::try_from(&data);
    /// assert!(result.is_ok());
    /// let data_type = result.unwrap();
    /// assert_eq!(length, data_type.length);
    /// assert_eq!(
    ///     secure_simple_pairing_randomizer_r256,
    ///     data_type.secure_simple_pairing_randomizer_r256
    /// );
    ///
    /// let data: Vec<u8> = Vec::new();
    /// let result = SecureSimplePairingRandomizerR256::try_from(&data);
    /// assert!(result.is_err());
    /// assert_eq!(
    ///     format!("Invalid data size :{}", data.len()),
    ///     result.unwrap_err()
    /// );
    /// ```
    fn try_from(value: &Vec<u8>) -> Result<Self, String> {
        let len = value.len();
        if len < 18 {
            return Err(format!("Invalid data size :{}", len).to_string());
        }
        let length = value[0];
        Ok(Self {
            length,
            secure_simple_pairing_randomizer_r256: u128::from_le_bytes(
                value[2..18].try_into().unwrap(),
            ),
        })
    }
}

impl Into<Vec<u8>> for SecureSimplePairingRandomizerR256 {
    /// Create [`Vec<u8>`] from [`SecureSimplePairingRandomizerR256`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::{BASE_UUID, data_types::{secure_simple_pairing_randomizer_r256::SecureSimplePairingRandomizerR256, data_type::DataType}};
    ///
    /// let secure_simple_pairing_randomizer_r256 = 0x0102030405060708090a0b0c0d0e0f10u128;
    /// let result1 = SecureSimplePairingRandomizerR256::new(secure_simple_pairing_randomizer_r256);
    ///
    /// let length = 17;
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(SecureSimplePairingRandomizerR256::data_type());
    /// data.append(&mut secure_simple_pairing_randomizer_r256.to_le_bytes().to_vec());
    ///
    /// let into_data: Vec<u8> = result1.into();
    /// assert_eq!(data, into_data);
    ///
    /// let result2 = SecureSimplePairingRandomizerR256::try_from(&data);
    /// assert!(result2.is_ok());
    /// let data_type = result2.unwrap();
    /// let into_data: Vec<u8> = data_type.into();
    /// assert_eq!(data, into_data);
    /// ```
    fn into(self) -> Vec<u8> {
        let mut data: Vec<u8> = Vec::new();
        data.push(self.length);
        data.push(Self::data_type());
        data.append(
            &mut self
                .secure_simple_pairing_randomizer_r256
                .to_le_bytes()
                .to_vec(),
        );
        return data;
    }
}

impl DataType for SecureSimplePairingRandomizerR256 {
    /// return `0x1e`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{secure_simple_pairing_randomizer_r256::SecureSimplePairingRandomizerR256, data_type::DataType};
    ///
    /// assert_eq!(0x1e, SecureSimplePairingRandomizerR256::data_type());
    /// ```
    fn data_type() -> u8 {
        0x1e
    }
}

/// check `Secure Simple Pairing Randomizer R-256.` data type.
///
/// # Examples
///
/// ```
/// use ble_data_struct::data_types::secure_simple_pairing_randomizer_r256::*;
/// use ble_data_struct::data_types::data_type::DataType;
///
/// assert!(is_secure_simple_pairing_randomizer_r256(0x1e));
/// assert!(!is_secure_simple_pairing_randomizer_r256(0x00));
/// ```
pub fn is_secure_simple_pairing_randomizer_r256(data_type: u8) -> bool {
    SecureSimplePairingRandomizerR256::data_type() == data_type
}

#[cfg(test)]
mod tests {
    use crate::data_types::{data_type::DataType, secure_simple_pairing_randomizer_r256::*};

    #[test]
    fn test_new() {
        let secure_simple_pairing_randomizer_r256 = 0x0102030405060708090a0b0c0d0e0f10u128;
        let result = SecureSimplePairingRandomizerR256::new(secure_simple_pairing_randomizer_r256);
        assert_eq!(17, result.length);
        assert_eq!(
            secure_simple_pairing_randomizer_r256,
            result.secure_simple_pairing_randomizer_r256
        );
    }

    #[test]
    fn test_try_from() {
        let secure_simple_pairing_randomizer_r256 = 0x0102030405060708090a0b0c0d0e0f10u128;
        let length = 17;
        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(SecureSimplePairingRandomizerR256::data_type());
        data.append(&mut secure_simple_pairing_randomizer_r256.to_le_bytes().to_vec());

        let result = SecureSimplePairingRandomizerR256::try_from(&data);
        assert!(result.is_ok());
        let data_type = result.unwrap();
        assert_eq!(length, data_type.length);
        assert_eq!(
            secure_simple_pairing_randomizer_r256,
            data_type.secure_simple_pairing_randomizer_r256
        );

        let mut data: Vec<u8> = vec![0u8; 17];
        data[0] = data.len() as u8 - 1;
        let result = SecureSimplePairingRandomizerR256::try_from(&data);
        assert!(result.is_err());
        assert_eq!(
            format!("Invalid data size :{}", data.len()),
            result.unwrap_err()
        );
    }

    #[test]
    fn test_into() {
        let secure_simple_pairing_randomizer_r256 = 0x0102030405060708090a0b0c0d0e0f10u128;
        let result1 = SecureSimplePairingRandomizerR256::new(secure_simple_pairing_randomizer_r256);

        let length = 17;
        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(SecureSimplePairingRandomizerR256::data_type());
        data.append(&mut secure_simple_pairing_randomizer_r256.to_le_bytes().to_vec());

        let into_data: Vec<u8> = result1.into();
        assert_eq!(data, into_data);

        let result2 = SecureSimplePairingRandomizerR256::try_from(&data);
        assert!(result2.is_ok());
        let data_type = result2.unwrap();
        let into_data: Vec<u8> = data_type.into();
        assert_eq!(data, into_data);
    }

    #[test]
    fn test_data_type() {
        assert_eq!(0x1e, SecureSimplePairingRandomizerR256::data_type());
    }

    #[test]
    fn test_is_secure_simple_pairing_randomizer_r256() {
        assert!(is_secure_simple_pairing_randomizer_r256(0x1e));
        assert!(!is_secure_simple_pairing_randomizer_r256(0x00));
    }
}

//! LE Role (Data Type Value: 0x1c) module.

use crate::data_types::data_type::DataType;

/// LE Role.
#[derive(Debug)]
pub struct LeRole {
    /// data length
    pub length: u8,

    /// LE Role
    pub le_role: u8,
}

impl LeRole {
    /// Create [LeRole] from `LE Role`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::le_role::*;
    ///
    /// let le_role = ONLY_PERIPHERAL_ROLE_SUPPORTED;
    /// let result = LeRole::new(le_role);
    /// assert_eq!(2, result.length);
    /// assert_eq!(le_role, result.le_role);
    ///
    /// let le_role = ONLY_CENTRAL_ROLE_SUPPORTED;
    /// let result = LeRole::new(le_role);
    /// assert_eq!(2, result.length);
    /// assert_eq!(le_role, result.le_role);
    ///
    /// let le_role = PERIPHERAL_ROLE_PREFERRED_FOR_CONNECTION_STABLISHMENT;
    /// let result = LeRole::new(le_role);
    /// assert_eq!(2, result.length);
    /// assert_eq!(le_role, result.le_role);
    ///
    /// let le_role = CENTRAL_ROLE_PREFERRED_FOR_CONNECTION_STABLISHMENT;
    /// let result = LeRole::new(le_role);
    /// assert_eq!(2, result.length);
    /// assert_eq!(le_role, result.le_role);
    /// ```
    pub fn new(le_role: u8) -> Self {
        Self { length: 2, le_role }
    }

    /// check Only Peripheral Role supported.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::le_role::*;
    ///
    /// let le_role = ONLY_PERIPHERAL_ROLE_SUPPORTED;
    /// let result = LeRole::new(le_role);
    /// assert!(result.is_only_peripheral_role_supported());
    ///
    /// let le_role = ONLY_CENTRAL_ROLE_SUPPORTED;
    /// let result = LeRole::new(le_role);
    /// assert!(!result.is_only_peripheral_role_supported());
    ///
    /// let le_role = PERIPHERAL_ROLE_PREFERRED_FOR_CONNECTION_STABLISHMENT;
    /// let result = LeRole::new(le_role);
    /// assert!(!result.is_only_peripheral_role_supported());
    ///
    /// let le_role = CENTRAL_ROLE_PREFERRED_FOR_CONNECTION_STABLISHMENT;
    /// let result = LeRole::new(le_role);
    /// assert!(!result.is_only_peripheral_role_supported());
    /// ```
    pub const fn is_only_peripheral_role_supported(&self) -> bool {
        self.le_role == ONLY_PERIPHERAL_ROLE_SUPPORTED
    }

    /// check Only Central Role supported.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::le_role::*;
    ///
    /// let le_role = ONLY_PERIPHERAL_ROLE_SUPPORTED;
    /// let result = LeRole::new(le_role);
    /// assert!(result.is_only_peripheral_role_supported());
    ///
    /// let le_role = ONLY_CENTRAL_ROLE_SUPPORTED;
    /// let result = LeRole::new(le_role);
    /// assert!(!result.is_only_peripheral_role_supported());
    ///
    /// let le_role = PERIPHERAL_ROLE_PREFERRED_FOR_CONNECTION_STABLISHMENT;
    /// let result = LeRole::new(le_role);
    /// assert!(!result.is_only_peripheral_role_supported());
    ///
    /// let le_role = CENTRAL_ROLE_PREFERRED_FOR_CONNECTION_STABLISHMENT;
    /// let result = LeRole::new(le_role);
    /// assert!(!result.is_only_peripheral_role_supported());
    /// ```
    pub const fn is_only_central_role_supported(&self) -> bool {
        self.le_role == ONLY_CENTRAL_ROLE_SUPPORTED
    }

    /// check Peripheral and Central Role supported, Peripheral Role preferred for connection establishment.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::le_role::*;
    ///
    /// let le_role = ONLY_PERIPHERAL_ROLE_SUPPORTED;
    /// let result = LeRole::new(le_role);
    /// assert!(!result.is_peripheral_role_preferred_for_connection_establishment());
    ///
    /// let le_role = ONLY_CENTRAL_ROLE_SUPPORTED;
    /// let result = LeRole::new(le_role);
    /// assert!(!result.is_peripheral_role_preferred_for_connection_establishment());
    ///
    /// let le_role = PERIPHERAL_ROLE_PREFERRED_FOR_CONNECTION_STABLISHMENT;
    /// let result = LeRole::new(le_role);
    /// assert!(result.is_peripheral_role_preferred_for_connection_establishment());
    ///
    /// let le_role = CENTRAL_ROLE_PREFERRED_FOR_CONNECTION_STABLISHMENT;
    /// let result = LeRole::new(le_role);
    /// assert!(!result.is_peripheral_role_preferred_for_connection_establishment());
    /// ```
    pub const fn is_peripheral_role_preferred_for_connection_establishment(&self) -> bool {
        self.le_role == PERIPHERAL_ROLE_PREFERRED_FOR_CONNECTION_STABLISHMENT
    }

    /// check Peripheral and Central Role supported, Central Role preferred for connection establishment.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::le_role::*;
    ///
    /// let le_role = ONLY_PERIPHERAL_ROLE_SUPPORTED;
    /// let result = LeRole::new(le_role);
    /// assert!(!result.is_central_role_preferred_for_connection_establishment());
    ///
    /// let le_role = ONLY_CENTRAL_ROLE_SUPPORTED;
    /// let result = LeRole::new(le_role);
    /// assert!(!result.is_central_role_preferred_for_connection_establishment());
    ///
    /// let le_role = PERIPHERAL_ROLE_PREFERRED_FOR_CONNECTION_STABLISHMENT;
    /// let result = LeRole::new(le_role);
    /// assert!(!result.is_central_role_preferred_for_connection_establishment());
    ///
    /// let le_role = CENTRAL_ROLE_PREFERRED_FOR_CONNECTION_STABLISHMENT;
    /// let result = LeRole::new(le_role);
    /// assert!(result.is_central_role_preferred_for_connection_establishment());
    /// ```
    pub const fn is_central_role_preferred_for_connection_establishment(&self) -> bool {
        self.le_role == CENTRAL_ROLE_PREFERRED_FOR_CONNECTION_STABLISHMENT
    }
}

/// Only Peripheral Role supported
pub const ONLY_PERIPHERAL_ROLE_SUPPORTED: u8 = 0x00;

/// Only Central Role supported
pub const ONLY_CENTRAL_ROLE_SUPPORTED: u8 = 0x01;

/// Peripheral and Central Role supported, Peripheral Role preferred for connection establishment
pub const PERIPHERAL_ROLE_PREFERRED_FOR_CONNECTION_STABLISHMENT: u8 = 0x02;

/// Peripheral and Central Role supported, Central Role preferred for connection establishment
pub const CENTRAL_ROLE_PREFERRED_FOR_CONNECTION_STABLISHMENT: u8 = 0x03;

impl TryFrom<&Vec<u8>> for LeRole {
    type Error = String;
    /// Create [LE Role] from `Vec<u8>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{le_role::*, data_type::DataType};
    ///
    /// let le_role = ONLY_PERIPHERAL_ROLE_SUPPORTED;
    /// let length = 2;
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(LeRole::data_type());
    /// data.push(le_role);
    ///
    /// let result = LeRole::try_from(&data);
    /// assert!(result.is_ok());
    /// let data_type = result.unwrap();
    /// assert_eq!(length, data_type.length);
    /// assert_eq!(le_role, data_type.le_role);
    ///
    /// let le_role = ONLY_CENTRAL_ROLE_SUPPORTED;
    /// let length = 2;
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(LeRole::data_type());
    /// data.push(le_role);
    ///
    /// let result = LeRole::try_from(&data);
    /// assert!(result.is_ok());
    /// let data_type = result.unwrap();
    /// assert_eq!(length, data_type.length);
    /// assert_eq!(le_role, data_type.le_role);
    ///
    /// let le_role = PERIPHERAL_ROLE_PREFERRED_FOR_CONNECTION_STABLISHMENT;
    /// let length = 2;
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(LeRole::data_type());
    /// data.push(le_role);
    ///
    /// let result = LeRole::try_from(&data);
    /// assert!(result.is_ok());
    /// let data_type = result.unwrap();
    /// assert_eq!(length, data_type.length);
    /// assert_eq!(le_role, data_type.le_role);
    ///
    /// let le_role = CENTRAL_ROLE_PREFERRED_FOR_CONNECTION_STABLISHMENT;
    /// let length = 2;
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(LeRole::data_type());
    /// data.push(le_role);
    ///
    /// let result = LeRole::try_from(&data);
    /// assert!(result.is_ok());
    /// let data_type = result.unwrap();
    /// assert_eq!(length, data_type.length);
    /// assert_eq!(le_role, data_type.le_role);
    ///
    /// let data: Vec<u8> = Vec::new();
    /// let result = LeRole::try_from(&data);
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
            le_role: value[2],
        })
    }
}

impl Into<Vec<u8>> for LeRole {
    /// Create `Vec<u8>` from [LeRole].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{le_role::*, data_type::DataType};
    ///
    /// let le_role = ONLY_PERIPHERAL_ROLE_SUPPORTED;
    /// let length = 2;
    /// let result1 = LeRole::new(le_role);
    ///
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(LeRole::data_type());
    /// data.push(le_role);
    ///
    /// let into_data: Vec<u8> = result1.into();
    /// assert_eq!(data, into_data);
    ///
    /// let result2 = LeRole::try_from(&data);
    /// assert!(result2.is_ok());
    /// let data_type = result2.unwrap();
    /// let into_data: Vec<u8> = data_type.into();
    /// assert_eq!(data, into_data);
    ///
    /// let le_role = ONLY_CENTRAL_ROLE_SUPPORTED;
    /// let length = 2;
    /// let result1 = LeRole::new(le_role);
    ///
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(LeRole::data_type());
    /// data.push(le_role);
    ///
    /// let into_data: Vec<u8> = result1.into();
    /// assert_eq!(data, into_data);
    ///
    /// let result2 = LeRole::try_from(&data);
    /// assert!(result2.is_ok());
    /// let data_type = result2.unwrap();
    /// let into_data: Vec<u8> = data_type.into();
    /// assert_eq!(data, into_data);
    ///
    /// let le_role = PERIPHERAL_ROLE_PREFERRED_FOR_CONNECTION_STABLISHMENT;
    /// let length = 2;
    /// let result1 = LeRole::new(le_role);
    ///
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(LeRole::data_type());
    /// data.push(le_role);
    ///
    /// let into_data: Vec<u8> = result1.into();
    /// assert_eq!(data, into_data);
    ///
    /// let result2 = LeRole::try_from(&data);
    /// assert!(result2.is_ok());
    /// let data_type = result2.unwrap();
    /// let into_data: Vec<u8> = data_type.into();
    /// assert_eq!(data, into_data);
    ///
    /// let le_role = CENTRAL_ROLE_PREFERRED_FOR_CONNECTION_STABLISHMENT;
    /// let length = 2;
    /// let result1 = LeRole::new(le_role);
    ///
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(LeRole::data_type());
    /// data.push(le_role);
    ///
    /// let into_data: Vec<u8> = result1.into();
    /// assert_eq!(data, into_data);
    ///
    /// let result2 = LeRole::try_from(&data);
    /// assert!(result2.is_ok());
    /// let data_type = result2.unwrap();
    /// let into_data: Vec<u8> = data_type.into();
    /// assert_eq!(data, into_data);
    /// ```
    fn into(self) -> Vec<u8> {
        let mut data: Vec<u8> = Vec::new();
        data.push(self.length);
        data.push(Self::data_type());
        data.push(self.le_role);
        return data;
    }
}

impl DataType for LeRole {
    /// return `0x1c`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{le_role::LeRole, data_type::DataType};
    ///
    /// assert_eq!(0x1c, LeRole::data_type());
    /// ```
    fn data_type() -> u8 {
        0x1c
    }
}

/// check `LE Role` data type.
///
/// # Examples
///
/// ```
/// use ble_data_struct::data_types::le_role::*;
/// use ble_data_struct::data_types::data_type::DataType;
///
/// assert!(is_le_role(0x1c));
/// assert!(!is_le_role(0x00));
/// ```
pub fn is_le_role(data_type: u8) -> bool {
    LeRole::data_type() == data_type
}

#[cfg(test)]
mod tests {
    use crate::data_types::{data_type::DataType, le_role::*};

    #[test]
    fn test_new() {
        let le_role = ONLY_PERIPHERAL_ROLE_SUPPORTED;
        let result = LeRole::new(le_role);
        assert_eq!(2, result.length);
        assert_eq!(le_role, result.le_role);

        let le_role = ONLY_CENTRAL_ROLE_SUPPORTED;
        let result = LeRole::new(le_role);
        assert_eq!(2, result.length);
        assert_eq!(le_role, result.le_role);

        let le_role = PERIPHERAL_ROLE_PREFERRED_FOR_CONNECTION_STABLISHMENT;
        let result = LeRole::new(le_role);
        assert_eq!(2, result.length);
        assert_eq!(le_role, result.le_role);

        let le_role = CENTRAL_ROLE_PREFERRED_FOR_CONNECTION_STABLISHMENT;
        let result = LeRole::new(le_role);
        assert_eq!(2, result.length);
        assert_eq!(le_role, result.le_role);
    }

    #[test]
    fn test_is_only_peripheral_role_supported() {
        let le_role = ONLY_PERIPHERAL_ROLE_SUPPORTED;
        let result = LeRole::new(le_role);
        assert!(result.is_only_peripheral_role_supported());

        let le_role = ONLY_CENTRAL_ROLE_SUPPORTED;
        let result = LeRole::new(le_role);
        assert!(!result.is_only_peripheral_role_supported());

        let le_role = PERIPHERAL_ROLE_PREFERRED_FOR_CONNECTION_STABLISHMENT;
        let result = LeRole::new(le_role);
        assert!(!result.is_only_peripheral_role_supported());

        let le_role = CENTRAL_ROLE_PREFERRED_FOR_CONNECTION_STABLISHMENT;
        let result = LeRole::new(le_role);
        assert!(!result.is_only_peripheral_role_supported());
    }

    #[test]
    fn test_is_only_central_role_supported() {
        let le_role = ONLY_PERIPHERAL_ROLE_SUPPORTED;
        let result = LeRole::new(le_role);
        assert!(!result.is_only_central_role_supported());

        let le_role = ONLY_CENTRAL_ROLE_SUPPORTED;
        let result = LeRole::new(le_role);
        assert!(result.is_only_central_role_supported());

        let le_role = PERIPHERAL_ROLE_PREFERRED_FOR_CONNECTION_STABLISHMENT;
        let result = LeRole::new(le_role);
        assert!(!result.is_only_central_role_supported());

        let le_role = CENTRAL_ROLE_PREFERRED_FOR_CONNECTION_STABLISHMENT;
        let result = LeRole::new(le_role);
        assert!(!result.is_only_central_role_supported());
    }

    #[test]
    fn test_is_peripheral_role_preferred_for_connection_establishment() {
        let le_role = ONLY_PERIPHERAL_ROLE_SUPPORTED;
        let result = LeRole::new(le_role);
        assert!(!result.is_peripheral_role_preferred_for_connection_establishment());

        let le_role = ONLY_CENTRAL_ROLE_SUPPORTED;
        let result = LeRole::new(le_role);
        assert!(!result.is_peripheral_role_preferred_for_connection_establishment());

        let le_role = PERIPHERAL_ROLE_PREFERRED_FOR_CONNECTION_STABLISHMENT;
        let result = LeRole::new(le_role);
        assert!(result.is_peripheral_role_preferred_for_connection_establishment());

        let le_role = CENTRAL_ROLE_PREFERRED_FOR_CONNECTION_STABLISHMENT;
        let result = LeRole::new(le_role);
        assert!(!result.is_peripheral_role_preferred_for_connection_establishment());
    }

    #[test]
    fn test_is_central_role_preferred_for_connection_establishment() {
        let le_role = ONLY_PERIPHERAL_ROLE_SUPPORTED;
        let result = LeRole::new(le_role);
        assert!(!result.is_central_role_preferred_for_connection_establishment());

        let le_role = ONLY_CENTRAL_ROLE_SUPPORTED;
        let result = LeRole::new(le_role);
        assert!(!result.is_central_role_preferred_for_connection_establishment());

        let le_role = PERIPHERAL_ROLE_PREFERRED_FOR_CONNECTION_STABLISHMENT;
        let result = LeRole::new(le_role);
        assert!(!result.is_central_role_preferred_for_connection_establishment());

        let le_role = CENTRAL_ROLE_PREFERRED_FOR_CONNECTION_STABLISHMENT;
        let result = LeRole::new(le_role);
        assert!(result.is_central_role_preferred_for_connection_establishment());
    }

    #[test]
    fn test_try_from() {
        let le_role = ONLY_PERIPHERAL_ROLE_SUPPORTED;
        let length = 2;
        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(LeRole::data_type());
        data.push(le_role);

        let result = LeRole::try_from(&data);
        assert!(result.is_ok());
        let data_type = result.unwrap();
        assert_eq!(length, data_type.length);
        assert_eq!(le_role, data_type.le_role);

        let le_role = ONLY_CENTRAL_ROLE_SUPPORTED;
        let length = 2;
        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(LeRole::data_type());
        data.push(le_role);

        let result = LeRole::try_from(&data);
        assert!(result.is_ok());
        let data_type = result.unwrap();
        assert_eq!(length, data_type.length);
        assert_eq!(le_role, data_type.le_role);

        let le_role = PERIPHERAL_ROLE_PREFERRED_FOR_CONNECTION_STABLISHMENT;
        let length = 2;
        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(LeRole::data_type());
        data.push(le_role);

        let result = LeRole::try_from(&data);
        assert!(result.is_ok());
        let data_type = result.unwrap();
        assert_eq!(length, data_type.length);
        assert_eq!(le_role, data_type.le_role);

        let le_role = CENTRAL_ROLE_PREFERRED_FOR_CONNECTION_STABLISHMENT;
        let length = 2;
        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(LeRole::data_type());
        data.push(le_role);

        let result = LeRole::try_from(&data);
        assert!(result.is_ok());
        let data_type = result.unwrap();
        assert_eq!(length, data_type.length);
        assert_eq!(le_role, data_type.le_role);

        let mut data: Vec<u8> = vec![0u8; 2];
        data[0] = data.len() as u8 - 1;
        let result = LeRole::try_from(&data);
        assert!(result.is_err());
        assert_eq!(
            format!("Invalid data size :{}", data.len()),
            result.unwrap_err()
        );
    }

    #[test]
    fn test_into() {
        let le_role = ONLY_PERIPHERAL_ROLE_SUPPORTED;
        let length = 2;
        let result1 = LeRole::new(le_role);

        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(LeRole::data_type());
        data.push(le_role);

        let into_data: Vec<u8> = result1.into();
        assert_eq!(data, into_data);

        let result2 = LeRole::try_from(&data);
        assert!(result2.is_ok());
        let data_type = result2.unwrap();
        let into_data: Vec<u8> = data_type.into();
        assert_eq!(data, into_data);

        let le_role = ONLY_CENTRAL_ROLE_SUPPORTED;
        let length = 2;
        let result1 = LeRole::new(le_role);

        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(LeRole::data_type());
        data.push(le_role);

        let into_data: Vec<u8> = result1.into();
        assert_eq!(data, into_data);

        let result2 = LeRole::try_from(&data);
        assert!(result2.is_ok());
        let data_type = result2.unwrap();
        let into_data: Vec<u8> = data_type.into();
        assert_eq!(data, into_data);

        let le_role = PERIPHERAL_ROLE_PREFERRED_FOR_CONNECTION_STABLISHMENT;
        let length = 2;
        let result1 = LeRole::new(le_role);

        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(LeRole::data_type());
        data.push(le_role);

        let into_data: Vec<u8> = result1.into();
        assert_eq!(data, into_data);

        let result2 = LeRole::try_from(&data);
        assert!(result2.is_ok());
        let data_type = result2.unwrap();
        let into_data: Vec<u8> = data_type.into();
        assert_eq!(data, into_data);

        let le_role = CENTRAL_ROLE_PREFERRED_FOR_CONNECTION_STABLISHMENT;
        let length = 2;
        let result1 = LeRole::new(le_role);

        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(LeRole::data_type());
        data.push(le_role);

        let into_data: Vec<u8> = result1.into();
        assert_eq!(data, into_data);

        let result2 = LeRole::try_from(&data);
        assert!(result2.is_ok());
        let data_type = result2.unwrap();
        let into_data: Vec<u8> = data_type.into();
        assert_eq!(data, into_data);
    }

    #[test]
    fn test_data_type() {
        assert_eq!(0x1c, LeRole::data_type());
    }

    #[test]
    fn test_is_le_role() {
        assert!(is_le_role(0x1c));
        assert!(!is_le_role(0x00));
    }
}

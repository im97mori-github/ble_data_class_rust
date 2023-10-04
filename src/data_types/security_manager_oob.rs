//! Security Manager Out of Band (Data Type Value: 0x11) module.

use crate::data_types::data_type::DataType;

/// Security Manager Out of Band.
#[derive(Debug)]
pub struct SecurityManagerOutOfBand {
    /// data length
    pub length: u8,

    /// Security Manager Out of Band Flag
    pub security_manager_oob: [bool; 8],
}

impl SecurityManagerOutOfBand {
    /// Create [`SecurityManagerOutOfBand`] from `Security Manager Out of Band`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::security_manager_oob::*;
    ///
    /// let security_manager_oob = [true, false, false, false, false, false, false, false];
    /// let result = SecurityManagerOutOfBand::new(&security_manager_oob);
    /// assert_eq!(2, result.length);
    /// assert_eq!(security_manager_oob, result.security_manager_oob);
    ///
    /// let security_manager_oob = [false, true, false, false, false, false, false, false];
    /// let result = SecurityManagerOutOfBand::new(&security_manager_oob);
    /// assert_eq!(2, result.length);
    /// assert_eq!(security_manager_oob, result.security_manager_oob);
    ///
    /// let security_manager_oob = [false, false, false, true, false, false, false, false];
    /// let result = SecurityManagerOutOfBand::new(&security_manager_oob);
    /// assert_eq!(2, result.length);
    /// assert_eq!(security_manager_oob, result.security_manager_oob);
    /// ```
    pub fn new(security_manager_oob: &[bool; 8]) -> Self {
        Self {
            length: 2,
            security_manager_oob: security_manager_oob.clone(),
        }
    }

    /// check OOB Flags Field.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::security_manager_oob::SecurityManagerOutOfBand;
    ///
    /// let security_manager_oob = [true, false, false, false, false, false, false, false];
    /// let result = SecurityManagerOutOfBand::new(&security_manager_oob);
    /// assert!(result.is_oob_flags_field());
    ///
    /// let security_manager_oob = [false, true, false, false, false, false, false, false];
    /// let result = SecurityManagerOutOfBand::new(&security_manager_oob);
    /// assert!(!result.is_oob_flags_field());
    ///
    /// let security_manager_oob = [false, false, false, true, false, false, false, false];
    /// let result = SecurityManagerOutOfBand::new(&security_manager_oob);
    /// assert!(!result.is_oob_flags_field());
    /// ```
    pub fn is_oob_flags_field(&self) -> bool {
        self.security_manager_oob[0]
    }

    /// check LE supported (Host).
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::security_manager_oob::SecurityManagerOutOfBand;
    ///
    /// let security_manager_oob = [true, false, false, false, false, false, false, false];
    /// let result = SecurityManagerOutOfBand::new(&security_manager_oob);
    /// assert!(!result.is_le_supported());
    ///
    /// let security_manager_oob = [false, true, false, false, false, false, false, false];
    /// let result = SecurityManagerOutOfBand::new(&security_manager_oob);
    /// assert!(result.is_le_supported());
    ///
    /// let security_manager_oob = [false, false, false, true, false, false, false, false];
    /// let result = SecurityManagerOutOfBand::new(&security_manager_oob);
    /// assert!(!result.is_le_supported());
    /// ```
    pub fn is_le_supported(&self) -> bool {
        self.security_manager_oob[1]
    }

    /// check Address type.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::security_manager_oob::SecurityManagerOutOfBand;
    ///
    /// let security_manager_oob = [true, false, false, false, false, false, false, false];
    /// let result = SecurityManagerOutOfBand::new(&security_manager_oob);
    /// assert!(!result.is_le_supported());
    ///
    /// let security_manager_oob = [false, true, false, false, false, false, false, false];
    /// let result = SecurityManagerOutOfBand::new(&security_manager_oob);
    /// assert!(result.is_le_supported());
    ///
    /// let security_manager_oob = [false, false, false, true, false, false, false, false];
    /// let result = SecurityManagerOutOfBand::new(&security_manager_oob);
    /// assert!(!result.is_le_supported());
    /// ```
    pub fn is_random_address(&self) -> bool {
        self.security_manager_oob[3]
    }
}

/// OOB Flags Field
/// (0 = OOB data not present, 1 = OOB data present)
pub const SECURITY_MANAGER_OUT_OF_BAND_FLAG_OOB_FLAGS: u8 = 0b00000001u8;

///  LE supported (Host)
///  (i.e. bit 65 of LMP Extended Feature bits Page 1)
pub const SECURITY_MANAGER_LE_SUPPORTED: u8 = 0b00000010u8;

/// Address type
/// (0 = Public Address, 1 = Random Address)
pub const SECURITY_MANAGER_ADDRESS_TYPE: u8 = 0b00001000u8;

impl TryFrom<&Vec<u8>> for SecurityManagerOutOfBand {
    type Error = String;
    /// Create [`SecurityManagerOutOfBand`] from `Vec<u8>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{security_manager_oob::*, data_type::DataType};
    ///
    /// let security_manager_oob_byte = SECURITY_MANAGER_OUT_OF_BAND_FLAG_OOB_FLAGS;
    /// let mut security_manager_oob = [false; 8];
    /// security_manager_oob[0] = true;
    /// let length = 2;
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(SecurityManagerOutOfBand::data_type());
    /// data.push(security_manager_oob_byte);
    ///
    /// let result = SecurityManagerOutOfBand::try_from(&data);
    /// assert!(result.is_ok());
    /// let data_type = result.unwrap();
    /// assert_eq!(length, data_type.length);
    /// assert_eq!(security_manager_oob, data_type.security_manager_oob);
    ///
    /// let security_manager_oob_byte = SECURITY_MANAGER_LE_SUPPORTED;
    /// let mut security_manager_oob = [false; 8];
    /// security_manager_oob[1] = true;
    /// let length = 2;
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(SecurityManagerOutOfBand::data_type());
    /// data.push(security_manager_oob_byte);
    ///
    /// let result = SecurityManagerOutOfBand::try_from(&data);
    /// assert!(result.is_ok());
    /// let data_type = result.unwrap();
    /// assert_eq!(length, data_type.length);
    /// assert_eq!(security_manager_oob, data_type.security_manager_oob);
    ///
    /// let security_manager_oob_byte = SECURITY_MANAGER_ADDRESS_TYPE;
    /// let mut security_manager_oob = [false; 8];
    /// security_manager_oob[3] = true;
    /// let length = 4;
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(SecurityManagerOutOfBand::data_type());
    /// data.push(security_manager_oob_byte);
    ///
    /// let result = SecurityManagerOutOfBand::try_from(&data);
    /// assert!(result.is_ok());
    /// let data_type = result.unwrap();
    /// assert_eq!(length, data_type.length);
    /// assert_eq!(security_manager_oob, data_type.security_manager_oob);
    ///
    /// let data: Vec<u8> = Vec::new();
    /// let result = SecurityManagerOutOfBand::try_from(&data);
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
        let mut security_manager_oob = [false; 8];
        security_manager_oob[0] = value[2] & SECURITY_MANAGER_OUT_OF_BAND_FLAG_OOB_FLAGS != 0;
        security_manager_oob[1] = value[2] & SECURITY_MANAGER_LE_SUPPORTED != 0;
        security_manager_oob[3] = value[2] & SECURITY_MANAGER_ADDRESS_TYPE != 0;
        Ok(Self {
            length,
            security_manager_oob,
        })
    }
}

impl Into<Vec<u8>> for SecurityManagerOutOfBand {
    /// Create `Vec<u8>` from [`SecurityManagerOutOfBand`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{security_manager_oob::*, data_type::DataType};
    ///
    /// let security_manager_oob = [true, false, false, false, false, false, false, false];
    /// let length = 2;
    /// let result1 = SecurityManagerOutOfBand::new(&security_manager_oob);
    ///
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(SecurityManagerOutOfBand::data_type());
    /// let mut security_manager_oob_byte = 0u8;
    /// if security_manager_oob[0] {
    ///     security_manager_oob_byte |= SECURITY_MANAGER_OUT_OF_BAND_FLAG_OOB_FLAGS;
    /// }
    /// if security_manager_oob[1] {
    ///     security_manager_oob_byte |= SECURITY_MANAGER_LE_SUPPORTED;
    /// }
    /// if security_manager_oob[3] {
    ///     security_manager_oob_byte |= SECURITY_MANAGER_ADDRESS_TYPE;
    /// }
    /// data.push(security_manager_oob_byte);
    ///
    /// let into_data: Vec<u8> = result1.into();
    /// assert_eq!(data, into_data);
    ///
    /// let result2 = SecurityManagerOutOfBand::try_from(&data);
    /// assert!(result2.is_ok());
    /// let data_type = result2.unwrap();
    /// let into_data: Vec<u8> = data_type.into();
    /// assert_eq!(data, into_data);
    ///
    /// let security_manager_oob = [false, true, false, false, false, false, false, false];
    /// let length = 2;
    /// let result1 = SecurityManagerOutOfBand::new(&security_manager_oob);
    ///
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(SecurityManagerOutOfBand::data_type());
    /// let mut security_manager_oob_byte = 0u8;
    /// if security_manager_oob[0] {
    ///     security_manager_oob_byte |= SECURITY_MANAGER_OUT_OF_BAND_FLAG_OOB_FLAGS;
    /// }
    /// if security_manager_oob[1] {
    ///     security_manager_oob_byte |= SECURITY_MANAGER_LE_SUPPORTED;
    /// }
    /// if security_manager_oob[3] {
    ///     security_manager_oob_byte |= SECURITY_MANAGER_ADDRESS_TYPE;
    /// }
    /// data.push(security_manager_oob_byte);
    ///
    /// let into_data: Vec<u8> = result1.into();
    /// assert_eq!(data, into_data);
    ///
    /// let result2 = SecurityManagerOutOfBand::try_from(&data);
    /// assert!(result2.is_ok());
    /// let data_type = result2.unwrap();
    /// let into_data: Vec<u8> = data_type.into();
    /// assert_eq!(data, into_data);
    ///
    /// let security_manager_oob = [false, false, false, true, false, false, false, false];
    /// let length = 2;
    /// let result1 = SecurityManagerOutOfBand::new(&security_manager_oob);
    ///
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(SecurityManagerOutOfBand::data_type());
    /// let mut security_manager_oob_byte = 0u8;
    /// if security_manager_oob[0] {
    ///     security_manager_oob_byte |= SECURITY_MANAGER_OUT_OF_BAND_FLAG_OOB_FLAGS;
    /// }
    /// if security_manager_oob[1] {
    ///     security_manager_oob_byte |= SECURITY_MANAGER_LE_SUPPORTED;
    /// }
    /// if security_manager_oob[3] {
    ///     security_manager_oob_byte |= SECURITY_MANAGER_ADDRESS_TYPE;
    /// }
    /// data.push(security_manager_oob_byte);
    ///
    /// let into_data: Vec<u8> = result1.into();
    /// assert_eq!(data, into_data);
    ///
    /// let result2 = SecurityManagerOutOfBand::try_from(&data);
    /// assert!(result2.is_ok());
    /// let data_type = result2.unwrap();
    /// let into_data: Vec<u8> = data_type.into();
    /// assert_eq!(data, into_data);
    /// ```
    fn into(self) -> Vec<u8> {
        let mut data: Vec<u8> = Vec::new();
        data.push(self.length);
        data.push(Self::data_type());

        let mut security_manager_oob = 0u8;
        if self.security_manager_oob[0] {
            security_manager_oob |= SECURITY_MANAGER_OUT_OF_BAND_FLAG_OOB_FLAGS;
        }
        if self.security_manager_oob[1] {
            security_manager_oob |= SECURITY_MANAGER_LE_SUPPORTED;
        }
        if self.security_manager_oob[3] {
            security_manager_oob |= SECURITY_MANAGER_ADDRESS_TYPE;
        }
        data.push(security_manager_oob);
        return data;
    }
}

impl DataType for SecurityManagerOutOfBand {
    /// return `0x11`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{security_manager_oob::SecurityManagerOutOfBand, data_type::DataType};
    ///
    /// assert_eq!(0x11, SecurityManagerOutOfBand::data_type());
    /// ```
    fn data_type() -> u8 {
        0x11
    }
}

/// check `Security Manager Out of Band` data type.
///
/// # Examples
///
/// ```
/// use ble_data_struct::data_types::security_manager_oob::*;
/// use ble_data_struct::data_types::data_type::DataType;
///
/// assert!(is_security_manager_oob(0x11));
/// assert!(!is_security_manager_oob(0x00));
/// ```
pub fn is_security_manager_oob(data_type: u8) -> bool {
    SecurityManagerOutOfBand::data_type() == data_type
}

#[cfg(test)]
mod tests {
    use crate::data_types::{data_type::DataType, security_manager_oob::*};

    #[test]
    fn test_new() {
        let security_manager_oob = [true, false, false, false, false, false, false, false];
        let result = SecurityManagerOutOfBand::new(&security_manager_oob);
        assert_eq!(2, result.length);
        assert_eq!(security_manager_oob, result.security_manager_oob);

        let security_manager_oob = [false, true, false, false, false, false, false, false];
        let result = SecurityManagerOutOfBand::new(&security_manager_oob);
        assert_eq!(2, result.length);
        assert_eq!(security_manager_oob, result.security_manager_oob);

        let security_manager_oob = [false, false, false, true, false, false, false, false];
        let result = SecurityManagerOutOfBand::new(&security_manager_oob);
        assert_eq!(2, result.length);
        assert_eq!(security_manager_oob, result.security_manager_oob);
    }

    #[test]
    fn test_is_oob_flags_field() {
        let security_manager_oob = [true, false, false, false, false, false, false, false];
        let result = SecurityManagerOutOfBand::new(&security_manager_oob);
        assert!(result.is_oob_flags_field());

        let security_manager_oob = [false, true, false, false, false, false, false, false];
        let result = SecurityManagerOutOfBand::new(&security_manager_oob);
        assert!(!result.is_oob_flags_field());

        let security_manager_oob = [false, false, false, true, false, false, false, false];
        let result = SecurityManagerOutOfBand::new(&security_manager_oob);
        assert!(!result.is_oob_flags_field());
    }

    #[test]
    fn test_is_le_supported() {
        let security_manager_oob = [true, false, false, false, false, false, false, false];
        let result = SecurityManagerOutOfBand::new(&security_manager_oob);
        assert!(!result.is_le_supported());

        let security_manager_oob = [false, true, false, false, false, false, false, false];
        let result = SecurityManagerOutOfBand::new(&security_manager_oob);
        assert!(result.is_le_supported());

        let security_manager_oob = [false, false, false, true, false, false, false, false];
        let result = SecurityManagerOutOfBand::new(&security_manager_oob);
        assert!(!result.is_le_supported());
    }

    #[test]
    fn test_is_random_address() {
        let security_manager_oob = [true, false, false, false, false, false, false, false];
        let result = SecurityManagerOutOfBand::new(&security_manager_oob);
        assert!(!result.is_random_address());

        let security_manager_oob = [false, true, false, false, false, false, false, false];
        let result = SecurityManagerOutOfBand::new(&security_manager_oob);
        assert!(!result.is_random_address());

        let security_manager_oob = [false, false, false, true, false, false, false, false];
        let result = SecurityManagerOutOfBand::new(&security_manager_oob);
        assert!(result.is_random_address());
    }

    #[test]
    fn test_try_from() {
        let security_manager_oob_byte = SECURITY_MANAGER_OUT_OF_BAND_FLAG_OOB_FLAGS;
        let mut security_manager_oob = [false; 8];
        security_manager_oob[0] = true;
        let length = 2;
        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(SecurityManagerOutOfBand::data_type());
        data.push(security_manager_oob_byte);

        let result = SecurityManagerOutOfBand::try_from(&data);
        assert!(result.is_ok());
        let data_type = result.unwrap();
        assert_eq!(length, data_type.length);
        assert_eq!(security_manager_oob, data_type.security_manager_oob);

        let security_manager_oob_byte = SECURITY_MANAGER_LE_SUPPORTED;
        let mut security_manager_oob = [false; 8];
        security_manager_oob[1] = true;
        let length = 2;
        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(SecurityManagerOutOfBand::data_type());
        data.push(security_manager_oob_byte);

        let result = SecurityManagerOutOfBand::try_from(&data);
        assert!(result.is_ok());
        let data_type = result.unwrap();
        assert_eq!(length, data_type.length);
        assert_eq!(security_manager_oob, data_type.security_manager_oob);

        let security_manager_oob_byte = SECURITY_MANAGER_ADDRESS_TYPE;
        let mut security_manager_oob = [false; 8];
        security_manager_oob[3] = true;
        let length = 4;
        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(SecurityManagerOutOfBand::data_type());
        data.push(security_manager_oob_byte);

        let result = SecurityManagerOutOfBand::try_from(&data);
        assert!(result.is_ok());
        let data_type = result.unwrap();
        assert_eq!(length, data_type.length);
        assert_eq!(security_manager_oob, data_type.security_manager_oob);

        let mut data: Vec<u8> = vec![0u8; 2];
        data[0] = data.len() as u8 - 1;
        let result = SecurityManagerOutOfBand::try_from(&data);
        assert!(result.is_err());
        assert_eq!(
            format!("Invalid data size :{}", data.len()),
            result.unwrap_err()
        );
    }

    #[test]
    fn test_into() {
        let security_manager_oob = [true, false, false, false, false, false, false, false];
        let length = 2;
        let result1 = SecurityManagerOutOfBand::new(&security_manager_oob);

        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(SecurityManagerOutOfBand::data_type());
        let mut security_manager_oob_byte = 0u8;
        if security_manager_oob[0] {
            security_manager_oob_byte |= SECURITY_MANAGER_OUT_OF_BAND_FLAG_OOB_FLAGS;
        }
        if security_manager_oob[1] {
            security_manager_oob_byte |= SECURITY_MANAGER_LE_SUPPORTED;
        }
        if security_manager_oob[3] {
            security_manager_oob_byte |= SECURITY_MANAGER_ADDRESS_TYPE;
        }
        data.push(security_manager_oob_byte);

        let into_data: Vec<u8> = result1.into();
        assert_eq!(data, into_data);

        let result2 = SecurityManagerOutOfBand::try_from(&data);
        assert!(result2.is_ok());
        let data_type = result2.unwrap();
        let into_data: Vec<u8> = data_type.into();
        assert_eq!(data, into_data);

        let security_manager_oob = [false, true, false, false, false, false, false, false];
        let length = 2;
        let result1 = SecurityManagerOutOfBand::new(&security_manager_oob);

        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(SecurityManagerOutOfBand::data_type());
        let mut security_manager_oob_byte = 0u8;
        if security_manager_oob[0] {
            security_manager_oob_byte |= SECURITY_MANAGER_OUT_OF_BAND_FLAG_OOB_FLAGS;
        }
        if security_manager_oob[1] {
            security_manager_oob_byte |= SECURITY_MANAGER_LE_SUPPORTED;
        }
        if security_manager_oob[3] {
            security_manager_oob_byte |= SECURITY_MANAGER_ADDRESS_TYPE;
        }
        data.push(security_manager_oob_byte);

        let into_data: Vec<u8> = result1.into();
        assert_eq!(data, into_data);

        let result2 = SecurityManagerOutOfBand::try_from(&data);
        assert!(result2.is_ok());
        let data_type = result2.unwrap();
        let into_data: Vec<u8> = data_type.into();
        assert_eq!(data, into_data);

        let security_manager_oob = [false, false, false, true, false, false, false, false];
        let length = 2;
        let result1 = SecurityManagerOutOfBand::new(&security_manager_oob);

        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(SecurityManagerOutOfBand::data_type());
        let mut security_manager_oob_byte = 0u8;
        if security_manager_oob[0] {
            security_manager_oob_byte |= SECURITY_MANAGER_OUT_OF_BAND_FLAG_OOB_FLAGS;
        }
        if security_manager_oob[1] {
            security_manager_oob_byte |= SECURITY_MANAGER_LE_SUPPORTED;
        }
        if security_manager_oob[3] {
            security_manager_oob_byte |= SECURITY_MANAGER_ADDRESS_TYPE;
        }
        data.push(security_manager_oob_byte);

        let into_data: Vec<u8> = result1.into();
        assert_eq!(data, into_data);

        let result2 = SecurityManagerOutOfBand::try_from(&data);
        assert!(result2.is_ok());
        let data_type = result2.unwrap();
        let into_data: Vec<u8> = data_type.into();
        assert_eq!(data, into_data);
    }

    #[test]
    fn test_data_type() {
        assert_eq!(0x11, SecurityManagerOutOfBand::data_type());
    }

    #[test]
    fn test_is_security_manager_oob() {
        assert!(is_security_manager_oob(0x11));
        assert!(!is_security_manager_oob(0x00));
    }
}

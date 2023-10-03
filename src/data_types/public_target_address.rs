//! Public Target Address (Data Type Value:0x17) module.

use crate::data_types::data_type::DataType;

/// Public Target Address.

#[derive(Debug)]
pub struct PublicTargetAddress {
    /// data length
    pub length: u8,

    pub public_target_address: Vec<u64>,
}

impl PublicTargetAddress {
    /// Create [`PublicTargetAddress`] from `Public Target Address`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::public_target_address::PublicTargetAddress;
    ///
    /// let public_target_address: Vec<u64> = [
    ///     u64::from_le_bytes([
    ///         0x01u8, 0x02u8, 0x03u8, 0x04u8, 0x05u8, 0x06u8, 0x00u8, 0x00u8,
    ///     ]),
    ///     u64::from_le_bytes([
    ///         0x07u8, 0x08u8, 0x09u8, 0x0au8, 0x0bu8, 0x0cu8, 0x00u8, 0x00u8,
    ///     ]),
    /// ]
    /// .to_vec();
    /// let result = PublicTargetAddress::new(&public_target_address);
    /// assert_eq!(public_target_address.len() as u8 * 6 + 1, result.length);
    /// assert_eq!(public_target_address, result.public_target_address);
    /// ```
    pub fn new(public_target_address: &Vec<u64>) -> Self {
        Self {
            length: public_target_address.len() as u8 * 6 + 1,
            public_target_address: public_target_address.clone(),
        }
    }
}

impl TryFrom<&Vec<u8>> for PublicTargetAddress {
    type Error = String;
    /// Create [`PublicTargetAddress`] from `Vec<u8>`.
    ///
    /// [`PublicTargetAddress::from_with_offset`]
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{public_target_address::PublicTargetAddress, data_type::DataType};
    ///
    /// let public_target_address_bytes = [
    ///     0x01u8, 0x02u8, 0x03u8, 0x04u8, 0x05u8, 0x06u8, 0x07u8, 0x08u8, 0x09u8, 0x0au8, 0x0bu8,
    ///     0x0cu8,
    /// ]
    /// .to_vec();
    /// let public_target_address: Vec<u64> = public_target_address_bytes
    ///     .windows(6)
    ///     .step_by(6)
    ///     .map(|f| {
    ///         let mut bytes = [0x00u8; 8];
    ///         bytes[0] = f[0];
    ///         bytes[1] = f[1];
    ///         bytes[2] = f[2];
    ///         bytes[3] = f[3];
    ///         bytes[4] = f[4];
    ///         bytes[5] = f[5];
    ///         u64::from_le_bytes(bytes)
    ///     })
    ///     .collect();
    /// let length = public_target_address_bytes.len() as u8 + 1;
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(PublicTargetAddress::data_type());
    /// data.append(&mut public_target_address_bytes.clone());
    ///
    /// let result = PublicTargetAddress::try_from(&data);
    /// assert!(result.is_ok());
    /// let data_type = result.unwrap();
    /// assert_eq!(length, data_type.length);
    /// assert_eq!(public_target_address, data_type.public_target_address);
    ///
    /// let data: Vec<u8> = Vec::new();
    /// let result = PublicTargetAddress::try_from(&data);
    /// assert!(result.is_err());
    /// assert_eq!(
    ///     format!("Invalid data size :{}", data.len()),
    ///     result.unwrap_err()
    /// );
    /// ```
    fn try_from(value: &Vec<u8>) -> Result<Self, String> {
        let len = value.len();
        if len < 8 {
            return Err(format!("Invalid data size :{}", len).to_string());
        }
        let length = value[0];
        Ok(Self {
            length,
            public_target_address: value[2..2 + length as usize - 1]
                .windows(6)
                .step_by(6)
                .map(|w| {
                    let mut bytes = [0x00u8; 8];
                    bytes[0] = w[0];
                    bytes[1] = w[1];
                    bytes[2] = w[2];
                    bytes[3] = w[3];
                    bytes[4] = w[4];
                    bytes[5] = w[5];
                    u64::from_le_bytes(bytes)
                })
                .collect(),
        })
    }
}

impl Into<Vec<u8>> for PublicTargetAddress {
    /// Create `Vec<u8>` from [`PublicTargetAddress`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{public_target_address::PublicTargetAddress, data_type::DataType};
    ///
    /// let public_target_address_bytes = [
    ///     0x01u8, 0x02u8, 0x03u8, 0x04u8, 0x05u8, 0x06u8, 0x07u8, 0x08u8, 0x09u8, 0x0au8, 0x0bu8,
    ///     0x0cu8,
    /// ]
    /// .to_vec();
    /// let public_target_address: Vec<u64> = public_target_address_bytes
    ///     .windows(6)
    ///     .step_by(6)
    ///     .map(|f| {
    ///         let mut bytes = [0x00u8; 8];
    ///         bytes[0] = f[0];
    ///         bytes[1] = f[1];
    ///         bytes[2] = f[2];
    ///         bytes[3] = f[3];
    ///         bytes[4] = f[4];
    ///         bytes[5] = f[5];
    ///         u64::from_le_bytes(bytes)
    ///     })
    ///     .collect();
    /// let result1 = PublicTargetAddress::new(&public_target_address);
    ///
    /// let length = public_target_address_bytes.len() as u8 + 1;
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(PublicTargetAddress::data_type());
    /// data.append(&mut public_target_address_bytes.clone());
    ///
    /// let into_data: Vec<u8> = result1.into();
    /// assert_eq!(data, into_data);
    ///
    /// let result2 = PublicTargetAddress::try_from(&data);
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
                .public_target_address
                .clone()
                .iter()
                .flat_map(|f| f.to_le_bytes()[..6].to_vec())
                .collect(),
        );

        return data;
    }
}

impl DataType for PublicTargetAddress {
    /// return `0x17`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{public_target_address::PublicTargetAddress, data_type::DataType};
    ///
    /// assert_eq!(0x17, PublicTargetAddress::data_type());
    /// ```
    fn data_type() -> u8 {
        0x17
    }
}

/// check `Public Target Address` data type.
///
/// # Examples
///
/// ```
/// use ble_data_struct::data_types::public_target_address::*;
/// use ble_data_struct::data_types::data_type::DataType;
///
/// assert!(is_public_target_address(0x17));
/// assert!(!is_public_target_address(0x00));
/// ```
pub fn is_public_target_address(data_type: u8) -> bool {
    PublicTargetAddress::data_type() == data_type
}

#[cfg(test)]
mod tests {
    use crate::data_types::{data_type::DataType, public_target_address::*};

    #[test]
    fn test_new() {
        let public_target_address: Vec<u64> = [
            u64::from_le_bytes([
                0x01u8, 0x02u8, 0x03u8, 0x04u8, 0x05u8, 0x06u8, 0x00u8, 0x00u8,
            ]),
            u64::from_le_bytes([
                0x07u8, 0x08u8, 0x09u8, 0x0au8, 0x0bu8, 0x0cu8, 0x00u8, 0x00u8,
            ]),
        ]
        .to_vec();
        let result = PublicTargetAddress::new(&public_target_address);
        assert_eq!(public_target_address.len() as u8 * 6 + 1, result.length);
        assert_eq!(public_target_address, result.public_target_address);
    }

    #[test]
    fn test_try_from() {
        let public_target_address_bytes = [
            0x01u8, 0x02u8, 0x03u8, 0x04u8, 0x05u8, 0x06u8, 0x07u8, 0x08u8, 0x09u8, 0x0au8, 0x0bu8,
            0x0cu8,
        ]
        .to_vec();
        let public_target_address: Vec<u64> = public_target_address_bytes
            .windows(6)
            .step_by(6)
            .map(|f| {
                let mut bytes = [0x00u8; 8];
                bytes[0] = f[0];
                bytes[1] = f[1];
                bytes[2] = f[2];
                bytes[3] = f[3];
                bytes[4] = f[4];
                bytes[5] = f[5];
                u64::from_le_bytes(bytes)
            })
            .collect();
        let length = public_target_address_bytes.len() as u8 + 1;
        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(PublicTargetAddress::data_type());
        data.append(&mut public_target_address_bytes.clone());

        let result = PublicTargetAddress::try_from(&data);
        assert!(result.is_ok());
        let data_type = result.unwrap();
        assert_eq!(length, data_type.length);
        assert_eq!(public_target_address, data_type.public_target_address);

        let data: Vec<u8> = Vec::new();
        let result = PublicTargetAddress::try_from(&data);
        assert!(result.is_err());
        assert_eq!(
            format!("Invalid data size :{}", data.len()),
            result.unwrap_err()
        );
    }

    #[test]
    fn test_into() {
        let public_target_address_bytes = [
            0x01u8, 0x02u8, 0x03u8, 0x04u8, 0x05u8, 0x06u8, 0x07u8, 0x08u8, 0x09u8, 0x0au8, 0x0bu8,
            0x0cu8,
        ]
        .to_vec();
        let public_target_address: Vec<u64> = public_target_address_bytes
            .windows(6)
            .step_by(6)
            .map(|f| {
                let mut bytes = [0x00u8; 8];
                bytes[0] = f[0];
                bytes[1] = f[1];
                bytes[2] = f[2];
                bytes[3] = f[3];
                bytes[4] = f[4];
                bytes[5] = f[5];
                u64::from_le_bytes(bytes)
            })
            .collect();
        let result1 = PublicTargetAddress::new(&public_target_address);

        let length = public_target_address_bytes.len() as u8 + 1;
        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(PublicTargetAddress::data_type());
        data.append(&mut public_target_address_bytes.clone());

        let into_data: Vec<u8> = result1.into();
        assert_eq!(data, into_data);

        let result2 = PublicTargetAddress::try_from(&data);
        assert!(result2.is_ok());
        let data_type = result2.unwrap();
        let into_data: Vec<u8> = data_type.into();
        assert_eq!(data, into_data);
    }

    #[test]
    fn test_data_type() {
        assert_eq!(0x17, PublicTargetAddress::data_type());
    }

    #[test]
    fn test_is_public_target_address() {
        assert!(is_public_target_address(0x17));
        assert!(!is_public_target_address(0x00));
    }
}

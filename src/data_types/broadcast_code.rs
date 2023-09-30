//! Broadcast_Code (Data Type Value: 0x2d) module.

use crate::data_types::data_type::DataType;

/// Broadcast_Code.
pub struct BroadcastCode {
    /// data length
    pub length: u8,

    /// Broadcast_Code
    pub broadcast_code: Vec<u8>,
}

impl BroadcastCode {
    /// Create [BroadcastCode] from `Broadcast_Code`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::broadcast_code::BroadcastCode;
    ///
    /// let broadcast_code = [0x00u8; 4].to_vec();
    /// let result = BroadcastCode::new(&broadcast_code);
    /// assert_eq!(broadcast_code.len() as u8 + 1, result.length);
    /// assert_eq!(broadcast_code, result.broadcast_code);
    ///
    /// let broadcast_code = [0x3fu8, 0x42u8, 0x0fu8, 0x00u8].to_vec();
    /// let result = BroadcastCode::new(&broadcast_code);
    /// assert_eq!(broadcast_code.len() as u8 + 1, result.length);
    /// assert_eq!(broadcast_code, result.broadcast_code);
    ///
    /// let broadcast_code = [
    ///     0x00u8, 0x01u8, 0x02u8, 0x03u8, 0x04u8, 0x05u8, 0x06u8, 0x08u8, 0x09u8, 0x0au8, 0x0bu8,
    ///     0x0cu8, 0x0du8, 0x0eu8, 0x0fu8, 0x10u8,
    /// ]
    /// .to_vec();
    /// let result = BroadcastCode::new(&broadcast_code);
    /// assert_eq!(broadcast_code.len() as u8 + 1, result.length);
    /// assert_eq!(broadcast_code, result.broadcast_code);
    /// ```
    pub fn new(broadcast_code: &Vec<u8>) -> Self {
        Self {
            length: 1 + broadcast_code.len() as u8,
            broadcast_code: broadcast_code.clone(),
        }
    }

    /// Create [BroadcastCode] from `Vec<u8>` with offset.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{broadcast_code::BroadcastCode, data_type::DataType};
    ///
    /// let broadcast_code = [0x00u8; 4].to_vec();
    /// let length = broadcast_code.len() as u8 + 1;
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(BroadcastCode::data_type());
    /// data.append(&mut broadcast_code.clone());
    ///
    /// let result = BroadcastCode::from_with_offset(&data, 0);
    /// assert_eq!(length, result.length);
    /// assert_eq!(broadcast_code, result.broadcast_code);
    ///
    /// data = Vec::new();
    /// data.push(0);
    /// data.push(length);
    /// data.push(BroadcastCode::data_type());
    /// data.append(&mut broadcast_code.clone());
    ///
    /// let result = BroadcastCode::from_with_offset(&data, 1);
    /// assert_eq!(length, result.length);
    /// assert_eq!(broadcast_code, result.broadcast_code);
    ///
    /// let broadcast_code = [0x3fu8, 0x42u8, 0x0fu8, 0x00u8].to_vec();
    /// let length = broadcast_code.len() as u8 + 1;
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(BroadcastCode::data_type());
    /// data.append(&mut broadcast_code.clone());
    ///
    /// let result = BroadcastCode::from_with_offset(&data, 0);
    /// assert_eq!(length, result.length);
    /// assert_eq!(broadcast_code, result.broadcast_code);
    ///
    /// data = Vec::new();
    /// data.push(0);
    /// data.push(length);
    /// data.push(BroadcastCode::data_type());
    /// data.append(&mut broadcast_code.clone());
    ///
    /// let result = BroadcastCode::from_with_offset(&data, 1);
    /// assert_eq!(length, result.length);
    /// assert_eq!(broadcast_code, result.broadcast_code);
    ///
    /// let broadcast_code = [
    ///     0x00u8, 0x01u8, 0x02u8, 0x03u8, 0x04u8, 0x05u8, 0x06u8, 0x08u8, 0x09u8, 0x0au8, 0x0bu8,
    ///     0x0cu8, 0x0du8, 0x0eu8, 0x0fu8, 0x10u8,
    /// ]
    /// .to_vec();
    /// let length = broadcast_code.len() as u8 + 1;
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(BroadcastCode::data_type());
    /// data.append(&mut broadcast_code.clone());
    ///
    /// let result = BroadcastCode::from_with_offset(&data, 0);
    /// assert_eq!(length, result.length);
    /// assert_eq!(broadcast_code, result.broadcast_code);
    ///
    /// data = Vec::new();
    /// data.push(0);
    /// data.push(length);
    /// data.push(BroadcastCode::data_type());
    /// data.append(&mut broadcast_code.clone());
    ///
    /// let result = BroadcastCode::from_with_offset(&data, 1);
    /// assert_eq!(length, result.length);
    /// assert_eq!(broadcast_code, result.broadcast_code);
    /// ```
    pub fn from_with_offset(data: &Vec<u8>, offset: usize) -> Self {
        let data = data[offset..].to_vec();
        let length = data[0];
        Self {
            length,
            broadcast_code: data[2..1 + length as usize].to_vec(),
        }
    }
}

impl From<&Vec<u8>> for BroadcastCode {
    /// Create [BroadcastCode] from `Vec<u8>`.
    ///
    /// [`BroadcastCode::from_with_offset`]
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{broadcast_code::BroadcastCode, data_type::DataType};
    ///
    /// let broadcast_code = [0x00u8; 4].to_vec();
    /// let length = broadcast_code.len() as u8 + 1;
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(BroadcastCode::data_type());
    /// data.append(&mut broadcast_code.clone());
    ///
    /// let result = BroadcastCode::from(&data);
    /// assert_eq!(length, result.length);
    /// assert_eq!(broadcast_code, result.broadcast_code);
    ///
    /// let broadcast_code = [0x3fu8, 0x42u8, 0x0fu8, 0x00u8].to_vec();
    /// let length = broadcast_code.len() as u8 + 1;
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(BroadcastCode::data_type());
    /// data.append(&mut broadcast_code.clone());
    ///
    /// let result = BroadcastCode::from(&data);
    /// assert_eq!(length, result.length);
    /// assert_eq!(broadcast_code, result.broadcast_code);
    ///
    /// let broadcast_code = [
    ///     0x00u8, 0x01u8, 0x02u8, 0x03u8, 0x04u8, 0x05u8, 0x06u8, 0x08u8, 0x09u8, 0x0au8, 0x0bu8,
    ///     0x0cu8, 0x0du8, 0x0eu8, 0x0fu8, 0x10u8,
    /// ]
    /// .to_vec();
    /// let length = broadcast_code.len() as u8 + 1;
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(BroadcastCode::data_type());
    /// data.append(&mut broadcast_code.clone());
    ///
    /// let result = BroadcastCode::from(&data);
    /// assert_eq!(length, result.length);
    /// assert_eq!(broadcast_code, result.broadcast_code);
    /// ```
    fn from(data: &Vec<u8>) -> Self {
        Self::from_with_offset(data, 0)
    }
}

impl Into<Vec<u8>> for BroadcastCode {
    /// Create `Vec<u8>` from [BroadcastCode].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{broadcast_code::BroadcastCode, data_type::DataType};
    ///
    /// let broadcast_code = [0x00u8; 4].to_vec();
    /// let result1 = BroadcastCode::new(&broadcast_code);
    ///
    /// let length = broadcast_code.len() as u8 + 1;
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(BroadcastCode::data_type());
    /// data.append(&mut broadcast_code.clone());
    ///
    /// let into_data: Vec<u8> = result1.into();
    /// assert_eq!(data, into_data);
    ///
    /// let result2 = BroadcastCode::from(&data);
    /// let into_data: Vec<u8> = result2.into();
    /// assert_eq!(data, into_data);
    ///
    /// let broadcast_code = [0x3fu8, 0x42u8, 0x0fu8, 0x00u8].to_vec();
    /// let result1 = BroadcastCode::new(&broadcast_code);
    ///
    /// let length = broadcast_code.len() as u8 + 1;
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(BroadcastCode::data_type());
    /// data.append(&mut broadcast_code.clone());
    ///
    /// let into_data: Vec<u8> = result1.into();
    /// assert_eq!(data, into_data);
    ///
    /// let result2 = BroadcastCode::from(&data);
    /// let into_data: Vec<u8> = result2.into();
    /// assert_eq!(data, into_data);
    ///
    /// let broadcast_code = [
    ///     0x00u8, 0x01u8, 0x02u8, 0x03u8, 0x04u8, 0x05u8, 0x06u8, 0x08u8, 0x09u8, 0x0au8, 0x0bu8,
    ///     0x0cu8, 0x0du8, 0x0eu8, 0x0fu8, 0x10u8,
    /// ]
    /// .to_vec();
    /// let result1 = BroadcastCode::new(&broadcast_code);
    ///
    /// let length = broadcast_code.len() as u8 + 1;
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(BroadcastCode::data_type());
    /// data.append(&mut broadcast_code.clone());
    ///
    /// let into_data: Vec<u8> = result1.into();
    /// assert_eq!(data, into_data);
    ///
    /// let result2 = BroadcastCode::from(&data);
    /// let into_data: Vec<u8> = result2.into();
    /// assert_eq!(data, into_data);
    /// ```
    fn into(self) -> Vec<u8> {
        let mut data: Vec<u8> = Vec::new();
        data.push(self.length);
        data.push(Self::data_type());
        data.append(&mut self.broadcast_code.clone());
        return data;
    }
}

impl DataType for BroadcastCode {
    /// return `0x2d`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{broadcast_code::BroadcastCode, data_type::DataType};
    ///
    /// assert_eq!(0x2d, BroadcastCode::data_type());
    /// ```
    fn data_type() -> u8 {
        0x2d
    }
}

/// check `Broadcast_Code.` data type.
///
/// # Examples
///
/// ```
/// use ble_data_struct::data_types::broadcast_code::*;
/// use ble_data_struct::data_types::data_type::DataType;
///
/// assert!(is_broadcast_code(0x2d));
/// assert!(!is_broadcast_code(0x00));
/// ```
pub fn is_broadcast_code(data_type: u8) -> bool {
    BroadcastCode::data_type() == data_type
}

#[cfg(test)]
mod tests {

    use crate::data_types::{broadcast_code::*, data_type::DataType};

    #[test]
    fn test_new() {
        let broadcast_code = [0x00u8; 4].to_vec();
        let result = BroadcastCode::new(&broadcast_code);
        assert_eq!(broadcast_code.len() as u8 + 1, result.length);
        assert_eq!(broadcast_code, result.broadcast_code);

        let broadcast_code = [0x3fu8, 0x42u8, 0x0fu8, 0x00u8].to_vec();
        let result = BroadcastCode::new(&broadcast_code);
        assert_eq!(broadcast_code.len() as u8 + 1, result.length);
        assert_eq!(broadcast_code, result.broadcast_code);

        let broadcast_code = [
            0x00u8, 0x01u8, 0x02u8, 0x03u8, 0x04u8, 0x05u8, 0x06u8, 0x08u8, 0x09u8, 0x0au8, 0x0bu8,
            0x0cu8, 0x0du8, 0x0eu8, 0x0fu8, 0x10u8,
        ]
        .to_vec();
        let result = BroadcastCode::new(&broadcast_code);
        assert_eq!(broadcast_code.len() as u8 + 1, result.length);
        assert_eq!(broadcast_code, result.broadcast_code);
    }

    #[test]
    fn test_from_with_offset() {
        let broadcast_code = [0x00u8; 4].to_vec();
        let length = broadcast_code.len() as u8 + 1;
        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(BroadcastCode::data_type());
        data.append(&mut broadcast_code.clone());

        let result = BroadcastCode::from_with_offset(&data, 0);
        assert_eq!(length, result.length);
        assert_eq!(broadcast_code, result.broadcast_code);

        data = Vec::new();
        data.push(0);
        data.push(length);
        data.push(BroadcastCode::data_type());
        data.append(&mut broadcast_code.clone());

        let result = BroadcastCode::from_with_offset(&data, 1);
        assert_eq!(length, result.length);
        assert_eq!(broadcast_code, result.broadcast_code);

        let broadcast_code = [0x3fu8, 0x42u8, 0x0fu8, 0x00u8].to_vec();
        let length = broadcast_code.len() as u8 + 1;
        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(BroadcastCode::data_type());
        data.append(&mut broadcast_code.clone());

        let result = BroadcastCode::from_with_offset(&data, 0);
        assert_eq!(length, result.length);
        assert_eq!(broadcast_code, result.broadcast_code);

        data = Vec::new();
        data.push(0);
        data.push(length);
        data.push(BroadcastCode::data_type());
        data.append(&mut broadcast_code.clone());

        let result = BroadcastCode::from_with_offset(&data, 1);
        assert_eq!(length, result.length);
        assert_eq!(broadcast_code, result.broadcast_code);

        let broadcast_code = [
            0x00u8, 0x01u8, 0x02u8, 0x03u8, 0x04u8, 0x05u8, 0x06u8, 0x08u8, 0x09u8, 0x0au8, 0x0bu8,
            0x0cu8, 0x0du8, 0x0eu8, 0x0fu8, 0x10u8,
        ]
        .to_vec();
        let length = broadcast_code.len() as u8 + 1;
        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(BroadcastCode::data_type());
        data.append(&mut broadcast_code.clone());

        let result = BroadcastCode::from_with_offset(&data, 0);
        assert_eq!(length, result.length);
        assert_eq!(broadcast_code, result.broadcast_code);

        data = Vec::new();
        data.push(0);
        data.push(length);
        data.push(BroadcastCode::data_type());
        data.append(&mut broadcast_code.clone());

        let result = BroadcastCode::from_with_offset(&data, 1);
        assert_eq!(length, result.length);
        assert_eq!(broadcast_code, result.broadcast_code);
    }

    #[test]
    fn test_from() {
        let broadcast_code = [0x00u8; 4].to_vec();
        let length = broadcast_code.len() as u8 + 1;
        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(BroadcastCode::data_type());
        data.append(&mut broadcast_code.clone());

        let result = BroadcastCode::from(&data);
        assert_eq!(length, result.length);
        assert_eq!(broadcast_code, result.broadcast_code);

        let broadcast_code = [0x3fu8, 0x42u8, 0x0fu8, 0x00u8].to_vec();
        let length = broadcast_code.len() as u8 + 1;
        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(BroadcastCode::data_type());
        data.append(&mut broadcast_code.clone());

        let result = BroadcastCode::from(&data);
        assert_eq!(length, result.length);
        assert_eq!(broadcast_code, result.broadcast_code);

        let broadcast_code = [
            0x00u8, 0x01u8, 0x02u8, 0x03u8, 0x04u8, 0x05u8, 0x06u8, 0x08u8, 0x09u8, 0x0au8, 0x0bu8,
            0x0cu8, 0x0du8, 0x0eu8, 0x0fu8, 0x10u8,
        ]
        .to_vec();
        let length = broadcast_code.len() as u8 + 1;
        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(BroadcastCode::data_type());
        data.append(&mut broadcast_code.clone());

        let result = BroadcastCode::from(&data);
        assert_eq!(length, result.length);
        assert_eq!(broadcast_code, result.broadcast_code);
    }

    #[test]
    fn test_into() {
        let broadcast_code = [0x00u8; 4].to_vec();
        let result1 = BroadcastCode::new(&broadcast_code);

        let length = broadcast_code.len() as u8 + 1;
        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(BroadcastCode::data_type());
        data.append(&mut broadcast_code.clone());

        let into_data: Vec<u8> = result1.into();
        assert_eq!(data, into_data);

        let result2 = BroadcastCode::from(&data);
        let into_data: Vec<u8> = result2.into();
        assert_eq!(data, into_data);

        let broadcast_code = [0x3fu8, 0x42u8, 0x0fu8, 0x00u8].to_vec();
        let result1 = BroadcastCode::new(&broadcast_code);

        let length = broadcast_code.len() as u8 + 1;
        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(BroadcastCode::data_type());
        data.append(&mut broadcast_code.clone());

        let into_data: Vec<u8> = result1.into();
        assert_eq!(data, into_data);

        let result2 = BroadcastCode::from(&data);
        let into_data: Vec<u8> = result2.into();
        assert_eq!(data, into_data);

        let broadcast_code = [
            0x00u8, 0x01u8, 0x02u8, 0x03u8, 0x04u8, 0x05u8, 0x06u8, 0x08u8, 0x09u8, 0x0au8, 0x0bu8,
            0x0cu8, 0x0du8, 0x0eu8, 0x0fu8, 0x10u8,
        ]
        .to_vec();
        let result1 = BroadcastCode::new(&broadcast_code);

        let length = broadcast_code.len() as u8 + 1;
        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(BroadcastCode::data_type());
        data.append(&mut broadcast_code.clone());

        let into_data: Vec<u8> = result1.into();
        assert_eq!(data, into_data);

        let result2 = BroadcastCode::from(&data);
        let into_data: Vec<u8> = result2.into();
        assert_eq!(data, into_data);
    }

    #[test]
    fn test_data_type() {
        assert_eq!(0x2d, BroadcastCode::data_type());
    }

    #[test]
    fn test_is_broadcast_code() {
        assert!(is_broadcast_code(0x2d));
        assert!(!is_broadcast_code(0x00));
    }
}

//! Encrypted Data (Data Type Value: 0x31) module.

use crate::data_types::data_type::DataType;

/// Encrypted Data.
pub struct EncryptedData {
    /// data length
    pub length: u8,

    /// Randomizer
    pub randomizer: [u8; 5],

    /// Payload
    pub payload: Vec<u8>,

    /// MIC
    pub mic: [u8; 4],
}

impl EncryptedData {
    /// Create [EncryptedData] from `Encrypted Data`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_class::data_types::encrypted_data::EncryptedData;
    ///
    /// let randomizer: [u8; 5] = [1, 2, 3, 4, 5];
    /// let payload = [6].to_vec();
    /// let mic: [u8; 4] = [7, 8, 9, 10];
    /// let result = EncryptedData::new(randomizer, &payload, mic);
    /// assert_eq!(11, result.length);
    /// assert_eq!(randomizer, result.randomizer);
    /// assert_eq!(payload, result.payload);
    /// assert_eq!(mic, result.mic);
    /// ```
    pub fn new(randomizer: [u8; 5], payload: &Vec<u8>, mic: [u8; 4]) -> Self {
        Self {
            length: 10 + payload.len() as u8,
            randomizer: randomizer.clone(),
            payload: payload.clone(),
            mic: mic.clone(),
        }
    }

    /// Create [EncryptedData] from `Vec<u8>` with offset.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_class::data_types::{encrypted_data::EncryptedData, data_type::DataType};
    ///
    /// let randomizer: [u8; 5] = [1, 2, 3, 4, 5];
    /// let payload = [6].to_vec();
    /// let mic: [u8; 4] = [7, 8, 9, 10];
    /// let length = 11;
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(EncryptedData::data_type());
    /// data.append(&mut randomizer.to_vec());
    /// data.append(&mut payload.clone());
    /// data.append(&mut mic.to_vec());
    /// 
    /// let result = EncryptedData::from_with_offset(&data, 0);
    /// assert_eq!(length, result.length);
    /// assert_eq!(randomizer, result.randomizer);
    /// assert_eq!(payload, result.payload);
    /// assert_eq!(mic, result.mic);
    /// 
    /// data = Vec::new();
    /// data.push(0);
    /// data.push(length);
    /// data.push(EncryptedData::data_type());
    /// data.append(&mut randomizer.to_vec());
    /// data.append(&mut payload.clone());
    /// data.append(&mut mic.to_vec());
    /// let result = EncryptedData::from_with_offset(&data, 1);
    /// assert_eq!(length, result.length);
    /// assert_eq!(randomizer, result.randomizer);
    /// assert_eq!(payload, result.payload);
    /// assert_eq!(mic, result.mic);
    /// ```
    pub fn from_with_offset(data: &Vec<u8>, offset: usize) -> Self {
        let data = data[offset..].to_vec();
        let length = data[0];
        Self {
            length,
            randomizer: data[2..7].try_into().unwrap(),
            payload: data[7..(length - 3) as usize].to_vec(),
            mic: data[data.len() - 4..].try_into().unwrap(),
        }
    }
}

impl From<&Vec<u8>> for EncryptedData {
    /// Create [EncryptedData] from `Vec<u8>`.
    ///
    /// [`EncryptedData::from_with_offset`]
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_class::data_types::{encrypted_data::EncryptedData, data_type::DataType};
    ///
    /// let randomizer: [u8; 5] = [1, 2, 3, 4, 5];
    /// let payload = [6].to_vec();
    /// let mic: [u8; 4] = [7, 8, 9, 10];
    /// let length = 11;
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(EncryptedData::data_type());
    /// data.append(&mut randomizer.to_vec());
    /// data.append(&mut payload.clone());
    /// data.append(&mut mic.to_vec());
    /// 
    /// let result = EncryptedData::from(&data);
    /// assert_eq!(length, result.length);
    /// assert_eq!(randomizer, result.randomizer);
    /// assert_eq!(payload, result.payload);
    /// assert_eq!(mic, result.mic);
    /// ```
    fn from(data: &Vec<u8>) -> Self {
        Self::from_with_offset(data, 0)
    }
}

impl Into<Vec<u8>> for EncryptedData {
    /// Create `Vec<u8>` from [EncryptedData].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_class::data_types::{encrypted_data::EncryptedData, data_type::DataType};
    ///
    /// let randomizer: [u8; 5] = [1, 2, 3, 4, 5];
    /// let payload = [6].to_vec();
    /// let mic: [u8; 4] = [7, 8, 9, 10];
    /// let length = 11;
    /// let result1 = EncryptedData::new(randomizer, &payload, mic);
    /// 
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(EncryptedData::data_type());
    /// data.append(&mut randomizer.to_vec());
    /// data.append(&mut payload.clone());
    /// data.append(&mut mic.to_vec());
    /// 
    /// let into_data: Vec<u8> = result1.into();
    /// assert_eq!(data, into_data);
    /// 
    /// let result2 = EncryptedData::from(&data);
    /// let into_data: Vec<u8> = result2.into();
    /// assert_eq!(data, into_data);
    /// ```
    fn into(self) -> Vec<u8> {
        let mut data: Vec<u8> = Vec::new();
        data.push(self.length);
        data.push(Self::data_type());
        data.append(&mut self.randomizer.clone().to_vec());
        data.append(&mut self.payload.clone());
        data.append(&mut self.mic.clone().to_vec());
        return data;
    }
}

impl DataType for EncryptedData {
    /// return `0x31`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_class::data_types::{encrypted_data::EncryptedData, data_type::DataType};
    ///
    /// assert_eq!(0x31, EncryptedData::data_type());
    /// ```
    fn data_type() -> u8 {
        0x31
    }
}

/// check `Encrypted Data` data type.
///
/// # Examples
///
/// ```
/// use ble_data_class::data_types::encrypted_data::*;
/// use ble_data_class::data_types::data_type::DataType;
///
/// assert!(is_encrypted_data(0x31));
/// assert!(!is_encrypted_data(0x00));
/// ```
pub fn is_encrypted_data(data_type: u8) -> bool {
    EncryptedData::data_type() == data_type
}

#[cfg(test)]
mod tests {
    use crate::data_types::{data_type::DataType, encrypted_data::{EncryptedData, is_encrypted_data}};

    #[test]
    fn test_new() {
        let randomizer: [u8; 5] = [1, 2, 3, 4, 5];
        let payload = [6].to_vec();
        let mic: [u8; 4] = [7, 8, 9, 10];
        let result = EncryptedData::new(randomizer, &payload, mic);
        assert_eq!(11, result.length);
        assert_eq!(randomizer, result.randomizer);
        assert_eq!(payload, result.payload);
        assert_eq!(mic, result.mic);
    }

    #[test]
    fn test_from_with_offset() {
        let randomizer: [u8; 5] = [1, 2, 3, 4, 5];
        let payload = [6].to_vec();
        let mic: [u8; 4] = [7, 8, 9, 10];
        let length = 11;
        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(EncryptedData::data_type());
        data.append(&mut randomizer.to_vec());
        data.append(&mut payload.clone());
        data.append(&mut mic.to_vec());

        let result = EncryptedData::from_with_offset(&data, 0);
        assert_eq!(length, result.length);
        assert_eq!(randomizer, result.randomizer);
        assert_eq!(payload, result.payload);
        assert_eq!(mic, result.mic);

        data = Vec::new();
        data.push(0);
        data.push(length);
        data.push(EncryptedData::data_type());
        data.append(&mut randomizer.to_vec());
        data.append(&mut payload.clone());
        data.append(&mut mic.to_vec());
        let result = EncryptedData::from_with_offset(&data, 1);
        assert_eq!(length, result.length);
        assert_eq!(randomizer, result.randomizer);
        assert_eq!(payload, result.payload);
        assert_eq!(mic, result.mic);
    }

    #[test]
    fn test_from() {
        let randomizer: [u8; 5] = [1, 2, 3, 4, 5];
        let payload = [6].to_vec();
        let mic: [u8; 4] = [7, 8, 9, 10];
        let length = 11;
        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(EncryptedData::data_type());
        data.append(&mut randomizer.to_vec());
        data.append(&mut payload.clone());
        data.append(&mut mic.to_vec());

        let result = EncryptedData::from(&data);
        assert_eq!(length, result.length);
        assert_eq!(randomizer, result.randomizer);
        assert_eq!(payload, result.payload);
        assert_eq!(mic, result.mic);
    }

    #[test]
    fn test_into() {
        let randomizer: [u8; 5] = [1, 2, 3, 4, 5];
        let payload = [6].to_vec();
        let mic: [u8; 4] = [7, 8, 9, 10];
        let length = 11;
        let result1 = EncryptedData::new(randomizer, &payload, mic);

        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(EncryptedData::data_type());
        data.append(&mut randomizer.to_vec());
        data.append(&mut payload.clone());
        data.append(&mut mic.to_vec());

        let into_data: Vec<u8> = result1.into();
        assert_eq!(data, into_data);

        let result2 = EncryptedData::from(&data);
        let into_data: Vec<u8> = result2.into();
        assert_eq!(data, into_data);
    }

    #[test]
    fn test_data_type() {
        assert_eq!(0x31, EncryptedData::data_type());
    }

    #[test]
    fn test_is_encrypted_data() {
        assert!(is_encrypted_data(0x31));
        assert!(!is_encrypted_data(0x00));
    }
}

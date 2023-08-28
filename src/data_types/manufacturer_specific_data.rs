//! Manufacturer Specific Data (Data Type Value: 0xff) module.

use crate::data_types::data_type::DataType;

/// Manufacturer Specific Data.
pub struct ManufacturerSpecificData {
    /// data length
    pub length: u8,

    /// Company Identifier Code
    pub company_identifier: u16,

    /// Manufacturer Specific Data
    pub manufacturer_specific_data: Vec<u8>,
}

impl ManufacturerSpecificData {
    /// Create [ManufacturerSpecificData] from Parameters.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_class::data_types::manufacturer_specific_data::ManufacturerSpecificData;
    ///
    /// let company_identifier = 0x0ca8u16;
    /// let manufacturer_specific_data = [0x03u8].to_vec();
    /// let result = ManufacturerSpecificData::new(company_identifier, &manufacturer_specific_data);
    /// assert_eq!(manufacturer_specific_data.len() as u8 + 3, result.length);
    /// assert_eq!(company_identifier, result.company_identifier);
    /// assert_eq!(
    ///     manufacturer_specific_data,
    ///     result.manufacturer_specific_data
    /// );
    /// ```
    pub fn new(company_identifier: u16, manufacturer_specific_data: &Vec<u8>) -> Self {
        Self {
            length: 3 + manufacturer_specific_data.len() as u8,
            company_identifier,
            manufacturer_specific_data: manufacturer_specific_data.clone(),
        }
    }

    /// Create [ManufacturerSpecificData] from `Vec<u8>` with offset.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_class::data_types::{manufacturer_specific_data::ManufacturerSpecificData, data_type::DataType};
    ///
    /// let company_identifier = 0x0ca8u16;
    /// let manufacturer_specific_data = [0x03u8].to_vec();
    /// let length = manufacturer_specific_data.len() as u8 + 3;
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(ManufacturerSpecificData::data_type());
    /// data.append(&mut u16::to_le_bytes(company_identifier).try_into().unwrap());
    /// data.append(&mut manufacturer_specific_data.clone());
    ///
    /// let result = ManufacturerSpecificData::from_with_offset(&data, 0);
    /// assert_eq!(length, result.length);
    /// assert_eq!(company_identifier, result.company_identifier);
    /// assert_eq!(
    ///     manufacturer_specific_data,
    ///     result.manufacturer_specific_data
    /// );
    ///
    /// data = Vec::new();
    /// data.push(0);
    /// data.push(length);
    /// data.push(ManufacturerSpecificData::data_type());
    /// data.append(&mut u16::to_le_bytes(company_identifier).try_into().unwrap());
    /// data.append(&mut manufacturer_specific_data.clone());
    ///
    /// let result = ManufacturerSpecificData::from_with_offset(&data, 1);
    /// assert_eq!(length, result.length);
    /// assert_eq!(company_identifier, result.company_identifier);
    /// assert_eq!(
    ///     manufacturer_specific_data,
    ///     result.manufacturer_specific_data
    /// );
    /// ```
    pub fn from_with_offset(data: &Vec<u8>, offset: usize) -> Self {
        let data = data[offset..].to_vec();
        let length = data[0];
        Self {
            length,
            company_identifier: u16::from_le_bytes(data[2..4].try_into().unwrap()),
            manufacturer_specific_data: data[4..1 + length as usize].to_vec(),
        }
    }
}

impl From<&Vec<u8>> for ManufacturerSpecificData {
    /// Create [ManufacturerSpecificData] from `Vec<u8>`.
    ///
    /// [`ManufacturerSpecificData::from_with_offset`]
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_class::data_types::{manufacturer_specific_data::ManufacturerSpecificData, data_type::DataType};
    ///
    /// let company_identifier = 0x0ca8u16;
    /// let manufacturer_specific_data = [0x03u8].to_vec();
    /// let length = manufacturer_specific_data.len() as u8 + 3;
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(ManufacturerSpecificData::data_type());
    /// data.append(&mut u16::to_le_bytes(company_identifier).try_into().unwrap());
    /// data.append(&mut manufacturer_specific_data.clone());
    ///
    /// let result = ManufacturerSpecificData::from(&data);
    /// assert_eq!(length, result.length);
    /// assert_eq!(company_identifier, result.company_identifier);
    /// assert_eq!(
    ///     manufacturer_specific_data,
    ///     result.manufacturer_specific_data
    /// );
    /// ```
    fn from(data: &Vec<u8>) -> Self {
        Self::from_with_offset(data, 0)
    }
}

impl Into<Vec<u8>> for ManufacturerSpecificData {
    /// Create `Vec<u8>` from [ManufacturerSpecificData].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_class::data_types::{manufacturer_specific_data::ManufacturerSpecificData, data_type::DataType};
    ///
    /// let company_identifier = 0x0ca8u16;
    /// let manufacturer_specific_data = [0x03u8].to_vec();
    /// let result1 =
    ///     ManufacturerSpecificData::new(company_identifier, &manufacturer_specific_data);
    ///
    /// let length = manufacturer_specific_data.len() as u8 + 3;
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(ManufacturerSpecificData::data_type());
    /// data.append(&mut u16::to_le_bytes(company_identifier).try_into().unwrap());
    /// data.append(&mut manufacturer_specific_data.clone());
    ///
    /// let into_data: Vec<u8> = result1.into();
    /// assert_eq!(data, into_data);
    ///
    /// let result2 = ManufacturerSpecificData::from(&data);
    /// let into_data: Vec<u8> = result2.into();
    /// assert_eq!(data, into_data);
    /// ```
    fn into(self) -> Vec<u8> {
        let mut data: Vec<u8> = Vec::new();
        data.push(self.length);
        data.push(Self::data_type());
        data.append(&mut self.company_identifier.to_le_bytes().to_vec());
        data.append(&mut self.manufacturer_specific_data.clone());
        return data;
    }
}

impl DataType for ManufacturerSpecificData {
    /// return `0xff`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_class::data_types::{manufacturer_specific_data::ManufacturerSpecificData, data_type::DataType};
    ///
    /// assert_eq!(0xff, ManufacturerSpecificData::data_type());
    /// ```
    fn data_type() -> u8 {
        0xff
    }
}

/// check `Manufacturer Specific Data.` data type.
///
/// # Examples
///
/// ```
/// use ble_data_class::data_types::manufacturer_specific_data::*;
/// use ble_data_class::data_types::data_type::DataType;
///
/// assert!(is_manufacturer_specific_data(0xff));
/// assert!(!is_manufacturer_specific_data(0x00));
/// ```
pub fn is_manufacturer_specific_data(data_type: u8) -> bool {
    ManufacturerSpecificData::data_type() == data_type
}

#[cfg(test)]
mod tests {

    use crate::data_types::{data_type::DataType, manufacturer_specific_data::*};

    #[test]
    fn test_new() {
        let company_identifier = 0x0ca8u16;
        let manufacturer_specific_data = [0x03u8].to_vec();
        let result = ManufacturerSpecificData::new(company_identifier, &manufacturer_specific_data);
        assert_eq!(manufacturer_specific_data.len() as u8 + 3, result.length);
        assert_eq!(company_identifier, result.company_identifier);
        assert_eq!(
            manufacturer_specific_data,
            result.manufacturer_specific_data
        );
    }

    #[test]
    fn test_from_with_offset() {
        let company_identifier = 0x0ca8u16;
        let manufacturer_specific_data = [0x03u8].to_vec();
        let length = manufacturer_specific_data.len() as u8 + 3;
        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(ManufacturerSpecificData::data_type());
        data.append(&mut u16::to_le_bytes(company_identifier).try_into().unwrap());
        data.append(&mut manufacturer_specific_data.clone());

        let result = ManufacturerSpecificData::from_with_offset(&data, 0);
        assert_eq!(length, result.length);
        assert_eq!(company_identifier, result.company_identifier);
        assert_eq!(
            manufacturer_specific_data,
            result.manufacturer_specific_data
        );

        data = Vec::new();
        data.push(0);
        data.push(length);
        data.push(ManufacturerSpecificData::data_type());
        data.append(&mut u16::to_le_bytes(company_identifier).try_into().unwrap());
        data.append(&mut manufacturer_specific_data.clone());

        let result = ManufacturerSpecificData::from_with_offset(&data, 1);
        assert_eq!(length, result.length);
        assert_eq!(company_identifier, result.company_identifier);
        assert_eq!(
            manufacturer_specific_data,
            result.manufacturer_specific_data
        );
    }

    #[test]
    fn test_from() {
        let company_identifier = 0x0ca8u16;
        let manufacturer_specific_data = [0x03u8].to_vec();
        let length = manufacturer_specific_data.len() as u8 + 3;
        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(ManufacturerSpecificData::data_type());
        data.append(&mut u16::to_le_bytes(company_identifier).try_into().unwrap());
        data.append(&mut manufacturer_specific_data.clone());

        let result = ManufacturerSpecificData::from(&data);
        assert_eq!(length, result.length);
        assert_eq!(company_identifier, result.company_identifier);
        assert_eq!(
            manufacturer_specific_data,
            result.manufacturer_specific_data
        );
    }

    #[test]
    fn test_into() {
        let company_identifier = 0x0ca8u16;
        let manufacturer_specific_data = [0x03u8].to_vec();
        let result1 =
            ManufacturerSpecificData::new(company_identifier, &manufacturer_specific_data);

        let length = manufacturer_specific_data.len() as u8 + 3;
        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(ManufacturerSpecificData::data_type());
        data.append(&mut u16::to_le_bytes(company_identifier).try_into().unwrap());
        data.append(&mut manufacturer_specific_data.clone());

        let into_data: Vec<u8> = result1.into();
        assert_eq!(data, into_data);

        let result2 = ManufacturerSpecificData::from(&data);
        let into_data: Vec<u8> = result2.into();
        assert_eq!(data, into_data);
    }

    #[test]
    fn test_data_type() {
        assert_eq!(0xff, ManufacturerSpecificData::data_type());
    }

    #[test]
    fn test_is_manufacturer_specific_data() {
        assert!(is_manufacturer_specific_data(0xff));
        assert!(!is_manufacturer_specific_data(0x00));
    }
}

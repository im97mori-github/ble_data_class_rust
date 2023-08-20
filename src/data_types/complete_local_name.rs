//! Complete Local Name (Data Type Value: 0x09) module.

use crate::data_types::data_type::DataType;
use crate::vec_converter::VecConverter;

/// Complete Local Name.
pub struct CompleteLocalName {
    /// data length
    pub length: u8,

    /// Complete Local Name
    pub complete_local_name: String,
}

const DATA_TYPE: u8 = 0x08;

impl CompleteLocalName {
    /// Create [CompleteLocalName] from `utf8`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_class::data_types::complete_local_name::CompleteLocalName;
    ///
    /// let name = "complete_local_name".to_string();
    /// let result = CompleteLocalName::new(&name);
    /// assert_eq!(name.as_bytes().len() as u8 + 1, result.length);
    /// assert_eq!(name, result.complete_local_name);
    /// ```
    pub fn new(complete_local_name: &String) -> CompleteLocalName {
        CompleteLocalName {
            length: complete_local_name.as_bytes().len() as u8 + 1,
            complete_local_name: complete_local_name.to_string(),
        }
    }
}

impl VecConverter<CompleteLocalName> for CompleteLocalName {
    /// Create [Vec`<u8>`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_class::vec_converter::VecConverter;
    /// use ble_data_class::data_types::complete_local_name::CompleteLocalName;
    /// use ble_data_class::data_types::data_type::DataType;
    ///
    /// let name = "complete_local_name".to_string();
    /// let result1 = CompleteLocalName::new(&name);
    ///
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(name.as_bytes().len() as u8 + 1);
    /// data.push(CompleteLocalName::data_type());
    /// data.append(&mut name.to_string().into_bytes());
    ///
    /// assert_eq!(data, result1.into_bytes());
    /// ```
    fn into_bytes(&self) -> Vec<u8> {
        let mut data: Vec<u8> = Vec::new();
        data.push(self.length);
        data.push(DATA_TYPE);
        data.append(&mut self.complete_local_name.clone().into_bytes());
        return data;
    }

    /// Create [[CompleteLocalName] from [Vec`<u8>`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_class::vec_converter::VecConverter;
    /// use ble_data_class::data_types::complete_local_name::CompleteLocalName;
    /// use ble_data_class::data_types::data_type::DataType;
    ///
    /// let name = "complete_local_name".to_string();
    /// let length = name.as_bytes().len() as u8 + 1;
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(CompleteLocalName::data_type());
    /// data.append(&mut name.to_string().into_bytes());
    /// let result = CompleteLocalName::from_vec(&data);
    /// assert_eq!(length, result.length);
    /// assert_eq!(name, result.complete_local_name);
    /// ```
    fn from_vec(data: &Vec<u8>) -> CompleteLocalName {
        let length = data[0];
        CompleteLocalName {
            length,
            complete_local_name: String::from_utf8(data[2..1 + usize::from(length)].to_vec())
                .unwrap(),
        }
    }
}

impl DataType<CompleteLocalName> for CompleteLocalName {
    /// return `0x08`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_class::data_types::complete_local_name::CompleteLocalName;
    /// use ble_data_class::data_types::data_type::DataType;
    ///
    /// assert_eq!(0x08, CompleteLocalName::data_type());
    /// ```
    fn data_type() -> u8 {
        DATA_TYPE
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        data_types::{
            complete_local_name::{CompleteLocalName, DATA_TYPE},
            data_type::DataType,
        },
        vec_converter::VecConverter,
    };

    #[test]
    fn test_new() {
        let name = "complete_local_name".to_string();
        let result = CompleteLocalName::new(&name);
        assert_eq!(name.as_bytes().len() as u8 + 1, result.length);
        assert_eq!(name, result.complete_local_name);
    }

    #[test]
    fn test_from_vec() {
        let name = "complete_local_name".to_string();
        let length = name.as_bytes().len() as u8 + 1;
        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(DATA_TYPE);
        data.append(&mut name.to_string().into_bytes());
        let result = CompleteLocalName::from_vec(&data);
        assert_eq!(length, result.length);
        assert_eq!(name, result.complete_local_name);
    }

    #[test]
    fn test_to_vec() {
        let name = "complete_local_name".to_string();
        let result1 = CompleteLocalName::new(&name);

        let mut data: Vec<u8> = Vec::new();
        data.push(name.as_bytes().len() as u8 + 1);
        data.push(DATA_TYPE);
        data.append(&mut name.to_string().into_bytes());

        assert_eq!(data, result1.into_bytes());

        let result2 = CompleteLocalName::from_vec(&data);
        assert_eq!(data, result2.into_bytes());
    }

    #[test]
    fn test_data_type() {
        assert_eq!(DATA_TYPE, CompleteLocalName::data_type());
    }
}

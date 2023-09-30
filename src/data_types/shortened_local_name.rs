//! Shortened Local Name (Data Type Value: 0x08) module.

use crate::data_types::data_type::DataType;

/// Shortened Local Name.
pub struct ShortenedLocalName {
    /// data length
    pub length: u8,

    /// Shortened Local Name
    pub shortened_local_name: String,
}

impl ShortenedLocalName {
    /// Create [ShortenedLocalName] from `utf8`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::shortened_local_name::ShortenedLocalName;
    ///
    /// let name = "shortened_local_name".to_string();
    /// let result = ShortenedLocalName::new(&name);
    /// assert_eq!(name.as_bytes().len() as u8 + 1, result.length);
    /// assert_eq!(name, result.shortened_local_name);
    /// ```
    pub fn new(shortened_local_name: &String) -> Self {
        Self {
            length: shortened_local_name.as_bytes().len() as u8 + 1,
            shortened_local_name: shortened_local_name.to_string(),
        }
    }

    /// Create [ShortenedLocalName] from `Vec<u8>` with offset.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{shortened_local_name::ShortenedLocalName, data_type::DataType};
    ///
    /// let name = "shortened_local_name".to_string();
    /// let length = name.as_bytes().len() as u8 + 1;
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(ShortenedLocalName::data_type());
    /// data.append(&mut name.to_string().into_bytes());
    ///
    /// let result = ShortenedLocalName::from_with_offset(&data, 0);
    /// assert_eq!(length, result.length);
    /// assert_eq!(name, result.shortened_local_name);
    ///
    /// data = Vec::new();
    /// data.push(0);
    /// data.push(length);
    /// data.push(ShortenedLocalName::data_type());
    /// data.append(&mut name.to_string().into_bytes());
    /// let result = ShortenedLocalName::from_with_offset(&data, 1);
    /// assert_eq!(length, result.length);
    /// assert_eq!(name, result.shortened_local_name);
    /// ```
    pub fn from_with_offset(data: &Vec<u8>, offset: usize) -> Self {
        let data = data[offset..].to_vec();
        let length = data[0];
        Self {
            length,
            shortened_local_name: String::from_utf8(data[2..1 + usize::from(length)].to_vec())
                .unwrap(),
        }
    }
}

impl From<&Vec<u8>> for ShortenedLocalName {
    /// Create [ShortenedLocalName] from `Vec<u8>`.
    ///
    /// [`ShortenedLocalName::from_with_offset`]
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{shortened_local_name::ShortenedLocalName, data_type::DataType};
    ///
    /// let name = "shortened_local_name".to_string();
    /// let length = name.as_bytes().len() as u8 + 1;
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(ShortenedLocalName::data_type());
    /// data.append(&mut name.to_string().into_bytes());
    /// let result = ShortenedLocalName::from(&data);
    /// assert_eq!(length, result.length);
    /// assert_eq!(name, result.shortened_local_name);
    /// ```
    fn from(data: &Vec<u8>) -> Self {
        Self::from_with_offset(data, 0)
    }
}

impl Into<Vec<u8>> for ShortenedLocalName {
    /// Create `Vec<u8>` from [ShortenedLocalName].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{shortened_local_name::ShortenedLocalName, data_type::DataType};
    ///
    /// let name = "shortened_local_name".to_string();
    /// let result1 = ShortenedLocalName::new(&name);
    ///
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(name.as_bytes().len() as u8 + 1);
    /// data.push(ShortenedLocalName::data_type());
    /// data.append(&mut name.to_string().into_bytes());
    ///
    /// let into_data: Vec<u8> = result1.into();
    /// assert_eq!(data, into_data);
    /// ```
    fn into(self) -> Vec<u8> {
        let mut data: Vec<u8> = Vec::new();
        data.push(self.length);
        data.push(Self::data_type());
        data.append(&mut self.shortened_local_name.clone().into_bytes());
        return data;
    }
}

impl DataType for ShortenedLocalName {
    /// return `0x08`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{shortened_local_name::ShortenedLocalName, data_type::DataType};
    ///
    /// assert_eq!(0x08, ShortenedLocalName::data_type());
    /// ```
    fn data_type() -> u8 {
        0x08
    }
}

/// check `Shortened Local Name` data type.
///
/// # Examples
///
/// ```
/// use ble_data_struct::data_types::shortened_local_name::*;
/// use ble_data_struct::data_types::data_type::DataType;
///
/// assert!(is_shortened_local_name(0x08));
/// assert!(!is_shortened_local_name(0x00));
/// ```
pub fn is_shortened_local_name(data_type: u8) -> bool {
    ShortenedLocalName::data_type() == data_type
}

#[cfg(test)]
mod tests {
    use crate::data_types::{shortened_local_name::*, data_type::DataType};

    #[test]
    fn test_new() {
        let name = "shortened_local_name".to_string();
        let result = ShortenedLocalName::new(&name);
        assert_eq!(name.as_bytes().len() as u8 + 1, result.length);
        assert_eq!(name, result.shortened_local_name);
    }

    #[test]
    fn test_from_with_offset() {
        let name = "shortened_local_name".to_string();
        let length = name.as_bytes().len() as u8 + 1;
        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(ShortenedLocalName::data_type());
        data.append(&mut name.to_string().into_bytes());

        let result = ShortenedLocalName::from_with_offset(&data, 0);
        assert_eq!(length, result.length);
        assert_eq!(name, result.shortened_local_name);

        data = Vec::new();
        data.push(0);
        data.push(length);
        data.push(ShortenedLocalName::data_type());
        data.append(&mut name.to_string().into_bytes());
        let result = ShortenedLocalName::from_with_offset(&data, 1);
        assert_eq!(length, result.length);
        assert_eq!(name, result.shortened_local_name);
    }

    #[test]
    fn test_from() {
        let name = "shortened_local_name".to_string();
        let length = name.as_bytes().len() as u8 + 1;
        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(ShortenedLocalName::data_type());
        data.append(&mut name.to_string().into_bytes());
        let result = ShortenedLocalName::from(&data);
        assert_eq!(length, result.length);
        assert_eq!(name, result.shortened_local_name);
    }

    #[test]
    fn test_into() {
        let name = "shortened_local_name".to_string();
        let result1 = ShortenedLocalName::new(&name);

        let mut data: Vec<u8> = Vec::new();
        data.push(name.as_bytes().len() as u8 + 1);
        data.push(ShortenedLocalName::data_type());
        data.append(&mut name.to_string().into_bytes());

        let into_data: Vec<u8> = result1.into();
        assert_eq!(data, into_data);

        let result2 = ShortenedLocalName::from(&data);
        let into_data: Vec<u8> = result2.into();
        assert_eq!(data, into_data);
    }

    #[test]
    fn test_data_type() {
        assert_eq!(0x08, ShortenedLocalName::data_type());
    }

    #[test]
    fn test_is_shortened_local_name() {
        assert!(is_shortened_local_name(0x08));
        assert!(!is_shortened_local_name(0x00));
    }
}

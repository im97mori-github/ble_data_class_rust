//! Shortened Local Name (Data Type Value: 0x08) module.

use crate::data_types::data_type::DataType;

/// Shortened Local Name.

#[derive(Debug)]
pub struct ShortenedLocalName {
    /// data length
    pub length: u8,

    /// Shortened Local Name
    pub shortened_local_name: String,
}

impl ShortenedLocalName {
    /// Create [`ShortenedLocalName`] from `utf8`.
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
}

impl TryFrom<&Vec<u8>> for ShortenedLocalName {
    type Error = String;
    /// Create [`ShortenedLocalName`] from `Vec<u8>`.
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
    /// let result = ShortenedLocalName::try_from(&data);
    /// assert!(result.is_ok());
    /// let data_type = result.unwrap();
    /// assert_eq!(length, data_type.length);
    /// assert_eq!(name, data_type.shortened_local_name);
    ///
    /// let data: Vec<u8> = Vec::new();
    /// let result = ShortenedLocalName::try_from(&data);
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
            shortened_local_name: String::from_utf8(value[2..1 + usize::from(length)].to_vec())
                .unwrap(),
        })
    }
}

impl Into<Vec<u8>> for ShortenedLocalName {
    /// Create `Vec<u8>` from [`ShortenedLocalName`].
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
    ///
    /// let result2 = ShortenedLocalName::try_from(&data);
    /// assert!(result2.is_ok());
    /// let data_type = result2.unwrap();
    /// let into_data: Vec<u8> = data_type.into();
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
    use crate::data_types::{data_type::DataType, shortened_local_name::*};

    #[test]
    fn test_new() {
        let name = "shortened_local_name".to_string();
        let result = ShortenedLocalName::new(&name);
        assert_eq!(name.as_bytes().len() as u8 + 1, result.length);
        assert_eq!(name, result.shortened_local_name);
    }

    #[test]
    fn test_try_from() {
        let name = "shortened_local_name".to_string();
        let length = name.as_bytes().len() as u8 + 1;
        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(ShortenedLocalName::data_type());
        data.append(&mut name.to_string().into_bytes());

        let result = ShortenedLocalName::try_from(&data);
        assert!(result.is_ok());
        let data_type = result.unwrap();
        assert_eq!(length, data_type.length);
        assert_eq!(name, data_type.shortened_local_name);

        let mut data: Vec<u8> = vec![0u8; 2];
        data[0] = data.len() as u8 - 1;
        let result = ShortenedLocalName::try_from(&data);
        assert!(result.is_err());
        assert_eq!(
            format!("Invalid data size :{}", data.len()),
            result.unwrap_err()
        );
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

        let result2 = ShortenedLocalName::try_from(&data);
        assert!(result2.is_ok());
        let data_type = result2.unwrap();
        let into_data: Vec<u8> = data_type.into();
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

//! Uniform Resource Identifier (Data Type Value: 0x08) module.

use crate::data_types::data_type::DataType;

/// Uniform Resource Identifier.
#[derive(Debug, PartialEq, Clone)]
pub struct UniformResourceIdentifier {
    /// data length
    pub length: u8,

    /// Scheme
    pub scheme: char,

    /// Uniform Resource Identifier
    pub uniform_resource_identifier: String,
}

impl UniformResourceIdentifier {
    /// Create [`UniformResourceIdentifier`] from `utf8`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::uniform_resource_identifier::UniformResourceIdentifier;
    ///
    /// let scheme = '\u{0016}';
    /// let body = "uniform_resource_identifier";
    /// let uri = scheme.to_string() + body;
    /// let result = UniformResourceIdentifier::new(&uri);
    /// assert_eq!(uri.as_bytes().len() as u8 + 1, result.length);
    /// assert_eq!(scheme, result.scheme);
    /// assert_eq!(body, result.uniform_resource_identifier);
    ///
    /// let scheme = '\u{0001}';
    /// let body = "empty:uniform_resource_identifier";
    /// let uri = scheme.to_string() + body;
    /// let result = UniformResourceIdentifier::new(&uri);
    /// assert_eq!(uri.as_bytes().len() as u8 + 1, result.length);
    /// assert_eq!(scheme, result.scheme);
    /// assert_eq!(body, result.uniform_resource_identifier);
    /// ```
    pub fn new(uniform_resource_identifier: &String) -> Self {
        Self {
            length: uniform_resource_identifier.as_bytes().len() as u8 + 1,
            scheme: uniform_resource_identifier.chars().next().unwrap(),
            uniform_resource_identifier: uniform_resource_identifier.split_at(1).1.to_string(),
        }
    }
}

impl TryFrom<&Vec<u8>> for UniformResourceIdentifier {
    type Error = String;
    /// Create [`UniformResourceIdentifier`] from `Vec<u8>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{uniform_resource_identifier::UniformResourceIdentifier, data_type::DataType};
    ///
    /// let scheme = '\u{0016}';
    /// let body = "uniform_resource_identifier";
    /// let uri = scheme.to_string() + body;
    /// let length = uri.as_bytes().len() as u8 + 1;
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(UniformResourceIdentifier::data_type());
    /// data.append(&mut uri.to_string().into_bytes());
    ///
    /// let result = UniformResourceIdentifier::try_from(&data);
    /// assert!(result.is_ok());
    /// let data_type = result.unwrap();
    /// assert_eq!(length, data_type.length);
    /// assert_eq!(scheme, data_type.scheme);
    /// assert_eq!(body, data_type.uniform_resource_identifier);
    ///
    /// let scheme = '\u{0001}';
    /// let body = "empty:uniform_resource_identifier";
    /// let uri = scheme.to_string() + body;
    /// let length = uri.as_bytes().len() as u8 + 1;
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(UniformResourceIdentifier::data_type());
    /// data.append(&mut uri.to_string().into_bytes());
    ///
    /// let result = UniformResourceIdentifier::try_from(&data);
    /// assert!(result.is_ok());
    /// let data_type = result.unwrap();
    /// assert_eq!(length, data_type.length);
    /// assert_eq!(scheme, data_type.scheme);
    /// assert_eq!(body, data_type.uniform_resource_identifier);
    ///
    /// let data: Vec<u8> = Vec::new();
    /// let result = UniformResourceIdentifier::try_from(&data);
    /// assert!(result.is_err());
    /// assert_eq!(
    ///     format!("Invalid data size :{}", data.len()),
    ///     result.unwrap_err()
    /// );
    /// ```
    fn try_from(value: &Vec<u8>) -> Result<Self, String> {
        let len = value.len();
        if len < 4 {
            return Err(format!("Invalid data size :{}", len).to_string());
        }
        let length = value[0];
        let uniform_resource_identifier =
            String::from_utf8(value[2..1 + usize::from(length)].to_vec()).unwrap();
        Ok(Self {
            length,
            scheme: uniform_resource_identifier.chars().next().unwrap(),
            uniform_resource_identifier: uniform_resource_identifier.split_at(1).1.to_string(),
        })
    }
}

impl Into<Vec<u8>> for UniformResourceIdentifier {
    /// Create `Vec<u8>` from [`UniformResourceIdentifier`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{uniform_resource_identifier::UniformResourceIdentifier, data_type::DataType};
    ///
    /// let scheme = '\u{0016}';
    /// let body = "uniform_resource_identifier";
    /// let uri = scheme.to_string() + body;
    /// let length = uri.as_bytes().len() as u8 + 1;
    /// let result1 = UniformResourceIdentifier::new(&uri);
    ///
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(UniformResourceIdentifier::data_type());
    /// data.append(&mut uri.to_string().into_bytes());
    ///
    /// let into_data: Vec<u8> = result1.into();
    /// assert_eq!(data, into_data);
    ///
    /// let result2 = UniformResourceIdentifier::try_from(&data);
    /// assert!(result2.is_ok());
    /// let data_type = result2.unwrap();
    /// let into_data: Vec<u8> = data_type.into();
    /// assert_eq!(data, into_data);
    ///
    /// let scheme = '\u{0001}';
    /// let body = "empty:uniform_resource_identifier";
    /// let uri = scheme.to_string() + body;
    /// let length = uri.as_bytes().len() as u8 + 1;
    /// let result1 = UniformResourceIdentifier::new(&uri);
    ///
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(UniformResourceIdentifier::data_type());
    /// data.append(&mut uri.to_string().into_bytes());
    ///
    /// let into_data: Vec<u8> = result1.into();
    /// assert_eq!(data, into_data);
    ///
    /// let result2 = UniformResourceIdentifier::try_from(&data);
    /// assert!(result2.is_ok());
    /// let data_type = result2.unwrap();
    /// let into_data: Vec<u8> = data_type.into();
    /// assert_eq!(data, into_data);
    /// ```
    fn into(self) -> Vec<u8> {
        let mut data: Vec<u8> = Vec::new();
        data.push(self.length);
        data.push(Self::data_type());
        data.append(&mut String::from(self.scheme).into_bytes());
        data.append(&mut self.uniform_resource_identifier.clone().into_bytes());
        return data;
    }
}

impl DataType for UniformResourceIdentifier {
    /// return `0x24`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{uniform_resource_identifier::UniformResourceIdentifier, data_type::DataType};
    ///
    /// assert_eq!(0x24, UniformResourceIdentifier::data_type());
    /// ```
    fn data_type() -> u8 {
        0x24
    }
}

/// check `Uniform Resource Identifier` data type.
///
/// # Examples
///
/// ```
/// use ble_data_struct::data_types::uniform_resource_identifier::*;
/// use ble_data_struct::data_types::data_type::DataType;
///
/// assert!(is_uniform_resource_identifier(0x24));
/// assert!(!is_uniform_resource_identifier(0x00));
/// ```
pub fn is_uniform_resource_identifier(data_type: u8) -> bool {
    UniformResourceIdentifier::data_type() == data_type
}

#[cfg(test)]
mod tests {
    use crate::data_types::{data_type::DataType, uniform_resource_identifier::*};

    #[test]
    fn test_new() {
        let scheme = '\u{0016}';
        let body = "uniform_resource_identifier";
        let uri = scheme.to_string() + body;
        let result = UniformResourceIdentifier::new(&uri);
        assert_eq!(uri.as_bytes().len() as u8 + 1, result.length);
        assert_eq!(scheme, result.scheme);
        assert_eq!(body, result.uniform_resource_identifier);

        let scheme = '\u{0001}';
        let body = "empty:uniform_resource_identifier";
        let uri = scheme.to_string() + body;
        let result = UniformResourceIdentifier::new(&uri);
        assert_eq!(uri.as_bytes().len() as u8 + 1, result.length);
        assert_eq!(scheme, result.scheme);
        assert_eq!(body, result.uniform_resource_identifier);
    }

    #[test]
    fn test_try_from() {
        let scheme = '\u{0016}';
        let body = "uniform_resource_identifier";
        let uri = scheme.to_string() + body;
        let length = uri.as_bytes().len() as u8 + 1;
        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(UniformResourceIdentifier::data_type());
        data.append(&mut uri.to_string().into_bytes());

        let result = UniformResourceIdentifier::try_from(&data);
        assert!(result.is_ok());
        let data_type = result.unwrap();
        assert_eq!(length, data_type.length);
        assert_eq!(scheme, data_type.scheme);
        assert_eq!(body, data_type.uniform_resource_identifier);

        let scheme = '\u{0001}';
        let body = "empty:uniform_resource_identifier";
        let uri = scheme.to_string() + body;
        let length = uri.as_bytes().len() as u8 + 1;
        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(UniformResourceIdentifier::data_type());
        data.append(&mut uri.to_string().into_bytes());

        let result = UniformResourceIdentifier::try_from(&data);
        assert!(result.is_ok());
        let data_type = result.unwrap();
        assert_eq!(length, data_type.length);
        assert_eq!(scheme, data_type.scheme);
        assert_eq!(body, data_type.uniform_resource_identifier);

        let mut data: Vec<u8> = vec![0u8; 3];
        data[0] = data.len() as u8 - 1;
        let result = UniformResourceIdentifier::try_from(&data);
        assert!(result.is_err());
        assert_eq!(
            format!("Invalid data size :{}", data.len()),
            result.unwrap_err()
        );
    }

    #[test]
    fn test_into() {
        let scheme = '\u{0016}';
        let body = "uniform_resource_identifier";
        let uri = scheme.to_string() + body;
        let length = uri.as_bytes().len() as u8 + 1;
        let result1 = UniformResourceIdentifier::new(&uri);

        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(UniformResourceIdentifier::data_type());
        data.append(&mut uri.to_string().into_bytes());

        let into_data: Vec<u8> = result1.into();
        assert_eq!(data, into_data);

        let result2 = UniformResourceIdentifier::try_from(&data);
        assert!(result2.is_ok());
        let data_type = result2.unwrap();
        let into_data: Vec<u8> = data_type.into();
        assert_eq!(data, into_data);

        let scheme = '\u{0001}';
        let body = "empty:uniform_resource_identifier";
        let uri = scheme.to_string() + body;
        let length = uri.as_bytes().len() as u8 + 1;
        let result1 = UniformResourceIdentifier::new(&uri);

        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(UniformResourceIdentifier::data_type());
        data.append(&mut uri.to_string().into_bytes());

        let into_data: Vec<u8> = result1.into();
        assert_eq!(data, into_data);

        let result2 = UniformResourceIdentifier::try_from(&data);
        assert!(result2.is_ok());
        let data_type = result2.unwrap();
        let into_data: Vec<u8> = data_type.into();
        assert_eq!(data, into_data);
    }

    #[test]
    fn test_data_type() {
        assert_eq!(0x24, UniformResourceIdentifier::data_type());
    }

    #[test]
    fn test_is_uniform_resource_identifier() {
        assert!(is_uniform_resource_identifier(0x24));
        assert!(!is_uniform_resource_identifier(0x00));
    }
}

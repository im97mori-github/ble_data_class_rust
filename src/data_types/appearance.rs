//! Appearance (Data Type Value: 0x19) module.

use crate::data_types::data_type::DataType;

/// Appearance.
#[derive(Debug)]
pub struct Appearance {
    /// data length
    pub length: u8,

    /// Appearance
    pub appearance: u16,
}

impl Appearance {
    /// Create [`Appearance`] from `Appearance`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::appearance::Appearance;
    ///
    /// let appearance: u16 = 0x1444;
    /// let result = Appearance::new(appearance);
    /// assert_eq!(3, result.length);
    /// assert_eq!(appearance, result.appearance);
    /// ```
    pub fn new(appearance: u16) -> Self {
        Self {
            length: 3,
            appearance,
        }
    }

    /// Get Category.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{appearance::Appearance, data_type::DataType};
    ///
    /// let appearance: u16 = 0x1444;
    /// let length = 3;
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(Appearance::data_type());
    /// data.append(&mut appearance.to_le_bytes().to_vec());
    ///
    /// let result = Appearance::try_from(&data);
    /// assert_eq!(0x051, result.unwrap().category());
    /// ```
    pub const fn category(&self) -> u16 {
        (self.appearance >> 6) & 0b00000011_11111111
    }

    /// Get Subcatgeory.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{appearance::Appearance, data_type::DataType};
    ///
    /// let appearance: u16 = 0x1444;
    /// let length = 3;
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(Appearance::data_type());
    /// data.append(&mut appearance.to_le_bytes().to_vec());
    ///
    /// let result = Appearance::try_from(&data);
    /// assert_eq!(0x04, result.unwrap().sub_category());
    /// ```
    pub const fn sub_category(&self) -> u16 {
        self.appearance & 0b00111111
    }
}

impl TryFrom<&Vec<u8>> for Appearance {
    type Error = String;
    /// Create [`Appearance`] from `Vec<u8>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{appearance::Appearance, data_type::DataType};
    ///
    /// let appearance: u16 = 0x1444;
    /// let length = 3;
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(Appearance::data_type());
    /// data.append(&mut appearance.to_le_bytes().to_vec());
    /// let result = Appearance::try_from(&data);
    /// assert!(result.is_ok());
    /// let data_type = result.unwrap();
    /// assert_eq!(length, data_type.length);
    /// assert_eq!(appearance, data_type.appearance);
    ///
    /// let data: Vec<u8> = Vec::new();
    /// let result = Appearance::try_from(&data);
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
        Ok(Self {
            length,
            appearance: u16::from_le_bytes(value[2..4].try_into().unwrap()),
        })
    }
}

impl Into<Vec<u8>> for Appearance {
    /// Create `Vec<u8>` from [`Appearance`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{appearance::Appearance, data_type::DataType};
    ///
    /// let appearance: u16 = 0x1444;
    /// let result1 = Appearance::new(appearance);
    ///
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(3);
    /// data.push(Appearance::data_type());
    /// data.append(&mut appearance.to_le_bytes().to_vec());
    ///
    /// let into_data: Vec<u8> = result1.into();
    /// assert_eq!(data, into_data);
    ///
    /// let result2 = Appearance::try_from(&data);
    /// assert!(result2.is_ok());
    /// let data_type = result2.unwrap();
    /// let into_data: Vec<u8> = data_type.into();
    /// assert_eq!(data, into_data);
    /// ```
    fn into(self) -> Vec<u8> {
        let mut data: Vec<u8> = Vec::new();
        data.push(self.length);
        data.push(Self::data_type());
        data.append(&mut self.appearance.to_le_bytes().to_vec());
        return data;
    }
}

impl DataType for Appearance {
    /// return `0x19`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{appearance::Appearance, data_type::DataType};
    ///
    /// assert_eq!(0x19, Appearance::data_type());
    /// ```
    fn data_type() -> u8 {
        0x19
    }
}

/// check `Appearance` data type.
///
/// # Examples
///
/// ```
/// use ble_data_struct::data_types::appearance::*;
/// use ble_data_struct::data_types::data_type::DataType;
///
/// assert!(is_appearance(0x19));
/// assert!(!is_appearance(0x00));
/// ```
pub fn is_appearance(data_type: u8) -> bool {
    Appearance::data_type() == data_type
}

#[cfg(test)]
mod tests {
    use crate::data_types::{appearance::*, data_type::DataType};

    #[test]
    fn test_new() {
        let appearance: u16 = 0x1444;
        let result = Appearance::new(appearance);
        assert_eq!(3, result.length);
        assert_eq!(appearance, result.appearance);
    }

    #[test]
    fn test_category() {
        let appearance: u16 = 0x1444;
        let length = 3;
        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(Appearance::data_type());
        data.append(&mut appearance.to_le_bytes().to_vec());

        let result = Appearance::try_from(&data);
        assert_eq!(0x051, result.unwrap().category());
    }

    #[test]
    fn test_sub_category() {
        let appearance: u16 = 0x1444;
        let length = 3;
        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(Appearance::data_type());
        data.append(&mut appearance.to_le_bytes().to_vec());

        let result = Appearance::try_from(&data);
        assert_eq!(0x04, result.unwrap().sub_category());
    }

    #[test]
    fn test_try_from() {
        let appearance: u16 = 0x1444;
        let length = 3;
        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(Appearance::data_type());
        data.append(&mut appearance.to_le_bytes().to_vec());
        let result = Appearance::try_from(&data);
        assert!(result.is_ok());
        let data_type = result.unwrap();
        assert_eq!(length, data_type.length);
        assert_eq!(appearance, data_type.appearance);

        let data: Vec<u8> = Vec::new();
        let result = Appearance::try_from(&data);
        assert!(result.is_err());
        assert_eq!(
            format!("Invalid data size :{}", data.len()),
            result.unwrap_err()
        );
    }

    #[test]
    fn test_into() {
        let appearance: u16 = 0x1444;
        let result1 = Appearance::new(appearance);

        let mut data: Vec<u8> = Vec::new();
        data.push(3);
        data.push(Appearance::data_type());
        data.append(&mut appearance.to_le_bytes().to_vec());

        let into_data: Vec<u8> = result1.into();
        assert_eq!(data, into_data);

        let result2 = Appearance::try_from(&data);
        assert!(result2.is_ok());
        let data_type = result2.unwrap();
        let into_data: Vec<u8> = data_type.into();
        assert_eq!(data, into_data);
    }

    #[test]
    fn test_data_type() {
        assert_eq!(0x19, Appearance::data_type());
    }

    #[test]
    fn test_is_appearance() {
        assert!(is_appearance(0x19));
        assert!(!is_appearance(0x00));
    }
}

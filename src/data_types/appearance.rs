//! Appearance (Data Type Value: 0x19) module.

use crate::data_types::data_type::DataType;

/// Appearance.
pub struct Appearance {
    /// data length
    pub length: u8,

    /// Appearance
    pub appearance: u16,
}

impl Appearance {
    /// Create [Appearance] from `Appearance`.
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

    /// Create [Appearance] from `Vec<u8>` with offset.
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
    /// let result = Appearance::from_with_offset(&data, 0);
    /// assert_eq!(length, result.length);
    /// assert_eq!(appearance, result.appearance);
    ///
    /// data = Vec::new();
    /// data.push(0);
    /// data.push(length);
    /// data.push(Appearance::data_type());
    /// data.append(&mut appearance.to_le_bytes().to_vec());
    /// let result = Appearance::from_with_offset(&data, 1);
    /// assert_eq!(length, result.length);
    /// assert_eq!(appearance, result.appearance);
    /// ```
    pub fn from_with_offset(data: &Vec<u8>, offset: usize) -> Self {
        let data = data[offset..].to_vec();
        let length = data[0];
        Self {
            length,
            appearance: u16::from_le_bytes(data[2..4].try_into().unwrap()),
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
    /// let result = Appearance::from_with_offset(&data, 0);
    /// assert_eq!(0x051, result.category());
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
    /// let result = Appearance::from_with_offset(&data, 0);
    /// assert_eq!(0x04, result.sub_category());
    /// ```
    pub const fn sub_category(&self) -> u16 {
        self.appearance & 0b00111111
    }
}

impl From<&Vec<u8>> for Appearance {
    /// Create [Appearance] from `Vec<u8>`.
    ///
    /// [`Appearance::from_with_offset`]
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
    /// let result = Appearance::from(&data);
    /// assert_eq!(length, result.length);
    /// assert_eq!(appearance, result.appearance);
    /// ```
    fn from(data: &Vec<u8>) -> Self {
        Self::from_with_offset(data, 0)
    }
}

impl Into<Vec<u8>> for Appearance {
    /// Create `Vec<u8>` from [Appearance].
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
    /// let result2 = Appearance::from(&data);
    /// let into_data: Vec<u8> = result2.into();
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
    fn test_from_with_offset() {
        let appearance: u16 = 0x1444;
        let length = 3;
        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(Appearance::data_type());
        data.append(&mut appearance.to_le_bytes().to_vec());

        let result = Appearance::from_with_offset(&data, 0);
        assert_eq!(length, result.length);
        assert_eq!(appearance, result.appearance);

        data = Vec::new();
        data.push(0);
        data.push(length);
        data.push(Appearance::data_type());
        data.append(&mut appearance.to_le_bytes().to_vec());
        let result = Appearance::from_with_offset(&data, 1);
        assert_eq!(length, result.length);
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

        let result = Appearance::from_with_offset(&data, 0);
        assert_eq!(0x051, result.category());
    }

    #[test]
    fn test_sub_category() {
        let appearance: u16 = 0x1444;
        let length = 3;
        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(Appearance::data_type());
        data.append(&mut appearance.to_le_bytes().to_vec());

        let result = Appearance::from_with_offset(&data, 0);
        assert_eq!(0x04, result.sub_category());
    }

    #[test]
    fn test_from() {
        let appearance: u16 = 0x1444;
        let length = 3;
        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(Appearance::data_type());
        data.append(&mut appearance.to_le_bytes().to_vec());
        let result = Appearance::from(&data);
        assert_eq!(length, result.length);
        assert_eq!(appearance, result.appearance);
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

        let result2 = Appearance::from(&data);
        let into_data: Vec<u8> = result2.into();
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

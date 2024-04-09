//! Characteristic Presentation Format (Attribute Type: 0x2904) module for windows.
use crate::Uuid16bit;

/// Characteristic Presentation Format.
#[derive(Debug, PartialEq, Clone)]
pub struct CharacteristicPresentationFormat {
    /// Format
    pub format: u8,
    /// Exponent
    pub exponent: i8,
    /// Unit
    pub unit: u16,
    /// Name Space
    pub name_space: u8,
    /// Description
    pub description: u16,
}

impl CharacteristicPresentationFormat {
    /// Create [`CharacteristicPresentationFormat`] from `Format`, `Exponent`, `Unit`, `Exponent`, `Name Space`, `Description`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::descriptors::characteristic_presentation_format::{
    ///     CharacteristicPresentationFormat,
    /// };
    ///
    /// let format = 0x01u8;
    /// let exponent = 0x02i8;
    /// let unit = 0x0403u16;
    /// let name_space = 0x05u8;
    /// let description = 0x0706u16;
    ///
    /// let result =
    ///     CharacteristicPresentationFormat::new(format, exponent, unit, name_space, description);
    /// assert_eq!(format, result.format);
    /// assert_eq!(exponent, result.exponent);
    /// assert_eq!(unit, result.unit);
    /// assert_eq!(name_space, result.name_space);
    /// assert_eq!(description, result.description);
    /// ```
    pub fn new(format: u8, exponent: i8, unit: u16, name_space: u8, description: u16) -> Self {
        Self {
            format,
            exponent,
            unit,
            name_space,
            description,
        }
    }
}

impl TryFrom<&Vec<u8>> for CharacteristicPresentationFormat {
    type Error = String;
    /// Create [`CharacteristicPresentationFormat`] from [`Vec<u8>`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::descriptors::characteristic_presentation_format::{
    ///     CharacteristicPresentationFormat,
    /// };
    ///
    /// let format = 0x01u8;
    /// let exponent = 0x02i8;
    /// let unit = 0x0403u16;
    /// let name_space = 0x05u8;
    /// let description = 0x0706u16;
    ///
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(format);
    /// data.push(exponent as u8);
    /// data.append(&mut unit.to_le_bytes().to_vec());
    /// data.push(name_space);
    /// data.append(&mut description.to_le_bytes().to_vec());
    /// let result = CharacteristicPresentationFormat::try_from(&data);
    /// assert!(result.is_ok());
    /// let descriptor = result.unwrap();
    /// assert_eq!(format, descriptor.format);
    /// assert_eq!(exponent, descriptor.exponent);
    /// assert_eq!(unit, descriptor.unit);
    /// assert_eq!(name_space, descriptor.name_space);
    /// assert_eq!(description, descriptor.description);
    ///
    /// let data = Vec::new();
    /// let result = CharacteristicPresentationFormat::try_from(&data);
    /// assert!(!result.is_ok());
    /// ```
    fn try_from(value: &Vec<u8>) -> Result<Self, String> {
        let len = value.len();
        if len != 7 {
            return Err(format!("Invalid data size :{}", len).to_string());
        }
        Ok(Self {
            format: value[0],
            exponent: value[1] as i8,
            unit: u16::from_le_bytes(value[2..4].try_into().unwrap()),
            name_space: value[4],
            description: u16::from_le_bytes(value[5..7].try_into().unwrap()),
        })
    }
}

impl Into<Vec<u8>> for CharacteristicPresentationFormat {
    /// Create [`Vec<u8>`] from [`CharacteristicPresentationFormat`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::descriptors::characteristic_presentation_format::{
    ///     CharacteristicPresentationFormat,
    /// };
    ///
    /// let format = 0x01u8;
    /// let exponent = 0x02i8;
    /// let unit = 0x0403u16;
    /// let name_space = 0x05u8;
    /// let description = 0x0706u16;
    ///
    /// let result =
    ///     CharacteristicPresentationFormat::new(format, exponent, unit, name_space, description);
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(format);
    /// data.push(exponent as u8);
    /// data.append(&mut unit.to_le_bytes().to_vec());
    /// data.push(name_space);
    /// data.append(&mut description.to_le_bytes().to_vec());
    /// let into_data: Vec<u8> = result.into();
    /// assert_eq!(data, into_data);
    /// ```
    fn into(self) -> Vec<u8> {
        let mut data: Vec<u8> = Vec::new();
        data.push(self.format);
        data.push(self.exponent as u8);
        data.append(&mut self.unit.to_le_bytes().to_vec());
        data.push(self.name_space);
        data.append(&mut self.description.to_le_bytes().to_vec());
        return data;
    }
}

impl Uuid16bit for CharacteristicPresentationFormat {
    /// return `0x2904`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::Uuid16bit;
    /// use ble_data_struct::descriptors::characteristic_presentation_format::CharacteristicPresentationFormat;
    ///
    /// assert_eq!(0x2904, CharacteristicPresentationFormat::uuid_16bit());
    /// ```
    fn uuid_16bit() -> u16 {
        0x2904
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        descriptors::characteristic_presentation_format::CharacteristicPresentationFormat,
        Uuid16bit,
    };

    #[test]
    fn test_new() {
        let format = 0x01u8;
        let exponent = 0x02i8;
        let unit = 0x0403u16;
        let name_space = 0x05u8;
        let description = 0x0706u16;

        let result =
            CharacteristicPresentationFormat::new(format, exponent, unit, name_space, description);
        assert_eq!(format, result.format);
        assert_eq!(exponent, result.exponent);
        assert_eq!(unit, result.unit);
        assert_eq!(name_space, result.name_space);
        assert_eq!(description, result.description);
    }

    #[test]
    fn test_try_from() {
        let format = 0x01u8;
        let exponent = 0x02i8;
        let unit = 0x0403u16;
        let name_space = 0x05u8;
        let description = 0x0706u16;

        let mut data: Vec<u8> = Vec::new();
        data.push(format);
        data.push(exponent as u8);
        data.append(&mut unit.to_le_bytes().to_vec());
        data.push(name_space);
        data.append(&mut description.to_le_bytes().to_vec());
        let result = CharacteristicPresentationFormat::try_from(&data);
        assert!(result.is_ok());
        let descriptor = result.unwrap();
        assert_eq!(format, descriptor.format);
        assert_eq!(exponent, descriptor.exponent);
        assert_eq!(unit, descriptor.unit);
        assert_eq!(name_space, descriptor.name_space);
        assert_eq!(description, descriptor.description);

        let data = Vec::new();
        let result = CharacteristicPresentationFormat::try_from(&data);
        assert!(!result.is_ok());
    }

    #[test]
    fn test_into() {
        let format = 0x01u8;
        let exponent = 0x02i8;
        let unit = 0x0403u16;
        let name_space = 0x05u8;
        let description = 0x0706u16;

        let result =
            CharacteristicPresentationFormat::new(format, exponent, unit, name_space, description);
        let mut data: Vec<u8> = Vec::new();
        data.push(format);
        data.push(exponent as u8);
        data.append(&mut unit.to_le_bytes().to_vec());
        data.push(name_space);
        data.append(&mut description.to_le_bytes().to_vec());
        let into_data: Vec<u8> = result.into();
        assert_eq!(data, into_data);
    }

    #[test]
    fn test_uuid_16bit() {
        assert_eq!(0x2904, CharacteristicPresentationFormat::uuid_16bit());
    }
}

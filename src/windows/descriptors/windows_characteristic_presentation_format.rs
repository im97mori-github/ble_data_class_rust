//! Characteristic Presentation Format (Attribute Type: 0x2904) module for windows.

#[cfg(target_os = "windows")]
use windows::{
    Devices::Bluetooth::GenericAttributeProfile::GattPresentationFormat, Storage::Streams::IBuffer,
};

#[cfg(target_os = "windows")]
use crate::{
    descriptors::characteristic_presentation_format::CharacteristicPresentationFormat,
    windows::buffer::{i_buffer_to_vec, vec_to_i_buffer},
};

#[cfg(target_os = "windows")]
impl TryFrom<&GattPresentationFormat> for CharacteristicPresentationFormat {
    type Error = String;
    /// Create [`CharacteristicPresentationFormat`] from [`GattPresentationFormat`].
    ///
    /// # Examples
    ///
    /// ```
    /// use windows::{
    ///     Devices::Bluetooth::GenericAttributeProfile::GattPresentationFormat,
    ///     Storage::Streams::DataWriter,
    /// };
    ///
    /// use ble_data_struct::descriptors::characteristic_presentation_format::CharacteristicPresentationFormat;
    ///
    /// let format = 0x01u8;
    /// let exponent = 0x02i8;
    /// let unit = 0x0403u16;
    /// let name_space = 0x05u8;
    /// let description = 0x0706u16;
    ///
    /// let gatt_presentation_format = GattPresentationFormat::FromParts(
    ///     format,
    ///     exponent.try_into().unwrap(),
    ///     unit,
    ///     name_space,
    ///     description,
    /// )
    /// .unwrap();
    /// let result = CharacteristicPresentationFormat::try_from(&gatt_presentation_format);
    /// assert!(result.is_ok());
    /// let value = result.unwrap();
    /// assert_eq!(format, value.format);
    /// assert_eq!(exponent, value.exponent);
    /// assert_eq!(unit, value.unit);
    /// assert_eq!(name_space, value.name_space);
    /// assert_eq!(description, value.description);
    /// ```
    fn try_from(value: &GattPresentationFormat) -> Result<Self, String> {
        let format = match value.FormatType() {
            Ok(format) => format,
            Err(e) => return Err(e.message()),
        };
        let exponent: i8 = match value.Exponent() {
            Ok(exponent) => exponent.try_into().unwrap(),
            Err(e) => return Err(e.message()),
        };
        let unit = match value.Unit() {
            Ok(unit) => unit,
            Err(e) => return Err(e.message()),
        };
        let name_space = match value.Namespace() {
            Ok(name_space) => name_space,
            Err(e) => return Err(e.message()),
        };
        let description = match value.Description() {
            Ok(description) => description,
            Err(e) => return Err(e.message()),
        };
        Ok(Self {
            format,
            exponent,
            unit,
            name_space,
            description,
        })
    }
}

#[cfg(target_os = "windows")]
impl TryFrom<IBuffer> for CharacteristicPresentationFormat {
    type Error = String;
    /// Create [`CharacteristicPresentationFormat`] from [`IBuffer`].
    ///
    /// # Examples
    ///
    /// ```
    /// use windows::{
    ///     Devices::Bluetooth::GenericAttributeProfile::GattPresentationFormat,
    ///     Storage::Streams::DataWriter,
    /// };
    ///
    /// use ble_data_struct::descriptors::characteristic_presentation_format::CharacteristicPresentationFormat;
    ///
    /// let format = 0x01u8;
    /// let exponent = 0x02i8;
    /// let unit = 0x0403u16;
    /// let name_space = 0x05u8;
    /// let description = 0x0706u16;
    ///
    /// let characteristic_presentation_format =
    ///     CharacteristicPresentationFormat::new(format, exponent, unit, name_space, description);
    /// let data_writer = DataWriter::new().unwrap();
    /// let ble_packet: Vec<u8> = characteristic_presentation_format.into();
    /// data_writer.WriteBytes(&ble_packet).unwrap();
    /// let buffer = data_writer.DetachBuffer().unwrap();
    ///
    /// let result = CharacteristicPresentationFormat::try_from(buffer);
    /// assert!(result.is_ok());
    /// let value = result.unwrap();
    /// assert_eq!(format, value.format);
    /// assert_eq!(exponent, value.exponent);
    /// assert_eq!(unit, value.unit);
    /// assert_eq!(name_space, value.name_space);
    /// assert_eq!(description, value.description);
    /// ```
    fn try_from(value: IBuffer) -> Result<Self, String> {
        let vec = i_buffer_to_vec(value).unwrap();
        Self::try_from(&vec)
    }
}

#[cfg(target_os = "windows")]
impl Into<GattPresentationFormat> for CharacteristicPresentationFormat {
    /// Create [`GattPresentationFormat`] from [`CharacteristicPresentationFormat`].
    ///
    /// # Examples
    ///
    /// ```
    /// use windows::{
    ///     Devices::Bluetooth::GenericAttributeProfile::GattPresentationFormat,
    ///     Storage::Streams::DataWriter,
    /// };
    ///
    /// use ble_data_struct::descriptors::characteristic_presentation_format::CharacteristicPresentationFormat;
    ///
    /// let format = 0x01u8;
    /// let exponent = 0x02i8;
    /// let unit = 0x0403u16;
    /// let name_space = 0x05u8;
    /// let description = 0x0706u16;
    /// 
    /// let value: GattPresentationFormat =
    ///     CharacteristicPresentationFormat::new(format, exponent, unit, name_space, description)
    ///         .into();
    /// 
    /// assert_eq!(format, value.FormatType().unwrap());
    /// assert_eq!(exponent, value.Exponent().unwrap().try_into().unwrap());
    /// assert_eq!(unit, value.Unit().unwrap());
    /// assert_eq!(name_space, value.Namespace().unwrap());
    /// assert_eq!(description, value.Description().unwrap());
    /// ```
    fn into(self) -> GattPresentationFormat {
        GattPresentationFormat::FromParts(
            self.format,
            self.exponent.try_into().unwrap(),
            self.unit,
            self.name_space,
            self.description,
        )
        .unwrap()
    }
}

#[cfg(target_os = "windows")]
impl Into<IBuffer> for CharacteristicPresentationFormat {
    /// Create [`IBuffer`] from [`CharacteristicPresentationFormat`].
    ///
    /// # Examples
    ///
    /// ```
    /// use windows::{
    ///     Devices::Bluetooth::GenericAttributeProfile::GattPresentationFormat,
    ///     Storage::Streams::{DataWriter, IBuffer},
    /// };
    ///
    /// use ble_data_struct::{
    ///     descriptors::characteristic_presentation_format::CharacteristicPresentationFormat,
    ///     windows::buffer::i_buffer_to_vec,
    /// };
    ///
    /// let format = 0x01u8;
    /// let exponent = 0x02i8;
    /// let unit = 0x0403u16;
    /// let name_space = 0x05u8;
    /// let description = 0x0706u16;
    /// 
    /// let value =
    ///     CharacteristicPresentationFormat::new(format, exponent, unit, name_space, description);
    /// let buffer: IBuffer = value.clone().into();
    /// let vec: Vec<u8> = value.into();
    /// assert_eq!(vec, i_buffer_to_vec(buffer).unwrap());
    /// ```
    fn into(self) -> IBuffer {
        let vec: Vec<u8> = self.into();
        vec_to_i_buffer(&vec).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use windows::{
        Devices::Bluetooth::GenericAttributeProfile::GattPresentationFormat,
        Storage::Streams::{DataWriter, IBuffer},
    };

    use crate::{
        descriptors::characteristic_presentation_format::CharacteristicPresentationFormat,
        windows::buffer::i_buffer_to_vec,
    };

    #[test]
    fn test_try_from_gatt_presentation_format() {
        let format = 0x01u8;
        let exponent = 0x02i8;
        let unit = 0x0403u16;
        let name_space = 0x05u8;
        let description = 0x0706u16;

        let gatt_presentation_format = GattPresentationFormat::FromParts(
            format,
            exponent.try_into().unwrap(),
            unit,
            name_space,
            description,
        )
        .unwrap();
        let result = CharacteristicPresentationFormat::try_from(&gatt_presentation_format);
        assert!(result.is_ok());
        let value = result.unwrap();
        assert_eq!(format, value.format);
        assert_eq!(exponent, value.exponent);
        assert_eq!(unit, value.unit);
        assert_eq!(name_space, value.name_space);
        assert_eq!(description, value.description);
    }

    #[test]
    fn test_try_from_i_buffer() {
        let format = 0x01u8;
        let exponent = 0x02i8;
        let unit = 0x0403u16;
        let name_space = 0x05u8;
        let description = 0x0706u16;

        let characteristic_presentation_format =
            CharacteristicPresentationFormat::new(format, exponent, unit, name_space, description);
        let data_writer = DataWriter::new().unwrap();
        let ble_packet: Vec<u8> = characteristic_presentation_format.into();
        data_writer.WriteBytes(&ble_packet).unwrap();
        let buffer = data_writer.DetachBuffer().unwrap();

        let result = CharacteristicPresentationFormat::try_from(buffer);
        assert!(result.is_ok());
        let value = result.unwrap();
        assert_eq!(format, value.format);
        assert_eq!(exponent, value.exponent);
        assert_eq!(unit, value.unit);
        assert_eq!(name_space, value.name_space);
        assert_eq!(description, value.description);
    }

    #[test]
    fn test_into_gatt_presentation_format() {
        let format = 0x01u8;
        let exponent = 0x02i8;
        let unit = 0x0403u16;
        let name_space = 0x05u8;
        let description = 0x0706u16;

        let value: GattPresentationFormat =
            CharacteristicPresentationFormat::new(format, exponent, unit, name_space, description)
                .into();

        assert_eq!(format, value.FormatType().unwrap());
        assert_eq!(exponent, value.Exponent().unwrap().try_into().unwrap());
        assert_eq!(unit, value.Unit().unwrap());
        assert_eq!(name_space, value.Namespace().unwrap());
        assert_eq!(description, value.Description().unwrap());
    }

    #[test]
    fn test_into_i_buffer() {
        let format = 0x01u8;
        let exponent = 0x02i8;
        let unit = 0x0403u16;
        let name_space = 0x05u8;
        let description = 0x0706u16;

        let value =
            CharacteristicPresentationFormat::new(format, exponent, unit, name_space, description);
        let buffer: IBuffer = value.clone().into();
        let vec: Vec<u8> = value.into();
        assert_eq!(vec, i_buffer_to_vec(buffer).unwrap());
    }
}

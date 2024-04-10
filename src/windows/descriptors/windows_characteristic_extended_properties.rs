//! Characteristic Extended Properties (Attribute Type: 0x2900) module for windows.

#[cfg(target_os = "windows")]
use windows::Storage::Streams::IBuffer;

#[cfg(target_os = "windows")]
use crate::{
    descriptors::characteristic_extended_properties::CharacteristicExtendedProperties,
    windows::buffer::{i_buffer_to_vec, vec_to_i_buffer},
};

#[cfg(target_os = "windows")]
impl TryFrom<IBuffer> for CharacteristicExtendedProperties {
    type Error = String;
    /// Create [`CharacteristicExtendedProperties`] from [`IBuffer`].
    ///
    /// # Examples
    ///
    /// ```
    /// use windows::Storage::Streams::{DataWriter, IBuffer};
    ///
    /// use ble_data_struct::descriptors::characteristic_extended_properties::{
    ///     CharacteristicExtendedProperties, RELIABLE_WRITE, WRITABLE_AUXILIARIES,
    /// };
    ///
    /// let client_characteristic_configuration =
    ///     CharacteristicExtendedProperties::new(RELIABLE_WRITE);
    /// let data_writer = DataWriter::new().unwrap();
    /// let ble_packet: Vec<u8> = client_characteristic_configuration.into();
    /// data_writer.WriteBytes(&ble_packet).unwrap();
    /// let buffer = data_writer.DetachBuffer().unwrap();
    ///
    /// let result = CharacteristicExtendedProperties::try_from(buffer);
    /// assert!(result.is_ok());
    /// let value = result.unwrap();
    /// assert!(value.is_reliable_write());
    /// ```
    fn try_from(value: IBuffer) -> Result<Self, String> {
        let vec = i_buffer_to_vec(value).unwrap();
        let properties = u16::from_le_bytes(vec.try_into().unwrap());
        Ok(Self { properties })
    }
}

#[cfg(target_os = "windows")]
impl Into<IBuffer> for CharacteristicExtendedProperties {
    /// Create [`IBuffer`] from [`CharacteristicExtendedProperties`].
    ///
    /// # Examples
    ///
    /// ```
    /// use windows::{
    ///     Storage::Streams::{DataWriter, IBuffer},
    /// };
    ///
    /// use ble_data_struct::{
    ///     descriptors::characteristic_extended_properties::{
    ///         CharacteristicExtendedProperties, RELIABLE_WRITE, WRITABLE_AUXILIARIES,
    ///     },
    ///     windows::buffer::i_buffer_to_vec,
    /// };
    ///
    /// let value = CharacteristicExtendedProperties::new(0);
    /// let buffer: IBuffer = value.clone().into();
    /// let vec: Vec<u8> = value.into();
    /// assert_eq!(vec, i_buffer_to_vec(buffer).unwrap());
    ///
    /// let value = CharacteristicExtendedProperties::new(RELIABLE_WRITE);
    /// let buffer: IBuffer = value.clone().into();
    /// let vec: Vec<u8> = value.into();
    /// assert_eq!(vec, i_buffer_to_vec(buffer).unwrap());
    ///
    /// let value = CharacteristicExtendedProperties::new(WRITABLE_AUXILIARIES);
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
    use windows::Storage::Streams::{DataWriter, IBuffer};

    use crate::{
        descriptors::characteristic_extended_properties::{
            CharacteristicExtendedProperties, RELIABLE_WRITE, WRITABLE_AUXILIARIES,
        },
        windows::buffer::i_buffer_to_vec,
    };

    #[test]
    fn test_try_from_i_buffer() {
        let client_characteristic_configuration =
            CharacteristicExtendedProperties::new(RELIABLE_WRITE);
        let data_writer = DataWriter::new().unwrap();
        let ble_packet: Vec<u8> = client_characteristic_configuration.into();
        data_writer.WriteBytes(&ble_packet).unwrap();
        let buffer = data_writer.DetachBuffer().unwrap();

        let result = CharacteristicExtendedProperties::try_from(buffer);
        assert!(result.is_ok());
        let value = result.unwrap();
        assert!(value.is_reliable_write());
    }

    #[test]
    fn test_into_i_buffer() {
        let value = CharacteristicExtendedProperties::new(0);
        let buffer: IBuffer = value.clone().into();
        let vec: Vec<u8> = value.into();
        assert_eq!(vec, i_buffer_to_vec(buffer).unwrap());

        let value = CharacteristicExtendedProperties::new(RELIABLE_WRITE);
        let buffer: IBuffer = value.clone().into();
        let vec: Vec<u8> = value.into();
        assert_eq!(vec, i_buffer_to_vec(buffer).unwrap());

        let value = CharacteristicExtendedProperties::new(WRITABLE_AUXILIARIES);
        let buffer: IBuffer = value.clone().into();
        let vec: Vec<u8> = value.into();
        assert_eq!(vec, i_buffer_to_vec(buffer).unwrap());
    }
}

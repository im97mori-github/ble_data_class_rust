//! Characteristic User Description (Attribute Type: 0x2901) module for windows.
#[cfg(target_os = "windows")]
use crate::descriptors::characteristic_user_description::CharacteristicUserDescription;
#[cfg(target_os = "windows")]
use crate::windows::buffer::{i_buffer_to_vec, vec_to_i_buffer};

#[cfg(target_os = "windows")]
use windows::Storage::Streams::IBuffer;

#[cfg(target_os = "windows")]
impl TryFrom<IBuffer> for CharacteristicUserDescription {
    type Error = String;
    /// Create [`CharacteristicUserDescription`] from [`IBuffer`].
    ///
    /// # Examples
    ///
    /// ```
    /// use windows::Storage::Streams::{DataWriter, IBuffer};
    ///
    /// use ble_data_struct::descriptors::characteristic_user_description::CharacteristicUserDescription;
    ///
    /// let description = "description".to_string();
    /// let data_writer = DataWriter::new().unwrap();
    /// let ble_packet: Vec<u8> = description.to_string().into();
    /// data_writer.WriteBytes(&ble_packet).unwrap();
    /// let buffer = data_writer.DetachBuffer().unwrap();
    ///
    /// let result = CharacteristicUserDescription::try_from(buffer);
    /// assert!(result.is_ok());
    /// let value = result.unwrap();
    /// assert_eq!(description, value.description);
    /// ```
    fn try_from(value: IBuffer) -> Result<Self, String> {
        let vec = i_buffer_to_vec(value).unwrap();
        let description = String::from_utf8(vec).unwrap();
        Ok(Self { description })
    }
}

#[cfg(target_os = "windows")]
impl Into<IBuffer> for CharacteristicUserDescription {
    /// Create [`IBuffer`] from [`CharacteristicUserDescription`].
    ///
    /// # Examples
    ///
    /// ```
    /// use windows::Storage::Streams::{DataWriter, IBuffer};
    ///
    /// use ble_data_struct::{
    ///     descriptors::characteristic_user_description::CharacteristicUserDescription,
    ///     windows::buffer::i_buffer_to_vec
    /// };
    ///
    /// let description = "description".to_string();
    /// let value = CharacteristicUserDescription::new(description.to_string());
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
        descriptors::characteristic_user_description::CharacteristicUserDescription,
        windows::buffer::i_buffer_to_vec,
    };

    #[test]
    fn test_try_from_i_buffer() {
        let description = "description".to_string();
        let data_writer = DataWriter::new().unwrap();
        let ble_packet: Vec<u8> = description.to_string().into();
        data_writer.WriteBytes(&ble_packet).unwrap();
        let buffer = data_writer.DetachBuffer().unwrap();

        let result = CharacteristicUserDescription::try_from(buffer);
        assert!(result.is_ok());
        let value = result.unwrap();
        assert_eq!(description, value.description);
    }

    #[test]
    fn test_into_i_buffer() {
        let description = "description".to_string();
        let value = CharacteristicUserDescription::new(description.to_string());
        let buffer: IBuffer = value.clone().into();
        let vec: Vec<u8> = value.into();
        assert_eq!(vec, i_buffer_to_vec(buffer).unwrap());
    }
}

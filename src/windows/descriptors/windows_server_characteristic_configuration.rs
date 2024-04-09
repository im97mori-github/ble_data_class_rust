//! Server Characteristic Configuration (Attribute Type: 0x2903) module for windows.
#[cfg(target_os = "windows")]
use crate::descriptors::server_characteristic_configuration::ServerCharacteristicConfiguration;
#[cfg(target_os = "windows")]
use crate::windows::buffer::{i_buffer_to_vec, vec_to_i_buffer};

#[cfg(target_os = "windows")]
use windows::Storage::Streams::IBuffer;

#[cfg(target_os = "windows")]
impl TryFrom<IBuffer> for ServerCharacteristicConfiguration {
    type Error = String;
    /// Create [`ServerCharacteristicConfiguration`] from [`IBuffer`].
    ///
    /// # Examples
    ///
    /// ```
    /// use windows::Storage::Streams::{DataWriter, IBuffer};
    ///
    /// use ble_data_struct::descriptors::server_characteristic_configuration::{
    ///     ServerCharacteristicConfiguration, BROADCAST,
    /// };
    ///
    /// let client_characteristic_configuration = ServerCharacteristicConfiguration::new(BROADCAST);
    /// let data_writer = DataWriter::new().unwrap();
    /// let ble_packet: Vec<u8> = client_characteristic_configuration.into();
    /// data_writer.WriteBytes(&ble_packet).unwrap();
    /// let buffer = data_writer.DetachBuffer().unwrap();
    ///
    /// let result = ServerCharacteristicConfiguration::try_from(buffer);
    /// assert!(result.is_ok());
    /// let value = result.unwrap();
    /// assert!(value.is_broadcast());
    /// ```
    fn try_from(value: IBuffer) -> Result<Self, String> {
        let vec = i_buffer_to_vec(value).unwrap();
        let configuration = u16::from_le_bytes(vec.try_into().unwrap());
        Ok(Self {
            configuration: configuration,
        })
    }
}

#[cfg(target_os = "windows")]
impl Into<IBuffer> for ServerCharacteristicConfiguration {
    /// Create [`IBuffer`] from [`ServerCharacteristicConfiguration`].
    ///
    /// # Examples
    ///
    /// ```
    /// use windows::Storage::Streams::{DataWriter, IBuffer};
    ///
    /// use ble_data_struct::{
    ///     descriptors::server_characteristic_configuration::{
    ///         ServerCharacteristicConfiguration, BROADCAST,
    ///     },
    ///     windows::buffer::i_buffer_to_vec,
    /// };
    ///
    /// let value = ServerCharacteristicConfiguration::new(0);
    /// let buffer: IBuffer = value.clone().into();
    /// let vec: Vec<u8> = value.into();
    /// assert_eq!(vec, i_buffer_to_vec(buffer).unwrap());
    /// 
    /// let value = ServerCharacteristicConfiguration::new(BROADCAST);
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
        descriptors::server_characteristic_configuration::{
            ServerCharacteristicConfiguration, BROADCAST,
        },
        windows::buffer::i_buffer_to_vec,
    };

    #[test]
    fn test_try_from_i_buffer() {
        let client_characteristic_configuration = ServerCharacteristicConfiguration::new(BROADCAST);
        let data_writer = DataWriter::new().unwrap();
        let ble_packet: Vec<u8> = client_characteristic_configuration.into();
        data_writer.WriteBytes(&ble_packet).unwrap();
        let buffer = data_writer.DetachBuffer().unwrap();

        let result = ServerCharacteristicConfiguration::try_from(buffer);
        assert!(result.is_ok());
        let value = result.unwrap();
        assert!(value.is_broadcast());
    }

    #[test]
    fn test_into_i_buffer() {
        let value = ServerCharacteristicConfiguration::new(0);
        let buffer: IBuffer = value.clone().into();
        let vec: Vec<u8> = value.into();
        assert_eq!(vec, i_buffer_to_vec(buffer).unwrap());

        let value = ServerCharacteristicConfiguration::new(BROADCAST);
        let buffer: IBuffer = value.clone().into();
        let vec: Vec<u8> = value.into();
        assert_eq!(vec, i_buffer_to_vec(buffer).unwrap());
    }
}

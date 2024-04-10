//! Client Characteristic Configuration (Attribute Type: 0x2902) module for windows.

#[cfg(target_os = "windows")]
use windows::{
    Devices::Bluetooth::GenericAttributeProfile::GattClientCharacteristicConfigurationDescriptorValue,
    Storage::Streams::IBuffer,
};

#[cfg(target_os = "windows")]
use crate::{
    descriptors::client_characteristic_configuration::ClientCharacteristicConfiguration,
    windows::buffer::{i_buffer_to_vec, vec_to_i_buffer},
};

#[cfg(target_os = "windows")]
impl TryFrom<&GattClientCharacteristicConfigurationDescriptorValue>
    for ClientCharacteristicConfiguration
{
    type Error = String;
    /// Create [`ClientCharacteristicConfiguration`] from [`GattClientCharacteristicConfigurationDescriptorValue`].
    ///
    /// # Examples
    ///
    /// ```
    /// use windows::{
    ///     Devices::Bluetooth::GenericAttributeProfile::GattClientCharacteristicConfigurationDescriptorValue,
    ///     Storage::Streams::DataWriter,
    /// };
    ///
    /// use ble_data_struct::descriptors::client_characteristic_configuration::{
    ///     ClientCharacteristicConfiguration, INDICATION, NOTIFICATION,
    /// };
    ///
    /// let result = ClientCharacteristicConfiguration::try_from(
    ///     &GattClientCharacteristicConfigurationDescriptorValue::None,
    /// );
    /// assert!(result.is_ok());
    /// let value = result.unwrap();
    /// assert!(!value.is_notification());
    /// assert!(!value.is_indication());
    ///
    /// let result = ClientCharacteristicConfiguration::try_from(
    ///     &GattClientCharacteristicConfigurationDescriptorValue::Notify,
    /// );
    /// assert!(result.is_ok());
    /// let value = result.unwrap();
    /// assert!(value.is_notification());
    /// assert!(!value.is_indication());
    ///
    /// let result = ClientCharacteristicConfiguration::try_from(
    ///     &GattClientCharacteristicConfigurationDescriptorValue::Indicate,
    /// );
    /// assert!(result.is_ok());
    /// let value = result.unwrap();
    /// assert!(!value.is_notification());
    /// assert!(value.is_indication());
    /// ```
    fn try_from(
        value: &GattClientCharacteristicConfigurationDescriptorValue,
    ) -> Result<Self, String> {
        Ok(Self {
            configuration: u16::from_le_bytes(value.0.to_le_bytes()[0..2].try_into().unwrap()),
        })
    }
}

#[cfg(target_os = "windows")]
impl TryFrom<IBuffer> for ClientCharacteristicConfiguration {
    type Error = String;
    /// Create [`ClientCharacteristicConfiguration`] from [`IBuffer`].
    ///
    /// # Examples
    ///
    /// ```
    /// use windows::Storage::Streams::{DataWriter, IBuffer};
    ///
    /// use ble_data_struct::descriptors::client_characteristic_configuration::{
    ///     ClientCharacteristicConfiguration, INDICATION, NOTIFICATION,
    /// };
    ///
    /// let client_characteristic_configuration =
    ///     ClientCharacteristicConfiguration::new(NOTIFICATION);
    /// let data_writer = DataWriter::new().unwrap();
    /// let ble_packet: Vec<u8> = client_characteristic_configuration.into();
    /// data_writer.WriteBytes(&ble_packet).unwrap();
    /// let buffer = data_writer.DetachBuffer().unwrap();

    /// let result = ClientCharacteristicConfiguration::try_from(buffer);
    /// assert!(result.is_ok());
    /// let value = result.unwrap();
    /// assert!(value.is_notification());
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
impl Into<GattClientCharacteristicConfigurationDescriptorValue>
    for ClientCharacteristicConfiguration
{
    /// Create [`GattClientCharacteristicConfigurationDescriptorValue`] from [`ClientCharacteristicConfiguration`].
    ///
    /// # Examples
    ///
    /// ```
    /// use windows::{
    ///     Devices::Bluetooth::GenericAttributeProfile::GattClientCharacteristicConfigurationDescriptorValue,
    ///     Storage::Streams::DataWriter,
    /// };
    ///
    /// use ble_data_struct::descriptors::client_characteristic_configuration::{
    ///     ClientCharacteristicConfiguration, INDICATION, NOTIFICATION,
    /// };
    ///
    /// let value: GattClientCharacteristicConfigurationDescriptorValue =
    ///     ClientCharacteristicConfiguration::new(0).into();
    /// assert_eq!(
    ///     GattClientCharacteristicConfigurationDescriptorValue::None,
    ///     value
    /// );
    ///
    /// let value: GattClientCharacteristicConfigurationDescriptorValue =
    ///     ClientCharacteristicConfiguration::new(NOTIFICATION).into();
    /// assert_eq!(
    ///     GattClientCharacteristicConfigurationDescriptorValue::Notify,
    ///     value
    /// );
    ///
    /// let value: GattClientCharacteristicConfigurationDescriptorValue =
    ///     ClientCharacteristicConfiguration::new(INDICATION).into();
    /// assert_eq!(
    ///     GattClientCharacteristicConfigurationDescriptorValue::Indicate,
    ///     value
    /// );
    /// ```
    fn into(self) -> GattClientCharacteristicConfigurationDescriptorValue {
        if self.is_notification() {
            GattClientCharacteristicConfigurationDescriptorValue::Notify
        } else if self.is_indication() {
            GattClientCharacteristicConfigurationDescriptorValue::Indicate
        } else {
            GattClientCharacteristicConfigurationDescriptorValue::None
        }
    }
}

#[cfg(target_os = "windows")]
impl Into<IBuffer> for ClientCharacteristicConfiguration {
    /// Create [`IBuffer`] from [`ClientCharacteristicConfiguration`].
    ///
    /// # Examples
    ///
    /// ```
    /// use windows::{
    ///     Devices::Bluetooth::GenericAttributeProfile::GattClientCharacteristicConfigurationDescriptorValue,
    ///     Storage::Streams::{DataWriter, IBuffer},
    /// };
    ///
    /// use ble_data_struct::{
    ///     descriptors::client_characteristic_configuration::{
    ///         ClientCharacteristicConfiguration, INDICATION, NOTIFICATION,
    ///     },
    ///     windows::buffer::i_buffer_to_vec,
    /// };
    ///
    /// let value = ClientCharacteristicConfiguration::new(0);
    /// let buffer: IBuffer = value.clone().into();
    /// let vec: Vec<u8> = value.into();
    /// assert_eq!(vec, i_buffer_to_vec(buffer).unwrap());
    ///
    /// let value = ClientCharacteristicConfiguration::new(NOTIFICATION);
    /// let buffer: IBuffer = value.clone().into();
    /// let vec: Vec<u8> = value.into();
    /// assert_eq!(vec, i_buffer_to_vec(buffer).unwrap());
    ///
    /// let value = ClientCharacteristicConfiguration::new(INDICATION);
    /// let buffer: IBuffer = value.clone().into();
    /// let vec: Vec<u8> = value.into();
    /// assert_eq!(vec, i_buffer_to_vec(buffer).unwrap());
    /// ```
    fn into(self) -> IBuffer {
        let vec: Vec<u8> = self.into();
        vec_to_i_buffer(&vec).unwrap()
    }
}

#[cfg(target_os = "windows")]
#[cfg(test)]
mod tests {
    use windows::{
        Devices::Bluetooth::GenericAttributeProfile::GattClientCharacteristicConfigurationDescriptorValue,
        Storage::Streams::{DataWriter, IBuffer},
    };

    use crate::{
        descriptors::client_characteristic_configuration::{
            ClientCharacteristicConfiguration, INDICATION, NOTIFICATION,
        },
        windows::buffer::i_buffer_to_vec,
    };

    #[test]
    fn test_try_from_gatt_client_characteristic_configuration_descriptor_value() {
        let result = ClientCharacteristicConfiguration::try_from(
            &GattClientCharacteristicConfigurationDescriptorValue::None,
        );
        assert!(result.is_ok());
        let value = result.unwrap();
        assert!(!value.is_notification());
        assert!(!value.is_indication());

        let result = ClientCharacteristicConfiguration::try_from(
            &GattClientCharacteristicConfigurationDescriptorValue::Notify,
        );
        assert!(result.is_ok());
        let value = result.unwrap();
        assert!(value.is_notification());
        assert!(!value.is_indication());

        let result = ClientCharacteristicConfiguration::try_from(
            &GattClientCharacteristicConfigurationDescriptorValue::Indicate,
        );
        assert!(result.is_ok());
        let value = result.unwrap();
        assert!(!value.is_notification());
        assert!(value.is_indication());
    }

    #[test]
    fn test_try_from_i_buffer() {
        let client_characteristic_configuration =
            ClientCharacteristicConfiguration::new(NOTIFICATION);
        let data_writer = DataWriter::new().unwrap();
        let ble_packet: Vec<u8> = client_characteristic_configuration.into();
        data_writer.WriteBytes(&ble_packet).unwrap();
        let buffer = data_writer.DetachBuffer().unwrap();

        let result = ClientCharacteristicConfiguration::try_from(buffer);
        assert!(result.is_ok());
        let value = result.unwrap();
        assert!(value.is_notification());
    }

    #[test]
    fn test_into_gatt_client_characteristic_configuration_descriptor_value() {
        let value: GattClientCharacteristicConfigurationDescriptorValue =
            ClientCharacteristicConfiguration::new(0).into();
        assert_eq!(
            GattClientCharacteristicConfigurationDescriptorValue::None,
            value
        );

        let value: GattClientCharacteristicConfigurationDescriptorValue =
            ClientCharacteristicConfiguration::new(NOTIFICATION).into();
        assert_eq!(
            GattClientCharacteristicConfigurationDescriptorValue::Notify,
            value
        );

        let value: GattClientCharacteristicConfigurationDescriptorValue =
            ClientCharacteristicConfiguration::new(INDICATION).into();
        assert_eq!(
            GattClientCharacteristicConfigurationDescriptorValue::Indicate,
            value
        );
    }

    #[test]
    fn test_into_i_buffer() {
        let value = ClientCharacteristicConfiguration::new(0);
        let buffer: IBuffer = value.clone().into();
        let vec: Vec<u8> = value.into();
        assert_eq!(vec, i_buffer_to_vec(buffer).unwrap());

        let value = ClientCharacteristicConfiguration::new(NOTIFICATION);
        let buffer: IBuffer = value.clone().into();
        let vec: Vec<u8> = value.into();
        assert_eq!(vec, i_buffer_to_vec(buffer).unwrap());

        let value = ClientCharacteristicConfiguration::new(INDICATION);
        let buffer: IBuffer = value.clone().into();
        let vec: Vec<u8> = value.into();
        assert_eq!(vec, i_buffer_to_vec(buffer).unwrap());
    }
}

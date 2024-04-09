//! Server Characteristic Configuration (Attribute Type: 0x2903) module for windows.

use crate::Uuid16bit;

/// Server Characteristic Configuration.
#[derive(Debug, PartialEq, Clone)]
pub struct ServerCharacteristicConfiguration {
    /// Characteristic Configuration Bits
    pub configuration: u16,
}

impl ServerCharacteristicConfiguration {
    /// Create [`ServerCharacteristicConfiguration`] from `Characteristic Configuration Bit`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::descriptors::server_characteristic_configuration::{
    ///     ServerCharacteristicConfiguration, BROADCAST,
    /// };
    ///
    /// let result = ServerCharacteristicConfiguration::new(BROADCAST);
    /// assert_eq!(BROADCAST, result.configuration);
    /// ```
    pub fn new(configuration: u16) -> Self {
        Self { configuration }
    }

    /// check Notification configuration.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::descriptors::server_characteristic_configuration::{
    ///     ServerCharacteristicConfiguration, BROADCAST,
    /// };
    ///
    /// let result = ServerCharacteristicConfiguration::new(BROADCAST);
    /// assert!(result.is_broadcast());
    /// ```
    pub fn is_broadcast(&self) -> bool {
        self.configuration == BROADCAST
    }
}

/// Broadcast
pub const BROADCAST: u16 = 0b00000001;

impl TryFrom<&Vec<u8>> for ServerCharacteristicConfiguration {
    type Error = String;
    /// Create [`ServerCharacteristicConfiguration`] from [`Vec<u8>`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::descriptors::server_characteristic_configuration::{
    ///     ServerCharacteristicConfiguration, BROADCAST,
    /// };
    ///
    /// let configuration = BROADCAST.to_le_bytes().to_vec();
    /// let result = ServerCharacteristicConfiguration::try_from(&configuration);
    /// assert!(result.is_ok());
    /// assert_eq!(BROADCAST, result.unwrap().configuration);
    /// 
    /// let configuration = Vec::new();
    /// let result = ServerCharacteristicConfiguration::try_from(&configuration);
    /// assert!(!result.is_ok());
    /// ```
    fn try_from(value: &Vec<u8>) -> Result<Self, String> {
        let len = value.len();
        if len != 2 {
            return Err(format!("Invalid data size :{}", len).to_string());
        }
        Ok(Self {
            configuration: u16::from_le_bytes(value[..2].try_into().unwrap()),
        })
    }
}

impl Into<Vec<u8>> for ServerCharacteristicConfiguration {
    /// Create [`Vec<u8>`] from [`ServerCharacteristicConfiguration`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::descriptors::server_characteristic_configuration::{
    ///     ServerCharacteristicConfiguration, BROADCAST,
    /// };
    ///
    /// let configuration = BROADCAST.to_le_bytes().to_vec();
    /// let result = ServerCharacteristicConfiguration::new(BROADCAST);
    /// let into_data: Vec<u8> = result.into();
    /// assert_eq!(configuration, into_data);
    /// ```
    fn into(self) -> Vec<u8> {
        u16::to_le_bytes(self.configuration).to_vec()
    }
}

impl Uuid16bit for ServerCharacteristicConfiguration {
    /// return `0x2903`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::Uuid16bit;
    /// use ble_data_struct::descriptors::server_characteristic_configuration::ServerCharacteristicConfiguration;
    ///
    /// assert_eq!(0x2903, ServerCharacteristicConfiguration::uuid_16bit());
    /// ```
    fn uuid_16bit() -> u16 {
        0x2903
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        descriptors::server_characteristic_configuration::{
            ServerCharacteristicConfiguration, BROADCAST,
        },
        Uuid16bit,
    };

    #[test]
    fn test_new() {
        let result = ServerCharacteristicConfiguration::new(BROADCAST);
        assert_eq!(BROADCAST, result.configuration);
    }

    #[test]
    fn test_is_broadcast() {
        let result = ServerCharacteristicConfiguration::new(BROADCAST);
        assert!(result.is_broadcast());
    }

    #[test]
    fn test_try_from() {
        let configuration = BROADCAST.to_le_bytes().to_vec();
        let result = ServerCharacteristicConfiguration::try_from(&configuration);
        assert!(result.is_ok());
        assert_eq!(BROADCAST, result.unwrap().configuration);

        let configuration = Vec::new();
        let result = ServerCharacteristicConfiguration::try_from(&configuration);
        assert!(!result.is_ok());
    }

    #[test]
    fn test_into() {
        let configuration = BROADCAST.to_le_bytes().to_vec();
        let result = ServerCharacteristicConfiguration::new(BROADCAST);
        let into_data: Vec<u8> = result.into();
        assert_eq!(configuration, into_data);
    }

    #[test]
    fn test_uuid_16bit() {
        assert_eq!(0x2903, ServerCharacteristicConfiguration::uuid_16bit());
    }
}

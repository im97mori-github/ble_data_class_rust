//! Client Characteristic Configuration (Attribute Type: 0x2902) module.

use crate::Uuid16bit;

/// Client Characteristic Configuration.
#[derive(Debug, PartialEq, Clone)]
pub struct ClientCharacteristicConfiguration {
    /// Characteristic Configuration Bits
    pub configuration: u16,
}

impl ClientCharacteristicConfiguration {
    /// Create [`ClientCharacteristicConfiguration`] from `Characteristic Configuration Bit`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::descriptors::client_characteristic_configuration::{
    ///     ClientCharacteristicConfiguration, INDICATION, NOTIFICATION,
    /// };
    ///
    /// let result = ClientCharacteristicConfiguration::new(NOTIFICATION);
    /// assert_eq!(NOTIFICATION, result.configuration);
    /// ```
    pub fn new(configuration: u16) -> Self {
        Self { configuration }
    }

    /// check Notification configuration.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::descriptors::client_characteristic_configuration::{
    ///     ClientCharacteristicConfiguration, INDICATION, NOTIFICATION,
    /// };
    ///
    /// let result = ClientCharacteristicConfiguration::new(NOTIFICATION);
    /// assert!(result.is_notification());
    /// assert!(!result.is_indication());
    /// ```
    pub fn is_notification(&self) -> bool {
        self.configuration == NOTIFICATION
    }

    /// check Inidication configuration.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::descriptors::client_characteristic_configuration::{
    ///     ClientCharacteristicConfiguration, INDICATION, NOTIFICATION,
    /// };
    ///
    /// let result = ClientCharacteristicConfiguration::new(INDICATION);
    /// assert!(!result.is_notification());
    /// assert!(result.is_indication());
    /// ```
    pub fn is_indication(&self) -> bool {
        self.configuration == INDICATION
    }
}

/// Notification
pub const NOTIFICATION: u16 = 0b00000001;

/// Indication
pub const INDICATION: u16 = 0b00000010;

impl TryFrom<&Vec<u8>> for ClientCharacteristicConfiguration {
    type Error = String;
    /// Create [`ClientCharacteristicConfiguration`] from [`Vec<u8>`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::descriptors::client_characteristic_configuration::{
    ///     ClientCharacteristicConfiguration, INDICATION, NOTIFICATION,
    /// };
    ///
    /// let configuration = NOTIFICATION.to_le_bytes().to_vec();
    /// let result = ClientCharacteristicConfiguration::try_from(&configuration);
    /// assert!(result.is_ok());
    /// assert_eq!(NOTIFICATION, result.unwrap().configuration);
    ///
    /// let configuration = INDICATION.to_le_bytes().to_vec();
    /// let result = ClientCharacteristicConfiguration::try_from(&configuration);
    /// assert!(result.is_ok());
    /// assert_eq!(INDICATION, result.unwrap().configuration);
    ///
    /// let configuration = Vec::new();
    /// let result = ClientCharacteristicConfiguration::try_from(&configuration);
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

impl Into<Vec<u8>> for ClientCharacteristicConfiguration {
    /// Create [`Vec<u8>`] from [`ClientCharacteristicConfiguration`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::descriptors::client_characteristic_configuration::{
    ///     ClientCharacteristicConfiguration, INDICATION, NOTIFICATION,
    /// };
    ///
    /// let configuration = NOTIFICATION.to_le_bytes().to_vec();
    /// let result = ClientCharacteristicConfiguration::new(NOTIFICATION);
    /// let into_data: Vec<u8> = result.into();
    /// assert_eq!(configuration, into_data);
    ///
    /// let configuration = INDICATION.to_le_bytes().to_vec();
    /// let result = ClientCharacteristicConfiguration::new(INDICATION);
    /// let into_data: Vec<u8> = result.into();
    /// assert_eq!(configuration, into_data);
    /// ```
    fn into(self) -> Vec<u8> {
        u16::to_le_bytes(self.configuration).to_vec()
    }
}

impl Uuid16bit for ClientCharacteristicConfiguration {
    /// return `0x2902`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::Uuid16bit;
    /// use ble_data_struct::descriptors::client_characteristic_configuration::ClientCharacteristicConfiguration;
    ///
    /// assert_eq!(0x2902, ClientCharacteristicConfiguration::uuid_16bit());
    /// ```
    fn uuid_16bit() -> u16 {
        0x2902
    }
}

#[cfg(test)]
mod tests {
    use crate::{descriptors::client_characteristic_configuration::{
        ClientCharacteristicConfiguration, INDICATION, NOTIFICATION,
    }, Uuid16bit};

    #[test]
    fn test_new() {
        let result = ClientCharacteristicConfiguration::new(NOTIFICATION);
        assert_eq!(NOTIFICATION, result.configuration);
    }

    #[test]
    fn test_is_notification() {
        let result = ClientCharacteristicConfiguration::new(NOTIFICATION);
        assert!(result.is_notification());
        assert!(!result.is_indication());
    }

    #[test]
    fn test_is_indication() {
        let result = ClientCharacteristicConfiguration::new(INDICATION);
        assert!(!result.is_notification());
        assert!(result.is_indication());
    }

    #[test]
    fn test_try_from() {
        let configuration = NOTIFICATION.to_le_bytes().to_vec();
        let result = ClientCharacteristicConfiguration::try_from(&configuration);
        assert!(result.is_ok());
        assert_eq!(NOTIFICATION, result.unwrap().configuration);

        let configuration = INDICATION.to_le_bytes().to_vec();
        let result = ClientCharacteristicConfiguration::try_from(&configuration);
        assert!(result.is_ok());
        assert_eq!(INDICATION, result.unwrap().configuration);

        let configuration = Vec::new();
        let result = ClientCharacteristicConfiguration::try_from(&configuration);
        assert!(!result.is_ok());
    }

    #[test]
    fn test_into() {
        let configuration = NOTIFICATION.to_le_bytes().to_vec();
        let result = ClientCharacteristicConfiguration::new(NOTIFICATION);
        let into_data: Vec<u8> = result.into();
        assert_eq!(configuration, into_data);

        let configuration = INDICATION.to_le_bytes().to_vec();
        let result = ClientCharacteristicConfiguration::new(INDICATION);
        let into_data: Vec<u8> = result.into();
        assert_eq!(configuration, into_data);
    }

    #[test]
    fn test_uuid_16bit() {
        assert_eq!(0x2902, ClientCharacteristicConfiguration::uuid_16bit());
    }
}

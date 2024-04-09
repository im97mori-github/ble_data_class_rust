//! Characteristic Extended Properties (Attribute Type: 0x2900) module.

use crate::Uuid16bit;

/// Characteristic Extended Properties.
#[derive(Debug, PartialEq, Clone)]
pub struct CharacteristicExtendedProperties {
    /// Characteristic Extended Properties Bit Field
    pub properties: u16,
}

impl CharacteristicExtendedProperties {
    /// Create [`CharacteristicExtendedProperties`] from `Characteristic Extended Properties Bit Field`.
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
    pub fn new(properties: u16) -> Self {
        Self { properties }
    }

    /// check Reliable Write.
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
    pub fn is_reliable_write(&self) -> bool {
        self.properties == RELIABLE_WRITE
    }

    /// check Writable Auxiliaries.
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
    pub fn is_writable_auxiliaries(&self) -> bool {
        self.properties == WRITABLE_AUXILIARIES
    }
}

/// Reliable Write
pub const RELIABLE_WRITE: u16 = 0b00000001;

/// Writable Auxiliaries
pub const WRITABLE_AUXILIARIES: u16 = 0b00000010;

impl TryFrom<&Vec<u8>> for CharacteristicExtendedProperties {
    type Error = String;
    /// Create [`CharacteristicExtendedProperties`] from [`Vec<u8>`].
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
            properties: u16::from_le_bytes(value[..2].try_into().unwrap()),
        })
    }
}

impl Into<Vec<u8>> for CharacteristicExtendedProperties {
    /// Create [`Vec<u8>`] from [`CharacteristicExtendedProperties`].
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
        u16::to_le_bytes(self.properties).to_vec()
    }
}

impl Uuid16bit for CharacteristicExtendedProperties {
    /// return `0x2900`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::Uuid16bit;
    /// use ble_data_struct::descriptors::characteristic_extended_properties::CharacteristicExtendedProperties;
    ///
    /// assert_eq!(0x2900, CharacteristicExtendedProperties::uuid_16bit());
    /// ```
    fn uuid_16bit() -> u16 {
        0x2900
    }
}

#[cfg(test)]
mod tests {
    use crate::{descriptors::characteristic_extended_properties::{
        CharacteristicExtendedProperties, RELIABLE_WRITE, WRITABLE_AUXILIARIES,
    }, Uuid16bit};

    #[test]
    fn test_new() {
        let result = CharacteristicExtendedProperties::new(RELIABLE_WRITE);
        assert_eq!(RELIABLE_WRITE, result.properties);
    }

    #[test]
    fn test_is_reliable_write() {
        let result = CharacteristicExtendedProperties::new(RELIABLE_WRITE);
        assert!(result.is_reliable_write());
        assert!(!result.is_writable_auxiliaries());
    }

    #[test]
    fn test_is_writable_auxiliaries() {
        let result = CharacteristicExtendedProperties::new(WRITABLE_AUXILIARIES);
        assert!(!result.is_reliable_write());
        assert!(result.is_writable_auxiliaries());
    }

    #[test]
    fn test_try_from() {
        let properties = RELIABLE_WRITE.to_le_bytes().to_vec();
        let result = CharacteristicExtendedProperties::try_from(&properties);
        assert!(result.is_ok());
        assert_eq!(RELIABLE_WRITE, result.unwrap().properties);

        let properties = WRITABLE_AUXILIARIES.to_le_bytes().to_vec();
        let result = CharacteristicExtendedProperties::try_from(&properties);
        assert!(result.is_ok());
        assert_eq!(WRITABLE_AUXILIARIES, result.unwrap().properties);

        let properties = Vec::new();
        let result = CharacteristicExtendedProperties::try_from(&properties);
        assert!(!result.is_ok());
    }

    #[test]
    fn test_into() {
        let properties = RELIABLE_WRITE.to_le_bytes().to_vec();
        let result = CharacteristicExtendedProperties::new(RELIABLE_WRITE);
        let into_data: Vec<u8> = result.into();
        assert_eq!(properties, into_data);

        let properties = WRITABLE_AUXILIARIES.to_le_bytes().to_vec();
        let result = CharacteristicExtendedProperties::new(WRITABLE_AUXILIARIES);
        let into_data: Vec<u8> = result.into();
        assert_eq!(properties, into_data);
    }

    #[test]
    fn test_uuid_16bit() {
        assert_eq!(0x2900, CharacteristicExtendedProperties::uuid_16bit());
    }
}

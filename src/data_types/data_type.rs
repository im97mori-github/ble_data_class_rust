//! EIR/AD/SRD/ACAD/OOB data type module.

use crate::{AdvertisingInterval, Appearance, CompleteLocalName};

use super::advertising_interval_long::AdvertisingIntervalLong;

/// Trait for EIR/AD/SRD/ACAD/OOB data type.
pub trait DataType {
    /// Get EIR/AD/SRD/ACAD/OOB data type
    fn data_type() -> u8;
}

/// check `Complete Local Name` data type.
///
/// # Examples
///
/// ```
/// use ble_data_class::is_complete_local_name;
/// use ble_data_class::data_types::complete_local_name::CompleteLocalName;
/// use ble_data_class::data_types::data_type::DataType;
///
/// assert!(is_complete_local_name(0x09));
/// assert!(!is_complete_local_name(0x00));
/// ```
pub fn is_complete_local_name(data_type: u8) -> bool {
    CompleteLocalName::data_type() == data_type
}

/// check `Advertising Interval` data type.
///
/// # Examples
///
/// ```
/// use ble_data_class::is_advertising_interval;
/// use ble_data_class::data_types::advertising_interval::AdvertisingInterval;
/// use ble_data_class::data_types::data_type::DataType;
///
/// assert!(is_advertising_interval(0x1a));
/// assert!(!is_advertising_interval(0x00));
/// ```
pub fn is_advertising_interval(data_type: u8) -> bool {
    AdvertisingInterval::data_type() == data_type
}

/// check `Advertising Interval - long` data type.
///
/// # Examples
///
/// ```
/// use ble_data_class::is_advertising_interval_long;
/// use ble_data_class::data_types::advertising_interval_long::AdvertisingIntervalLong;
/// use ble_data_class::data_types::data_type::DataType;
///
/// assert!(is_advertising_interval_long(0x2f));
/// assert!(!is_advertising_interval_long(0x00));
/// ```
pub fn is_advertising_interval_long(data_type: u8) -> bool {
    AdvertisingIntervalLong::data_type() == data_type
}

/// check `Appearance` data type.
///
/// # Examples
///
/// ```
/// use ble_data_class::is_appearance;
/// use ble_data_class::data_types::appearance::Appearance;
/// use ble_data_class::data_types::data_type::DataType;
///
/// assert!(is_appearance(0x19));
/// assert!(!is_appearance(0x00));
/// ```
pub fn is_appearance(data_type: u8) -> bool {
    Appearance::data_type() == data_type
}

#[cfg(test)]
mod tests {
    use crate::{
        data_types::data_type::{is_advertising_interval, is_advertising_interval_long},
        is_appearance, is_complete_local_name,
    };

    #[test]
    fn test_is_complete_local_name() {
        assert!(is_complete_local_name(0x09));
        assert!(!is_complete_local_name(0x00));
    }

    #[test]
    fn test_is_advertising_interval() {
        assert!(is_advertising_interval(0x1a));
        assert!(!is_advertising_interval(0x00));
    }

    #[test]
    fn test_is_advertising_interval_long() {
        assert!(is_advertising_interval_long(0x2f));
        assert!(!is_advertising_interval_long(0x00));
    }

    #[test]
    fn test_is_appearance() {
        assert!(is_appearance(0x19));
        assert!(!is_appearance(0x00));
    }
}

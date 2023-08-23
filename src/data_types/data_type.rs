//! EIR/AD/SRD/ACAD/OOB data type module.

use crate::{
    AdvertisingInterval, AdvertisingIntervalLong, Appearance, BigInfo, ChannelMapUpdateIndication,
    CompleteListOf16BitServiceUuids, CompleteListOf32BitServiceUuids, CompleteLocalName,
};

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

/// check `Channel Map Update Indication` data type.
///
/// # Examples
///
/// ```
/// use ble_data_class::is_channel_map_update_indication;
/// use ble_data_class::data_types::channel_map_update_indication::ChannelMapUpdateIndication;
/// use ble_data_class::data_types::data_type::DataType;
///
/// assert!(is_channel_map_update_indication(0x28));
/// assert!(!is_channel_map_update_indication(0x00));
/// ```
pub fn is_channel_map_update_indication(data_type: u8) -> bool {
    ChannelMapUpdateIndication::data_type() == data_type
}

/// check `Channel Map Update Indication` data type.
///
/// # Examples
///
/// ```
/// use ble_data_class::is_big_info;
/// use ble_data_class::data_types::big_info::BigInfo;
/// use ble_data_class::data_types::data_type::DataType;
///
/// assert!(is_big_info(0x2c));
/// assert!(!is_big_info(0x00));
/// ```
pub fn is_big_info(data_type: u8) -> bool {
    BigInfo::data_type() == data_type
}

/// check `Complete List of 16-bit Service Class UUIDs.` data type.
///
/// # Examples
///
/// ```
/// use ble_data_class::is_complete_list_of_16bit_service_uuids;
/// use ble_data_class::data_types::complete_list_of_16bit_service_uuids::CompleteListOf16BitServiceUuids;
/// use ble_data_class::data_types::data_type::DataType;
///
/// assert!(is_complete_list_of_16bit_service_uuids(0x03));
/// assert!(!is_complete_list_of_16bit_service_uuids(0x00));
/// ```
pub fn is_complete_list_of_16bit_service_uuids(data_type: u8) -> bool {
    CompleteListOf16BitServiceUuids::data_type() == data_type
}

/// check `Complete List of 32-bit Service Class UUIDs.` data type.
///
/// # Examples
///
/// ```
/// use ble_data_class::is_complete_list_of_32bit_service_uuids;
/// use ble_data_class::data_types::complete_list_of_32bit_service_uuids::CompleteListOf32BitServiceUuids;
/// use ble_data_class::data_types::data_type::DataType;
///
/// assert!(is_complete_list_of_32bit_service_uuids(0x05));
/// assert!(!is_complete_list_of_32bit_service_uuids(0x00));
/// ```
pub fn is_complete_list_of_32bit_service_uuids(data_type: u8) -> bool {
    CompleteListOf32BitServiceUuids::data_type() == data_type
}

#[cfg(test)]
mod tests {
    use crate::{
        data_types::data_type::{
            is_advertising_interval, is_advertising_interval_long, is_channel_map_update_indication,
        },
        is_appearance, is_big_info, is_complete_list_of_16bit_service_uuids,
        is_complete_list_of_32bit_service_uuids, is_complete_local_name,
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

    #[test]
    fn test_is_channel_map_update_indication() {
        assert!(is_channel_map_update_indication(0x28));
        assert!(!is_channel_map_update_indication(0x00));
    }

    #[test]
    fn test_is_big_info() {
        assert!(is_big_info(0x2c));
        assert!(!is_big_info(0x00));
    }

    #[test]
    fn test_is_complete_list_of_16bit_service_uuids() {
        assert!(is_complete_list_of_16bit_service_uuids(0x03));
        assert!(!is_complete_list_of_16bit_service_uuids(0x00));
    }

    #[test]
    fn test_is_complete_list_of_32bit_service_uuids() {
        assert!(is_complete_list_of_32bit_service_uuids(0x05));
        assert!(!is_complete_list_of_32bit_service_uuids(0x00));
    }
}

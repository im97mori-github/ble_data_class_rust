//! BLE data struct.
pub mod data_types {
    //! EIR/AD/SRD/ACAD/OOB module.
    pub mod advertising_interval;
    pub mod advertising_interval_long;
    pub mod appearance;
    pub mod big_info;
    pub mod broadcast_code;
    pub mod channel_map_update_indication;
    pub mod class_of_device;
    pub mod complete_list_of_128bit_service_uuids;
    pub mod complete_list_of_16bit_service_uuids;
    pub mod complete_list_of_32bit_service_uuids;
    pub mod complete_local_name;
    pub mod data_type;
    pub mod data_type_parser;
    pub mod encrypted_data;
    pub mod flags;
    pub mod incomplete_list_of_128bit_service_uuids;
    pub mod incomplete_list_of_16bit_service_uuids;
    pub mod incomplete_list_of_32bit_service_uuids;
    pub mod le_bluetooth_device_address;
    pub mod le_role;
    pub mod le_secure_connections_confirmation_value;
    pub mod le_secure_connections_random_value;
    pub mod le_supported_features;
    pub mod list_of_128bit_service_solicitation_uuids;
    pub mod list_of_16bit_service_solicitation_uuids;
    pub mod list_of_32bit_service_solicitation_uuids;
    pub mod manufacturer_specific_data;
    pub mod periodic_advertising_response_timing_information;
    pub mod peripheral_connection_interval_range;
    pub mod public_target_address;
    pub mod random_target_address;
    pub mod secure_simple_pairing_hash_c192;
    pub mod secure_simple_pairing_hash_c256;
    pub mod secure_simple_pairing_randomizer_r192;
    pub mod secure_simple_pairing_randomizer_r256;
    pub mod security_manager_oob;
    pub mod security_manager_tk_value;
    pub mod service_data_128bit_uuid;
    pub mod service_data_16bit_uuid;
    pub mod service_data_32bit_uuid;
    pub mod shortened_local_name;
    pub mod tx_power_level;
    pub mod uniform_resource_identifier;
}

pub mod descriptors {
    //! descriptor module.
    pub mod characteristic_extended_properties;
    pub mod characteristic_user_description;
    pub mod client_characteristic_configuration;
    pub mod server_characteristic_configuration;
    pub mod characteristic_presentation_format;
    pub mod characteristic_aggregate_format;
}

/// for Windows
#[cfg(target_os = "windows")]
pub mod windows {
    pub mod data_types {
        pub mod windows_data_type_parser;
    }
    pub mod descriptors {
        // pub mod windows_characteristic_aggregate_format;
        // pub mod windows_characteristic_extended_properties;
        // pub mod windows_characteristic_presentation_format;
        pub mod windows_characteristic_user_description;
        pub mod windows_client_characteristic_configuration;
        pub mod windows_server_characteristic_configuration;
    }
    pub mod buffer;
}

use uuid::{uuid, Uuid};

/// BASE UUID.
///
/// 00000000-0000-1000-8000-00805F9B34FB
pub const BASE_UUID: Uuid = uuid!("00000000-0000-1000-8000-00805F9B34FB");

/// Create [`Uuid`] from [`u16`].
///
/// # Examples
///
/// ```
/// use ble_data_struct::uuid_from_u16;
/// use uuid::uuid;
///
/// assert_eq!(
///     uuid!("00001234-0000-1000-8000-00805F9B34FB"),
///     uuid_from_u16(0x1234)
/// );
/// ```
pub fn uuid_from_u16(value: u16) -> Uuid {
    let (d1, d2, d3, d4) = BASE_UUID.as_fields();
    Uuid::from_fields(d1 | value as u32, d2, d3, d4)
}

/// Create [`Uuid`] from [`u32`].
///
/// # Examples
///
/// ```
/// use ble_data_struct::uuid_from_u32;
/// use uuid::uuid;
///
/// assert_eq!(
///     uuid!("12345678-0000-1000-8000-00805F9B34FB"),
///     uuid_from_u32(0x12345678)
/// );
/// ```
pub fn uuid_from_u32(value: u32) -> Uuid {
    let (d1, d2, d3, d4) = BASE_UUID.as_fields();
    Uuid::from_fields(d1 | value, d2, d3, d4)
}

/// Trait for Assigned 16bit-UUID.
pub trait Uuid16bit {
    /// Assigned 16bit-UUID
    fn uuid_16bit() -> u16;
}

#[cfg(test)]
mod tests {
    use crate::{uuid_from_u16, uuid_from_u32};
    use uuid::uuid;

    #[test]
    fn test_uuid_from_u16() {
        assert_eq!(
            uuid!("00001234-0000-1000-8000-00805F9B34FB"),
            uuid_from_u16(0x1234)
        );
    }

    #[test]
    fn test_uuid_from_u32() {
        assert_eq!(
            uuid!("12345678-0000-1000-8000-00805F9B34FB"),
            uuid_from_u32(0x12345678)
        );
    }
}

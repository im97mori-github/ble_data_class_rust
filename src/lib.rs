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
    pub mod data_type;
    pub mod parser;
}

use uuid::{uuid, Uuid};

/// BASE UUID.
///
/// 00000000-0000-1000-8000-00805F9B34FB
pub const BASE_UUID: Uuid = uuid!("00000000-0000-1000-8000-00805F9B34FB");

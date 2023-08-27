//! BLE data class.
pub mod data_types {
    //! EIR/AD/SRD/ACAD/OOB module.
    pub mod advertising_interval;
    pub mod advertising_interval_long;
    pub mod appearance;
    pub mod big_info;
    pub mod channel_map_update_indication;
    pub mod complete_list_of_128bit_service_uuids;
    pub mod complete_list_of_16bit_service_uuids;
    pub mod complete_list_of_32bit_service_uuids;
    pub mod incomplete_list_of_128bit_service_uuids;
    pub mod incomplete_list_of_16bit_service_uuids;
    pub mod incomplete_list_of_32bit_service_uuids;
    pub mod complete_local_name;
    pub mod list_of_128bit_service_solicitation_uuids;
    pub mod list_of_16bit_service_solicitation_uuids;
    pub mod list_of_32bit_service_solicitation_uuids;
    pub mod data_type;
}

use uuid::{uuid, Uuid};

/// BASE UUID.
///
/// 00000000-0000-1000-8000-00805F9B34FB
pub const BASE_UUID: Uuid = uuid!("00000000-0000-1000-8000-00805F9B34FB");

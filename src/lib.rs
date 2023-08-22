//! BLE data class.
pub mod data_types {
    //! EIR/AD/SRD/ACAD/OOB module.
    pub mod data_type;
    pub mod complete_local_name;
    pub mod advertising_interval;
}

pub use data_types::data_type::DataType;
pub use data_types::data_type::is_advertising_interval;
pub use data_types::data_type::is_complete_local_name;
pub use data_types::complete_local_name::CompleteLocalName;
pub use data_types::advertising_interval::AdvertisingInterval;
//! BLE data class.
pub mod data_types {
    //! EIR/AD/SRD/ACAD/OOB module.
    pub mod complete_local_name;
    pub mod data_type;
}

use data_types::{complete_local_name::CompleteLocalName, data_type::DataType};

/// check `Complete Local Name` data type.
///
/// # Examples
///
/// ```
/// use ble_data_class::is_complete_local_name;
/// use ble_data_class::data_types::complete_local_name::CompleteLocalName;
/// use ble_data_class::data_types::data_type::DataType;
///
/// assert!(is_complete_local_name(CompleteLocalName::data_type()));
/// assert!(!is_complete_local_name(CompleteLocalName::data_type() + 1));
/// ```
pub fn is_complete_local_name(data_type: u8) -> bool {
    CompleteLocalName::data_type() == data_type
}

#[cfg(test)]
mod tests {
    use crate::{
        data_types::{complete_local_name::CompleteLocalName, data_type::DataType},
        is_complete_local_name,
    };

    #[test]
    fn test_is_complete_local_name() {
        assert!(is_complete_local_name(CompleteLocalName::data_type()));
        assert!(!is_complete_local_name(CompleteLocalName::data_type() + 1));
    }
}

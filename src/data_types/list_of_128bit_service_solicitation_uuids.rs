//! List of 128-bit Service Solicitation UUIDs (Data Type Value: 0x15) module.

use uuid::Uuid;

use crate::data_types::data_type::DataType;

/// List of 128-bit Service Solicitation UUIDs.
pub struct ListOf128BitServiceSolicitationUUIDs {
    /// data length
    pub length: u8,

    /// UUIDs
    pub uuids: Vec<Uuid>,
}

impl ListOf128BitServiceSolicitationUUIDs {
    /// Create [ListOf128BitServiceSolicitationUUIDs] from [`Vec<Uuid>`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_class::data_types::list_of_128bit_service_solicitation_uuids::ListOf128BitServiceSolicitationUUIDs;
    /// use uuid::{uuid, Uuid};
    ///
    /// let uuids: Vec<Uuid> = [
    ///     uuid!("00000001-0000-1000-8000-00805F9B34FB"),
    ///     uuid!("00000002-0000-1000-8000-00805F9B34FB"),
    /// ]
    /// .to_vec();
    /// let result = ListOf128BitServiceSolicitationUUIDs::new(&uuids);
    /// assert_eq!(uuids.len() as u8 * 16 + 1, result.length);
    /// assert_eq!(uuids, result.uuids);
    /// ```
    pub fn new(uuids: &Vec<Uuid>) -> Self {
        Self {
            length: (uuids.len() * 16 + 1) as u8,
            uuids: uuids.clone(),
        }
    }

    /// Create [ListOf128BitServiceSolicitationUUIDs] from `Vec<u8>` with offset.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_class::{BASE_UUID, data_types::{list_of_128bit_service_solicitation_uuids::ListOf128BitServiceSolicitationUUIDs, data_type::DataType}};
    /// use uuid::{uuid, Uuid};
    ///
    /// let uuid_bytes: Vec<u8> = [
    ///     0x01u8, 0x02u8, 0x03u8, 0x04u8, 0x05u8, 0x06u8, 0x07u8, 0x08u8,
    ///     0x09u8, 0x0au8, 0x0bu8, 0x0cu8, 0x0du8, 0x0eu8, 0x0fu8, 0x10u8,
    ///     0x11u8, 0x12u8, 0x13u8, 0x14u8, 0x15u8, 0x16u8, 0x17u8, 0x18u8,
    ///     0x19u8, 0x1au8, 0x1bu8, 0x1cu8, 0x1du8, 0x1eu8, 0x1fu8, 0x20u8,
    /// ]
    /// .to_vec();
    /// let uuids: Vec<Uuid> = uuid_bytes
    ///     .windows(16)
    ///     .step_by(16)
    ///     .map(|f| {
    ///         Uuid::from_bytes_le(f.try_into().unwrap())
    ///     })
    ///     .collect();
    /// let length = uuid_bytes.len() as u8 + 1;
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(ListOf128BitServiceSolicitationUUIDs::data_type());
    /// data.append(&mut uuid_bytes.clone());
    ///
    /// let result = ListOf128BitServiceSolicitationUUIDs::from_with_offset(&data, 0);
    /// assert_eq!(length, result.length);
    /// assert_eq!(uuids, result.uuids);
    ///
    /// data = Vec::new();
    /// data.push(0);
    /// data.push(length);
    /// data.push(ListOf128BitServiceSolicitationUUIDs::data_type());
    /// data.append(&mut uuid_bytes.clone());
    /// let result = ListOf128BitServiceSolicitationUUIDs::from_with_offset(&data, 1);
    /// assert_eq!(length, result.length);
    /// assert_eq!(uuids, result.uuids);
    /// ```
    pub fn from_with_offset(data: &Vec<u8>, offset: usize) -> Self {
        let data = data[offset..].to_vec();
        let length = data[0];
        Self {
            length,
            uuids: data[2..2 + length as usize - 1]
                .windows(16)
                .step_by(16)
                .map(|w| Uuid::from_bytes_le(w.try_into().unwrap()))
                .collect(),
        }
    }
}

impl From<&Vec<u8>> for ListOf128BitServiceSolicitationUUIDs {
    /// Create [ListOf128BitServiceSolicitationUUIDs] from `Vec<u8>`.
    ///
    /// [`ListOf128BitServiceSolicitationUUIDs::from_with_offset`]
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_class::{BASE_UUID, data_types::{list_of_128bit_service_solicitation_uuids::ListOf128BitServiceSolicitationUUIDs, data_type::DataType}};
    /// use uuid::{uuid, Uuid};
    ///
    /// let uuid_bytes: Vec<u8> = [
    ///     0x01u8, 0x02u8, 0x03u8, 0x04u8, 0x05u8, 0x06u8, 0x07u8, 0x08u8,
    ///     0x09u8, 0x0au8, 0x0bu8, 0x0cu8, 0x0du8, 0x0eu8, 0x0fu8, 0x10u8,
    ///     0x11u8, 0x12u8, 0x13u8, 0x14u8, 0x15u8, 0x16u8, 0x17u8, 0x18u8,
    ///     0x19u8, 0x1au8, 0x1bu8, 0x1cu8, 0x1du8, 0x1eu8, 0x1fu8, 0x20u8,
    /// ]
    /// .to_vec();
    /// let uuids: Vec<Uuid> = uuid_bytes
    ///     .windows(16)
    ///     .step_by(16)
    ///     .map(|f| {
    ///         Uuid::from_bytes_le(f.try_into().unwrap())
    ///     })
    ///     .collect();
    /// let length = uuid_bytes.len() as u8 + 1;
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(ListOf128BitServiceSolicitationUUIDs::data_type());
    /// data.append(&mut uuid_bytes.clone());
    ///
    /// let result = ListOf128BitServiceSolicitationUUIDs::from(&data);
    /// assert_eq!(length, result.length);
    /// assert_eq!(uuids, result.uuids);
    /// ```
    fn from(data: &Vec<u8>) -> Self {
        Self::from_with_offset(data, 0)
    }
}

impl Into<Vec<u8>> for ListOf128BitServiceSolicitationUUIDs {
    /// Create `Vec<u8>` from [ListOf128BitServiceSolicitationUUIDs].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_class::{BASE_UUID, data_types::{list_of_128bit_service_solicitation_uuids::ListOf128BitServiceSolicitationUUIDs, data_type::DataType}};
    /// use uuid::{uuid, Uuid};
    ///
    /// let uuid_bytes: Vec<u8> = [
    ///     0x01u8, 0x02u8, 0x03u8, 0x04u8, 0x05u8, 0x06u8, 0x07u8, 0x08u8,
    ///     0x09u8, 0x0au8, 0x0bu8, 0x0cu8, 0x0du8, 0x0eu8, 0x0fu8, 0x10u8,
    ///     0x11u8, 0x12u8, 0x13u8, 0x14u8, 0x15u8, 0x16u8, 0x17u8, 0x18u8,
    ///     0x19u8, 0x1au8, 0x1bu8, 0x1cu8, 0x1du8, 0x1eu8, 0x1fu8, 0x20u8,
    /// ]
    /// .to_vec();
    /// let uuids: Vec<Uuid> = uuid_bytes
    ///     .windows(16)
    ///     .step_by(16)
    ///     .map(|f| {
    ///         Uuid::from_bytes_le(f.try_into().unwrap())
    ///     })
    ///     .collect();
    /// let result1 = ListOf128BitServiceSolicitationUUIDs::new(&uuids);
    ///
    /// let length = uuid_bytes.len() as u8 + 1;
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(ListOf128BitServiceSolicitationUUIDs::data_type());
    /// data.append(&mut uuid_bytes.clone());
    ///
    /// let into_data: Vec<u8> = result1.into();
    /// assert_eq!(data, into_data);
    ///
    /// let result2 = ListOf128BitServiceSolicitationUUIDs::from(&data);
    /// let into_data: Vec<u8> = result2.into();
    /// assert_eq!(data, into_data);
    /// ```
    fn into(self) -> Vec<u8> {
        let mut data: Vec<u8> = Vec::new();
        data.push(self.length);
        data.push(Self::data_type());
        data.append(
            &mut self
                .uuids
                .clone()
                .iter()
                .flat_map(|f| f.to_bytes_le().to_vec())
                .collect(),
        );
        return data;
    }
}

impl DataType for ListOf128BitServiceSolicitationUUIDs {
    /// return `0x15`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_class::data_types::{list_of_128bit_service_solicitation_uuids::ListOf128BitServiceSolicitationUUIDs, data_type::DataType};
    ///
    /// assert_eq!(0x15, ListOf128BitServiceSolicitationUUIDs::data_type());
    /// ```
    fn data_type() -> u8 {
        0x15
    }
}

/// check `List of 128-bit Service Solicitation UUIDs.` data type.
///
/// # Examples
///
/// ```
/// use ble_data_class::data_types::list_of_128bit_service_solicitation_uuids::*;
/// use ble_data_class::data_types::data_type::DataType;
///
/// assert!(is_list_of_128bit_service_solicitation_uuids(0x15));
/// assert!(!is_list_of_128bit_service_solicitation_uuids(0x00));
/// ```
pub fn is_list_of_128bit_service_solicitation_uuids(data_type: u8) -> bool {
    ListOf128BitServiceSolicitationUUIDs::data_type() == data_type
}

#[cfg(test)]
mod tests {
    use uuid::{uuid, Uuid};

    use crate::data_types::{
        list_of_128bit_service_solicitation_uuids::{
            is_list_of_128bit_service_solicitation_uuids, ListOf128BitServiceSolicitationUUIDs,
        },
        data_type::DataType,
    };

    #[test]
    fn test_new() {
        let uuids: Vec<Uuid> = [
            uuid!("00000001-0000-1000-8000-00805F9B34FB"),
            uuid!("00000002-0000-1000-8000-00805F9B34FB"),
        ]
        .to_vec();
        let result = ListOf128BitServiceSolicitationUUIDs::new(&uuids);
        assert_eq!(uuids.len() as u8 * 16 + 1, result.length);
        assert_eq!(uuids, result.uuids);
    }

    #[test]
    fn test_from_with_offset() {
        let uuid_bytes: Vec<u8> = [
            0x01u8, 0x02u8, 0x03u8, 0x04u8, 0x05u8, 0x06u8, 0x07u8, 0x08u8, 0x09u8, 0x0au8, 0x0bu8,
            0x0cu8, 0x0du8, 0x0eu8, 0x0fu8, 0x10u8, 0x11u8, 0x12u8, 0x13u8, 0x14u8, 0x15u8, 0x16u8,
            0x17u8, 0x18u8, 0x19u8, 0x1au8, 0x1bu8, 0x1cu8, 0x1du8, 0x1eu8, 0x1fu8, 0x20u8,
        ]
        .to_vec();
        let uuids: Vec<Uuid> = uuid_bytes
            .windows(16)
            .step_by(16)
            .map(|f| Uuid::from_bytes_le(f.try_into().unwrap()))
            .collect();
        let length = uuid_bytes.len() as u8 + 1;
        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(ListOf128BitServiceSolicitationUUIDs::data_type());
        data.append(&mut uuid_bytes.clone());

        let result = ListOf128BitServiceSolicitationUUIDs::from_with_offset(&data, 0);
        assert_eq!(length, result.length);
        assert_eq!(uuids, result.uuids);

        data = Vec::new();
        data.push(0);
        data.push(length);
        data.push(ListOf128BitServiceSolicitationUUIDs::data_type());
        data.append(&mut uuid_bytes.clone());
        let result = ListOf128BitServiceSolicitationUUIDs::from_with_offset(&data, 1);
        assert_eq!(length, result.length);
        assert_eq!(uuids, result.uuids);
    }

    #[test]
    fn test_from() {
        let uuid_bytes: Vec<u8> = [
            0x01u8, 0x02u8, 0x03u8, 0x04u8, 0x05u8, 0x06u8, 0x07u8, 0x08u8, 0x09u8, 0x0au8, 0x0bu8,
            0x0cu8, 0x0du8, 0x0eu8, 0x0fu8, 0x10u8, 0x11u8, 0x12u8, 0x13u8, 0x14u8, 0x15u8, 0x16u8,
            0x17u8, 0x18u8, 0x19u8, 0x1au8, 0x1bu8, 0x1cu8, 0x1du8, 0x1eu8, 0x1fu8, 0x20u8,
        ]
        .to_vec();
        let uuids: Vec<Uuid> = uuid_bytes
            .windows(16)
            .step_by(16)
            .map(|f| Uuid::from_bytes_le(f.try_into().unwrap()))
            .collect();
        let length = uuid_bytes.len() as u8 + 1;
        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(ListOf128BitServiceSolicitationUUIDs::data_type());
        data.append(&mut uuid_bytes.clone());

        let result = ListOf128BitServiceSolicitationUUIDs::from(&data);
        assert_eq!(length, result.length);
        assert_eq!(uuids, result.uuids);
    }

    #[test]
    fn test_into() {
        let uuid_bytes: Vec<u8> = [
            0x01u8, 0x02u8, 0x03u8, 0x04u8, 0x05u8, 0x06u8, 0x07u8, 0x08u8, 0x09u8, 0x0au8, 0x0bu8,
            0x0cu8, 0x0du8, 0x0eu8, 0x0fu8, 0x10u8, 0x11u8, 0x12u8, 0x13u8, 0x14u8, 0x15u8, 0x16u8,
            0x17u8, 0x18u8, 0x19u8, 0x1au8, 0x1bu8, 0x1cu8, 0x1du8, 0x1eu8, 0x1fu8, 0x20u8,
        ]
        .to_vec();
        let uuids: Vec<Uuid> = uuid_bytes
            .windows(16)
            .step_by(16)
            .map(|f| Uuid::from_bytes_le(f.try_into().unwrap()))
            .collect();
        let result1 = ListOf128BitServiceSolicitationUUIDs::new(&uuids);

        let length = uuid_bytes.len() as u8 + 1;
        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(ListOf128BitServiceSolicitationUUIDs::data_type());
        data.append(&mut uuid_bytes.clone());

        let into_data: Vec<u8> = result1.into();
        assert_eq!(data, into_data);

        let result2 = ListOf128BitServiceSolicitationUUIDs::from(&data);
        let into_data: Vec<u8> = result2.into();
        assert_eq!(data, into_data);
    }

    #[test]
    fn test_data_type() {
        assert_eq!(0x15, ListOf128BitServiceSolicitationUUIDs::data_type());
    }

    #[test]
    fn test_is_list_of_128bit_service_solicitation_uuids() {
        assert!(is_list_of_128bit_service_solicitation_uuids(0x15));
        assert!(!is_list_of_128bit_service_solicitation_uuids(0x00));
    }
}

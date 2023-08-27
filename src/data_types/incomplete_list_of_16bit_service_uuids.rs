//! Incomplete List of 16-bit Service Class UUIDs (Data Type Value: 0x02) module.

use uuid::Uuid;

use crate::{data_types::data_type::DataType, BASE_UUID};

/// Incomplete List of 16-bit Service Class UUIDs.
pub struct IncompleteListOf16BitServiceUuids {
    /// data length
    pub length: u8,

    /// UUIDs
    pub uuids: Vec<Uuid>,
}

impl IncompleteListOf16BitServiceUuids {
    /// Create [IncompleteListOf16BitServiceUuids] from [`Vec<Uuid>`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_class::data_types::incomplete_list_of_16bit_service_uuids::IncompleteListOf16BitServiceUuids;
    /// use uuid::{uuid, Uuid};
    ///
    /// let uuids: Vec<Uuid> = [
    ///     uuid!("00000001-0000-1000-8000-00805F9B34FB"),
    ///     uuid!("00000002-0000-1000-8000-00805F9B34FB"),
    /// ]
    /// .to_vec();
    /// let result = IncompleteListOf16BitServiceUuids::new(&uuids);
    /// assert_eq!(uuids.len() as u8 * 2 + 1, result.length);
    /// assert_eq!(uuids, result.uuids);
    /// ```
    pub fn new(uuids: &Vec<Uuid>) -> Self {
        Self {
            length: (uuids.len() * 2 + 1) as u8,
            uuids: uuids.clone(),
        }
    }

    /// Create [IncompleteListOf16BitServiceUuids] from `Vec<u8>` with offset.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_class::{BASE_UUID, data_types::{incomplete_list_of_16bit_service_uuids::IncompleteListOf16BitServiceUuids, data_type::DataType}};
    /// use uuid::{uuid, Uuid};
    ///
    /// let uuid_bytes: Vec<u8> = [0x01u8, 0x02u8, 0x03u8, 0x04u8].to_vec();
    /// let uuids: Vec<Uuid> = uuid_bytes
    ///     .windows(2)
    ///     .step_by(2)
    ///     .map(|f| {
    ///         let (d1, d2, d3, d4) = BASE_UUID.as_fields();
    ///         Uuid::from_fields(d1 | ((f[0] as u32) << 0) | ((f[1] as u32) << 8), d2, d3, d4)
    ///     })
    ///     .collect();
    /// let length = uuid_bytes.len() as u8 + 1;
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(IncompleteListOf16BitServiceUuids::data_type());
    /// data.append(&mut uuid_bytes.clone());
    ///
    /// let result = IncompleteListOf16BitServiceUuids::from_with_offset(&data, 0);
    /// assert_eq!(length, result.length);
    /// assert_eq!(uuids, result.uuids);
    ///
    /// data = Vec::new();
    /// data.push(0);
    /// data.push(length);
    /// data.push(IncompleteListOf16BitServiceUuids::data_type());
    /// data.append(&mut uuid_bytes.clone());
    /// let result = IncompleteListOf16BitServiceUuids::from_with_offset(&data, 1);
    /// assert_eq!(length, result.length);
    /// assert_eq!(uuids, result.uuids);
    /// ```
    pub fn from_with_offset(data: &Vec<u8>, offset: usize) -> Self {
        let data = data[offset..].to_vec();
        let length = data[0];
        Self {
            length,
            uuids: data[2..2 + length as usize - 1]
                .windows(2)
                .step_by(2)
                .map(|w| {
                    let mut bytes = BASE_UUID.to_bytes_le();
                    bytes[0] = w[0];
                    bytes[1] = w[1];
                    Uuid::from_bytes_le(bytes)
                })
                .collect(),
        }
    }
}

impl From<&Vec<u8>> for IncompleteListOf16BitServiceUuids {
    /// Create [IncompleteListOf16BitServiceUuids] from `Vec<u8>`.
    ///
    /// [`IncompleteListOf16BitServiceUuids::from_with_offset`]
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_class::{BASE_UUID, data_types::{incomplete_list_of_16bit_service_uuids::IncompleteListOf16BitServiceUuids, data_type::DataType}};
    /// use uuid::{uuid, Uuid};
    ///
    /// let uuid_bytes: Vec<u8> = [0x01u8, 0x02u8, 0x03u8, 0x04u8].to_vec();
    /// let uuids: Vec<Uuid> = uuid_bytes
    ///     .windows(2)
    ///     .step_by(2)
    ///     .map(|f| {
    ///         let (d1, d2, d3, d4) = BASE_UUID.as_fields();
    ///         Uuid::from_fields(d1 | ((f[0] as u32) << 0) | ((f[1] as u32) << 8), d2, d3, d4)
    ///     })
    ///     .collect();
    /// let length = uuid_bytes.len() as u8 + 1;
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(IncompleteListOf16BitServiceUuids::data_type());
    /// data.append(&mut uuid_bytes.clone());
    ///
    /// let result = IncompleteListOf16BitServiceUuids::from_with_offset(&data, 0);
    /// assert_eq!(length, result.length);
    /// assert_eq!(uuids, result.uuids);
    ///
    /// data = Vec::new();
    /// data.push(0);
    /// data.push(length);
    /// data.push(IncompleteListOf16BitServiceUuids::data_type());
    /// data.append(&mut uuid_bytes.clone());
    /// let result = IncompleteListOf16BitServiceUuids::from_with_offset(&data, 1);
    /// assert_eq!(length, result.length);
    /// assert_eq!(uuids, result.uuids);
    /// ```
    fn from(data: &Vec<u8>) -> Self {
        Self::from_with_offset(data, 0)
    }
}

impl Into<Vec<u8>> for IncompleteListOf16BitServiceUuids {
    /// Create `Vec<u8>` from [IncompleteListOf16BitServiceUuids].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_class::{BASE_UUID, data_types::{incomplete_list_of_16bit_service_uuids::IncompleteListOf16BitServiceUuids, data_type::DataType}};
    /// use uuid::{uuid, Uuid};
    ///
    /// let uuid_bytes: Vec<u8> = [0x01u8, 0x02u8, 0x03u8, 0x04u8].to_vec();
    /// let uuids: Vec<Uuid> = uuid_bytes
    ///     .windows(2)
    ///     .step_by(2)
    ///     .map(|f| {
    ///         let (d1, d2, d3, d4) = BASE_UUID.as_fields();
    ///         Uuid::from_fields(d1 | ((f[0] as u32) << 0) | ((f[1] as u32) << 8), d2, d3, d4)
    ///     })
    ///     .collect();
    /// let result1 = IncompleteListOf16BitServiceUuids::new(&uuids);
    ///
    /// let length = uuid_bytes.len() as u8 + 1;
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(IncompleteListOf16BitServiceUuids::data_type());
    /// data.append(&mut uuid_bytes.clone());
    ///
    /// let into_data: Vec<u8> = result1.into();
    /// assert_eq!(data, into_data);
    ///
    /// let result2 = IncompleteListOf16BitServiceUuids::from(&data);
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
                .flat_map(|f| f.to_bytes_le()[..2].to_vec())
                .collect(),
        );
        return data;
    }
}

impl DataType for IncompleteListOf16BitServiceUuids {
    /// return `0x02`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_class::data_types::{incomplete_list_of_16bit_service_uuids::IncompleteListOf16BitServiceUuids, data_type::DataType};
    ///
    /// assert_eq!(0x02, IncompleteListOf16BitServiceUuids::data_type());
    /// ```
    fn data_type() -> u8 {
        0x02
    }
}

/// check `Incomplete List of 16-bit Service Class UUIDs` data type.
///
/// # Examples
///
/// ```
/// use ble_data_class::data_types::incomplete_list_of_16bit_service_uuids::*;
/// use ble_data_class::data_types::data_type::DataType;
///
/// assert!(is_incomplete_list_of_16bit_service_uuids(0x02));
/// assert!(!is_incomplete_list_of_16bit_service_uuids(0x00));
/// ```
pub fn is_incomplete_list_of_16bit_service_uuids(data_type: u8) -> bool {
    IncompleteListOf16BitServiceUuids::data_type() == data_type
}

#[cfg(test)]
mod tests {
    use uuid::{uuid, Uuid};

    use crate::{
        data_types::{
            incomplete_list_of_16bit_service_uuids::{IncompleteListOf16BitServiceUuids, is_incomplete_list_of_16bit_service_uuids},
            data_type::DataType,
        },
        BASE_UUID,
    };

    #[test]
    fn test_new() {
        let uuids: Vec<Uuid> = [
            uuid!("00000001-0000-1000-8000-00805F9B34FB"),
            uuid!("00000002-0000-1000-8000-00805F9B34FB"),
        ]
        .to_vec();
        let result = IncompleteListOf16BitServiceUuids::new(&uuids);
        assert_eq!(uuids.len() as u8 * 2 + 1, result.length);
        assert_eq!(uuids, result.uuids);
    }

    #[test]
    fn test_from_with_offset() {
        let uuid_bytes: Vec<u8> = [0x01u8, 0x02u8, 0x03u8, 0x04u8].to_vec();
        let uuids: Vec<Uuid> = uuid_bytes
            .windows(2)
            .step_by(2)
            .map(|f| {
                let (d1, d2, d3, d4) = BASE_UUID.as_fields();
                Uuid::from_fields(d1 | ((f[0] as u32) << 0) | ((f[1] as u32) << 8), d2, d3, d4)
            })
            .collect();
        let length = uuid_bytes.len() as u8 + 1;
        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(IncompleteListOf16BitServiceUuids::data_type());
        data.append(&mut uuid_bytes.clone());

        let result = IncompleteListOf16BitServiceUuids::from_with_offset(&data, 0);
        assert_eq!(length, result.length);
        assert_eq!(uuids, result.uuids);

        data = Vec::new();
        data.push(0);
        data.push(length);
        data.push(IncompleteListOf16BitServiceUuids::data_type());
        data.append(&mut uuid_bytes.clone());
        let result = IncompleteListOf16BitServiceUuids::from_with_offset(&data, 1);
        assert_eq!(length, result.length);
        assert_eq!(uuids, result.uuids);
    }

    #[test]
    fn test_from() {
        let uuid_bytes: Vec<u8> = [0x01u8, 0x02u8, 0x03u8, 0x04u8].to_vec();
        let uuids: Vec<Uuid> = uuid_bytes
            .windows(2)
            .step_by(2)
            .map(|f| {
                let (d1, d2, d3, d4) = BASE_UUID.as_fields();
                Uuid::from_fields(d1 | ((f[0] as u32) << 0) | ((f[1] as u32) << 8), d2, d3, d4)
            })
            .collect();
        let length = uuid_bytes.len() as u8 + 1;
        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(IncompleteListOf16BitServiceUuids::data_type());
        data.append(&mut uuid_bytes.clone());

        let result = IncompleteListOf16BitServiceUuids::from(&data);
        assert_eq!(length, result.length);
        assert_eq!(uuids, result.uuids);
    }

    #[test]
    fn test_into() {
        let uuid_bytes: Vec<u8> = [0x01u8, 0x02u8, 0x03u8, 0x04u8].to_vec();
        let uuids: Vec<Uuid> = uuid_bytes
            .windows(2)
            .step_by(2)
            .map(|f| {
                let (d1, d2, d3, d4) = BASE_UUID.as_fields();
                Uuid::from_fields(d1 | ((f[0] as u32) << 0) | ((f[1] as u32) << 8), d2, d3, d4)
            })
            .collect();
        let result1 = IncompleteListOf16BitServiceUuids::new(&uuids);

        let length = uuid_bytes.len() as u8 + 1;
        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(IncompleteListOf16BitServiceUuids::data_type());
        data.append(&mut uuid_bytes.clone());

        let into_data: Vec<u8> = result1.into();
        assert_eq!(data, into_data);

        let result2 = IncompleteListOf16BitServiceUuids::from(&data);
        let into_data: Vec<u8> = result2.into();
        assert_eq!(data, into_data);
    }

    #[test]
    fn test_data_type() {
        assert_eq!(0x02, IncompleteListOf16BitServiceUuids::data_type());
    }

    #[test]
    fn test_is_incomplete_list_of_16bit_service_uuids() {
        assert!(is_incomplete_list_of_16bit_service_uuids(0x02));
        assert!(!is_incomplete_list_of_16bit_service_uuids(0x00));
    }
}

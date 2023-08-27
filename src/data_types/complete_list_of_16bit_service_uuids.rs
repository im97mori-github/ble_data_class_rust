//! Complete List of 16-bit Service Class UUIDs (Data Type Value: 0x03) module.

use uuid::Uuid;

use crate::{data_types::data_type::DataType, BASE_UUID};

/// Complete List of 16-bit Service Class UUIDs.
pub struct CompleteListOf16BitServiceUuids {
    /// data length
    pub length: u8,

    /// UUIDs
    pub uuids: Vec<Uuid>,
}

impl CompleteListOf16BitServiceUuids {
    /// Create [CompleteListOf16BitServiceUuids] from [`Vec<Uuid>`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_class::data_types::complete_list_of_16bit_service_uuids::CompleteListOf16BitServiceUuids;
    /// use uuid::{uuid, Uuid};
    ///
    /// let uuids: Vec<Uuid> = [
    ///     uuid!("00000001-0000-1000-8000-00805F9B34FB"),
    ///     uuid!("00000002-0000-1000-8000-00805F9B34FB"),
    /// ]
    /// .to_vec();
    /// let result = CompleteListOf16BitServiceUuids::new(&uuids);
    /// assert_eq!(uuids.len() as u8 * 2 + 1, result.length);
    /// assert_eq!(uuids, result.uuids);
    /// ```
    pub fn new(uuids: &Vec<Uuid>) -> Self {
        Self {
            length: (uuids.len() * 2 + 1) as u8,
            uuids: uuids.clone(),
        }
    }

    /// Create [CompleteListOf16BitServiceUuids] from `Vec<u8>` with offset.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_class::{BASE_UUID, data_types::{complete_list_of_16bit_service_uuids::CompleteListOf16BitServiceUuids, data_type::DataType}};
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
    /// data.push(CompleteListOf16BitServiceUuids::data_type());
    /// data.append(&mut uuid_bytes.clone());
    ///
    /// let result = CompleteListOf16BitServiceUuids::from_with_offset(&data, 0);
    /// assert_eq!(length, result.length);
    /// assert_eq!(uuids, result.uuids);
    ///
    /// data = Vec::new();
    /// data.push(0);
    /// data.push(length);
    /// data.push(CompleteListOf16BitServiceUuids::data_type());
    /// data.append(&mut uuid_bytes.clone());
    /// let result = CompleteListOf16BitServiceUuids::from_with_offset(&data, 1);
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

impl From<&Vec<u8>> for CompleteListOf16BitServiceUuids {
    /// Create [CompleteListOf16BitServiceUuids] from `Vec<u8>`.
    ///
    /// [`CompleteListOf16BitServiceUuids::from_with_offset`]
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_class::{BASE_UUID, data_types::{complete_list_of_16bit_service_uuids::CompleteListOf16BitServiceUuids, data_type::DataType}};
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
    /// data.push(CompleteListOf16BitServiceUuids::data_type());
    /// data.append(&mut uuid_bytes.clone());
    ///
    /// let result = CompleteListOf16BitServiceUuids::from_with_offset(&data, 0);
    /// assert_eq!(length, result.length);
    /// assert_eq!(uuids, result.uuids);
    ///
    /// data = Vec::new();
    /// data.push(0);
    /// data.push(length);
    /// data.push(CompleteListOf16BitServiceUuids::data_type());
    /// data.append(&mut uuid_bytes.clone());
    /// let result = CompleteListOf16BitServiceUuids::from_with_offset(&data, 1);
    /// assert_eq!(length, result.length);
    /// assert_eq!(uuids, result.uuids);
    /// ```
    fn from(data: &Vec<u8>) -> Self {
        Self::from_with_offset(data, 0)
    }
}

impl Into<Vec<u8>> for CompleteListOf16BitServiceUuids {
    /// Create `Vec<u8>` from [CompleteListOf16BitServiceUuids].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_class::{BASE_UUID, data_types::{complete_list_of_16bit_service_uuids::CompleteListOf16BitServiceUuids, data_type::DataType}};
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
    /// let result1 = CompleteListOf16BitServiceUuids::new(&uuids);
    ///
    /// let length = uuid_bytes.len() as u8 + 1;
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(CompleteListOf16BitServiceUuids::data_type());
    /// data.append(&mut uuid_bytes.clone());
    ///
    /// let into_data: Vec<u8> = result1.into();
    /// assert_eq!(data, into_data);
    ///
    /// let result2 = CompleteListOf16BitServiceUuids::from(&data);
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

impl DataType for CompleteListOf16BitServiceUuids {
    /// return `0x03`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_class::data_types::{complete_list_of_16bit_service_uuids::CompleteListOf16BitServiceUuids, data_type::DataType};
    ///
    /// assert_eq!(0x03, CompleteListOf16BitServiceUuids::data_type());
    /// ```
    fn data_type() -> u8 {
        0x03
    }
}

/// check `Complete List of 16-bit Service Class UUIDs.` data type.
///
/// # Examples
///
/// ```
/// use ble_data_class::data_types::complete_list_of_16bit_service_uuids::*;
/// use ble_data_class::data_types::data_type::DataType;
///
/// assert!(is_complete_list_of_16bit_service_uuids(0x03));
/// assert!(!is_complete_list_of_16bit_service_uuids(0x00));
/// ```
pub fn is_complete_list_of_16bit_service_uuids(data_type: u8) -> bool {
    CompleteListOf16BitServiceUuids::data_type() == data_type
}

#[cfg(test)]
mod tests {
    use uuid::{uuid, Uuid};

    use crate::{
        data_types::{
            complete_list_of_16bit_service_uuids::{CompleteListOf16BitServiceUuids, is_complete_list_of_16bit_service_uuids},
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
        let result = CompleteListOf16BitServiceUuids::new(&uuids);
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
        data.push(CompleteListOf16BitServiceUuids::data_type());
        data.append(&mut uuid_bytes.clone());

        let result = CompleteListOf16BitServiceUuids::from_with_offset(&data, 0);
        assert_eq!(length, result.length);
        assert_eq!(uuids, result.uuids);

        data = Vec::new();
        data.push(0);
        data.push(length);
        data.push(CompleteListOf16BitServiceUuids::data_type());
        data.append(&mut uuid_bytes.clone());
        let result = CompleteListOf16BitServiceUuids::from_with_offset(&data, 1);
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
        data.push(CompleteListOf16BitServiceUuids::data_type());
        data.append(&mut uuid_bytes.clone());

        let result = CompleteListOf16BitServiceUuids::from(&data);
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
        let result1 = CompleteListOf16BitServiceUuids::new(&uuids);

        let length = uuid_bytes.len() as u8 + 1;
        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(CompleteListOf16BitServiceUuids::data_type());
        data.append(&mut uuid_bytes.clone());

        let into_data: Vec<u8> = result1.into();
        assert_eq!(data, into_data);

        let result2 = CompleteListOf16BitServiceUuids::from(&data);
        let into_data: Vec<u8> = result2.into();
        assert_eq!(data, into_data);
    }

    #[test]
    fn test_data_type() {
        assert_eq!(0x03, CompleteListOf16BitServiceUuids::data_type());
    }

    #[test]
    fn test_is_complete_list_of_16bit_service_uuids() {
        assert!(is_complete_list_of_16bit_service_uuids(0x03));
        assert!(!is_complete_list_of_16bit_service_uuids(0x00));
    }
}
